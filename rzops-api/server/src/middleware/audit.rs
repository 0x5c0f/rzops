//! 审计日志（AuditLog）自动写入中间件。
//!
//! 拦截所有写方法（POST / PUT / PATCH / DELETE），在请求进入 handler 前
//! 异步记录一条审计日志（操作者 / 动作 / 资源类型 / 资源ID / IP / User-Agent）。
//! 审计记录为 best-effort：写入失败只记日志，不阻塞主流程。

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::Method,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use rzops_domain::models::audit_log::AuditLog;
use rzops_domain::ports::audit_log_repository::AuditLogRepository;
use rzops_domain::ports::token_service::TokenService;

/// 审计中间件所需状态。
#[derive(Clone)]
pub struct AuditState {
    pub audit_repo: Arc<dyn AuditLogRepository>,
    pub token_service: Arc<dyn TokenService>,
}

impl AuditState {
    pub fn new(
        audit_repo: Arc<dyn AuditLogRepository>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self { audit_repo, token_service }
    }
}

/// 将 URL 路径中的资源段映射为审计用的资源类型名。
/// 路径形如 `/api/v1/{resource}[/{id}]`。
fn parse_resource_type(seg: &str) -> &'static str {
    match seg {
        "auth" => "auth",
        "users" => "user",
        "roles" => "role",
        "recycle" => "recycle",
        "providers" => "provider",
        "data-centers" => "datacenter",
        "servers" => "server",
        "server-ips" => "server_ip",
        "server-ports" => "server_port",
        "server-port-templates" => "server_port_template",
        "domains" => "domain",
        "certificates" => "certificate",
        "certificate-domains" => "certificate_domain",
        "database-instances" => "database_instance",
        "ops-sites" => "ops_site",
        "credentials" => "credential",
        "backup-plans" => "backup_plan",
        "monitor-targets" => "monitor_target",
        "contracts" => "contract",
        "attachments" => "attachment",
        "site-relations" => "site_relation",
        "audit-logs" => "audit_log",
        "change-records" => "change_record",
        "dicts" => "dict",
        _ => "unknown",
    }
}

/// 从路径提取资源类型与可选资源 ID。
/// `/recycle/{resource_type}/{id}` 这类多段路径也会被正确解析：
/// 返回的 resource_type 为回收站中被操作资源（如 domain），resource_id 为末段 UUID。
fn parse_path(path: &str) -> (Option<String>, Option<Uuid>) {
    let mut segs = path.split('/').filter(|s| !s.is_empty());
    // 跳过 api / v1
    let _ = segs.next();
    let _ = segs.next();
    let resource = segs.next().unwrap_or("");
    if resource == "recycle" {
        // /api/v1/recycle/{resource_type}/{id}
        let rt = segs.next().unwrap_or("unknown").to_string();
        let id = segs.next().and_then(|s| Uuid::parse_str(s).ok());
        return (Some(rt), id);
    }
    let id = segs.next().and_then(|s| Uuid::parse_str(s).ok());
    (Some(parse_resource_type(resource).to_string()), id)
}

/// 从响应 JSON body 中提取新建资源的 id（POST 新建场景）。
/// 形如 {"id":"<uuid>", ...}；非 JSON 或解析失败返回 None（保持 resource_id 为空）。
fn extract_resource_id(bytes: &[u8]) -> Option<Uuid> {
    let text = String::from_utf8_lossy(bytes);
    serde_json::from_str::<serde_json::Value>(text.trim())
        .ok()
        .and_then(|v| {
            v.get("id")
                .and_then(|i| i.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
        })
}

/// 审计中间件：写请求时记录一条审计日志，随后继续处理。
///
/// 同步写入（不 spawn）以保证：
/// 1. POST 新建可在响应 body 中提取到新建资源的 id 后再落库，资源列可回填名称；
/// 2. 避免异步写入与后续读取之间的竞态。
pub async fn audit_log_middleware(
    State(state): State<AuditState>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    if !matches!(method, Method::POST | Method::PUT | Method::PATCH | Method::DELETE) {
        return next.run(request).await;
    }

    // 提取操作者（Bearer token）
    let actor_id = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .and_then(|t| state.token_service.validate_token(t).ok())
        .and_then(|c| Uuid::parse_str(&c.sub).ok());

    let ip = request
        .headers()
        .get("x-forwarded-for")
        .or_else(|| request.headers().get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let ua = request
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let action = match method {
        Method::POST => "create",
        Method::PUT | Method::PATCH => "update",
        Method::DELETE => {
            // 回收站彻底删除（永久删除）与普通软删除区分
            if path.contains("/recycle/") {
                "purge"
            } else {
                "delete"
            }
        }
        _ => "other",
    };

    let (resource_type, path_id) = parse_path(&path);
    // POST 新建且路径无 id：需从响应 body 提取新建资源 id
    let needs_extract = method == Method::POST && path_id.is_none();

    let response = next.run(request).await;

    let mut resource_id = path_id;
    if needs_extract {
        let (parts, body) = response.into_parts();
        match axum::body::to_bytes(body, 1024 * 1024).await {
            Ok(bytes) => {
                resource_id = extract_resource_id(&bytes).or(path_id);
                return finalize(
                    state,
                    actor_id,
                    action,
                    resource_type,
                    resource_id,
                    ip,
                    ua,
                    Response::from_parts(parts, bytes.into()),
                ).await;
            }
            Err(_) => {
                // 读取 body 失败（极罕见）：移除 content-length 以匹配空 body，仍完成审计
                let mut parts = parts;
                parts.headers.remove(axum::http::header::CONTENT_LENGTH);
                return finalize(
                    state,
                    actor_id,
                    action,
                    resource_type,
                    resource_id,
                    ip,
                    ua,
                    Response::from_parts(parts, axum::body::Body::empty()),
                ).await;
            }
        }
    }

    finalize(state, actor_id, action, resource_type, resource_id, ip, ua, response).await
}

/// 写入审计日志并返回响应（best-effort：失败只记日志，不阻塞主流程）。
async fn finalize(
    state: AuditState,
    actor_id: Option<Uuid>,
    action: &str,
    resource_type: Option<String>,
    resource_id: Option<Uuid>,
    ip: Option<String>,
    ua: Option<String>,
    response: Response,
) -> Response {
    if let Some(rt) = resource_type {
        let log = AuditLog {
            id: Uuid::new_v4(),
            actor_id,
            action: action.to_string(),
            resource_type: rt.clone(),
            resource_id,
            ip_address: ip,
            user_agent: ua,
            extra_data: serde_json::json!({}),
            created_at: chrono::Utc::now(),
        };
        let repo = state.audit_repo.clone();
        // 同步写入：写操作低频，一次 INSERT 开销可接受，且保证与响应顺序一致
        if let Err(e) = repo.create(&log).await {
            tracing::error!("failed to record audit for {}: {}", rt, e);
        }
    }
    response
}
