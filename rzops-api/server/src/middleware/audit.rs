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
        "providers" => "provider",
        "data-centers" => "datacenter",
        "servers" => "server",
        "server-ips" => "server_ip",
        "server-ports" => "server_port",
        "domains" => "domain",
        "certificates" => "certificate",
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
        _ => "unknown",
    }
}

/// 从路径提取资源类型与可选资源 ID。
fn parse_path(path: &str) -> (Option<&'static str>, Option<Uuid>) {
    let mut segs = path.split('/').filter(|s| !s.is_empty());
    // 跳过 api / v1
    let _ = segs.next();
    let _ = segs.next();
    let resource = segs.next().unwrap_or("");
    let id = segs.next().and_then(|s| Uuid::parse_str(s).ok());
    (Some(parse_resource_type(resource)), id)
}

/// 审计中间件：写请求时记录一条审计日志，随后继续处理。
pub async fn audit_log_middleware(
    State(state): State<AuditState>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    if matches!(method, Method::POST | Method::PUT | Method::PATCH | Method::DELETE) {
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
            Method::DELETE => "delete",
            _ => "other",
        };

        let (resource_type, resource_id) = parse_path(&path);
        let repo = state.audit_repo.clone();

        if let Some(rt) = resource_type {
            let log = AuditLog {
                id: Uuid::new_v4(),
                actor_id,
                action: action.to_string(),
                resource_type: rt.to_string(),
                resource_id,
                ip_address: ip,
                user_agent: ua,
                extra_data: serde_json::json!({}),
                created_at: chrono::Utc::now(),
            };
            tokio::spawn(async move {
                if let Err(e) = repo.create(&log).await {
                    tracing::error!("failed to record audit for {}: {}", rt, e);
                }
            });
        }
    }

    next.run(request).await
}
