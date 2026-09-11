//! 权限校验（RBAC）中间件。
//!
//! 按请求 path + method 映射权限点（如 `server:read` / `server:create`），
//! 校验当前用户是否拥有该权限。超管（is_superuser）恒放行。
//! 白名单路径（login / register / swagger / openapi / me）无需权限。
//!
//! 中间件解析 JWT → 查询用户角色与权限集合 → 注入 `AuthUser` extension，
//! 下游 handler 的 `AuthUser` 提取器直接复用，避免二次查库。

use std::collections::HashSet;
use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{header, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use rzops_domain::ports::role_repository::RoleRepository;
use rzops_domain::ports::token_service::TokenService;
use rzops_domain::ports::user_repository::UserRepository;
use rzops_api::auth_extractor::AuthUser;
use rzops_api::dto::provider_dto::ErrorResponse;

/// 权限中间件所需状态。
#[derive(Clone)]
pub struct AuthzState {
    pub user_repo: Arc<dyn UserRepository>,
    pub role_repo: Arc<dyn RoleRepository>,
    pub token_service: Arc<dyn TokenService>,
}

impl AuthzState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self { user_repo, role_repo, token_service }
    }
}

/// 判断路径是否在白名单（无需权限）。
fn is_whitelisted(path: &str, method: &Method) -> bool {
    // Swagger / OpenAPI
    if path.starts_with("/swagger-ui") || path == "/api-docs/openapi.json" {
        return true;
    }
    // 登录 / 注册（匿名）
    if method == Method::POST && (path == "/api/v1/auth/login" || path == "/api/v1/auth/register") {
        return true;
    }
    false
}

/// 将 URL 路径中的资源段映射为权限资源名。
fn resource_from_segment(seg: &str) -> Option<&'static str> {
    match seg {
        "users" => Some("user"),
        "roles" => Some("role"),
        "recycle" => Some("recycle"),
        "providers" => Some("provider"),
        "data-centers" => Some("datacenter"),
        "servers" => Some("server"),
        "server-ips" => Some("server_ip"),
        "server-ports" => Some("server_port"),
        "server-port-templates" => Some("server_port_template"),
        "domains" => Some("domain"),
        "certificates" => Some("certificate"),
        "certificate-domains" => Some("certificate"),
        "database-instances" => Some("database_instance"),
        "ops-sites" => Some("ops_site"),
        "backup-plans" => Some("backup_plan"),
        "monitor-targets" => Some("monitor_target"),
        "contracts" => Some("contract"),
        "attachments" => Some("attachment"),
        "dicts" => Some("dict"),
        "site-relations" => Some("ops_site"),
        "audit-logs" => Some("audit"),
        "change-records" => Some("change"),
        _ => None,
    }
}

/// 根据请求 path + method 推断所需权限点。
fn required_permission(path: &str, method: &Method) -> Option<String> {
    let segments: Vec<&str> = path.trim_matches('/').split('/').collect();
    // 期望形如 ["api", "v1", "resource", ...]
    if segments.len() < 3 || segments[0] != "api" || segments[1] != "v1" {
        return None;
    }
    let resource = resource_from_segment(segments[2])?;

    // 系统级资源：用户 / 角色 / 回收站 / 审计 / 变更
    let system_resources = ["user", "role", "recycle", "audit", "change"];
    if system_resources.contains(&resource) {
        return Some(format!("system:{}", resource));
    }

    // 业务资源：按方法映射 read/create/update/delete
    let action = match *method {
        Method::GET | Method::HEAD => "read",
        Method::POST => "create",
        Method::PUT | Method::PATCH => "update",
        Method::DELETE => "delete",
        _ => return None,
    };
    Some(format!("{}:{}", resource, action))
}

/// 权限校验中间件。
pub async fn authz_middleware(
    State(state): State<AuthzState>,
    mut req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    // 白名单直接放行（不注入 AuthUser）
    if is_whitelisted(&path, &method) {
        return next.run(req).await;
    }

    // 解析 Authorization
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());
    let token = match auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(t) => t.to_string(),
        None => {
            return unauthorized("missing or invalid Authorization header");
        }
    };

    // 校验 JWT
    let claims = match state.token_service.validate_token(&token) {
        Ok(c) => c,
        Err(_) => return unauthorized("invalid or expired token"),
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return unauthorized("invalid token claims"),
    };

    // 查询用户（有效 & active）
    let user = match state.user_repo.find_by_id(user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => return unauthorized("user not found"),
        Err(_) => return internal_error("database error"),
    };
    if !user.is_active {
        return forbidden("account is disabled");
    }

    // 查询角色与权限
    let roles = match state.role_repo.get_user_role_codes(user_id).await {
        Ok(r) => r,
        Err(_) => return internal_error("failed to load roles"),
    };
    let permissions: HashSet<String> = match state.role_repo.get_user_permissions(user_id).await {
        Ok(p) => p.into_iter().collect(),
        Err(_) => return internal_error("failed to load permissions"),
    };

    let auth_user = AuthUser {
        user_id,
        email: claims.email,
        is_superuser: claims.is_superuser,
        roles,
        permissions,
    };

    // 校验权限点
    if let Some(perm) = required_permission(&path, &method) {
        if !auth_user.has_perm(&perm) {
            return forbidden(&format!("forbidden: permission '{}' required", perm));
        }
    }

    // 注入 AuthUser，供 handler 复用
    req.extensions_mut().insert(auth_user);
    next.run(req).await
}

fn unauthorized(msg: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse { error: msg.to_string() }),
    )
        .into_response()
}

fn forbidden(msg: &str) -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(ErrorResponse { error: msg.to_string() }),
    )
        .into_response()
}

fn internal_error(msg: &str) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse { error: msg.to_string() }),
    )
        .into_response()
}
