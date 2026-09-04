use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::role::Role;
use rzops_domain::ports::role_repository::RoleRepository;

use crate::auth_extractor::AuthUser;
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::role_dto::*;

/// Shared state for role management routes.
#[derive(Clone)]
pub struct RoleMgmtState {
    pub role_repo: Arc<dyn RoleRepository>,
}

fn to_response(r: &Role) -> RoleResponse {
    RoleResponse {
        id: r.id,
        code: r.code.clone(),
        name: r.name.clone(),
        description: r.description.clone(),
        is_builtin: r.is_builtin,
        is_active: r.is_active,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }
}

/// GET /roles — list all roles.
#[utoipa::path(
    get,
    path = "/api/v1/roles",
    responses((status = 200, body = RoleListResponse)),
    tag = "Role"
)]
pub async fn list_roles(
    _auth: AuthUser,
    State(state): State<RoleMgmtState>,
) -> impl IntoResponse {
    match state.role_repo.find_all().await {
        Ok(roles) => {
            let data: Vec<RoleResponse> = roles.iter().map(to_response).collect();
            let count = data.len() as i64;
            (StatusCode::OK, Json(RoleListResponse { data, count })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// GET /roles/:id — get role with its permissions.
#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}",
    params(("id" = Uuid, Path, description = "Role ID")),
    responses((status = 200, body = RoleDetailResponse), (status = 404, body = ErrorResponse)),
    tag = "Role"
)]
pub async fn get_role(
    _auth: AuthUser,
    State(state): State<RoleMgmtState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.role_repo.find_by_id(id).await {
        Ok(Some(r)) => {
            let perms = state.role_repo.get_role_permissions(id).await.unwrap_or_default();
            (StatusCode::OK, Json(RoleDetailResponse { role: to_response(&r), permissions: perms })).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "role not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// POST /roles — create a custom role.
#[utoipa::path(
    post,
    path = "/api/v1/roles",
    request_body = CreateRoleRequest,
    responses((status = 201, body = RoleDetailResponse), (status = 409, body = ErrorResponse)),
    tag = "Role"
)]
pub async fn create_role(
    _auth: AuthUser,
    State(state): State<RoleMgmtState>,
    Json(body): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    let code = body.code.trim().to_string();
    if code.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "code is required".to_string() })).into_response();
    }
    if !code.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "code may only contain letters, digits, '_' and '-'".to_string() })).into_response();
    }
    if let Ok(Some(_)) = state.role_repo.find_by_code(&code).await {
        return (StatusCode::CONFLICT, Json(ErrorResponse { error: "role code already exists".to_string() })).into_response();
    }
    let now = Utc::now();
    let role = Role {
        id: Uuid::new_v4(),
        code,
        name: body.name.trim().to_string(),
        description: body.description.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        is_builtin: false,
        is_active: true,
        created_at: now,
        updated_at: now,
        deleted_at: None,
    };
    match state.role_repo.create(&role).await {
        Ok(created) => {
            if !body.permissions.is_empty() {
                let _ = state.role_repo.set_role_permissions(created.id, &body.permissions).await;
            }
            let perms = state.role_repo.get_role_permissions(created.id).await.unwrap_or_default();
            (StatusCode::CREATED, Json(RoleDetailResponse { role: to_response(&created), permissions: perms })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// PUT /roles/:id — update role (builtin roles: name/description only; custom: all).
#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    params(("id" = Uuid, Path)),
    request_body = UpdateRoleRequest,
    responses((status = 200, body = RoleDetailResponse), (status = 404, body = ErrorResponse)),
    tag = "Role"
)]
pub async fn update_role(
    _auth: AuthUser,
    State(state): State<RoleMgmtState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateRoleRequest>,
) -> impl IntoResponse {
    let existing = match state.role_repo.find_by_id(id).await {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "role not found".to_string() })).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    };
    let name = body.name.clone().unwrap_or_else(|| existing.name.clone());
    let description = match body.description {
        Some(d) => {
            let t = d.trim().to_string();
            if t.is_empty() { None } else { Some(t) }
        }
        None => existing.description.clone(),
    };
    let is_active = body.is_active.unwrap_or(existing.is_active);

    match state.role_repo.update(id, &name, description.as_deref(), is_active).await {
        Ok(Some(updated)) => {
            // 内置角色也允许自定义权限（便于灵活扩展）；仅限制删除
            if let Some(perms) = body.permissions {
                let _ = state.role_repo.set_role_permissions(id, &perms).await;
            }
            let perms = state.role_repo.get_role_permissions(id).await.unwrap_or_default();
            (StatusCode::OK, Json(RoleDetailResponse { role: to_response(&updated), permissions: perms })).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "role not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// DELETE /roles/:id — soft delete a custom role (builtin roles are protected).
#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}",
    params(("id" = Uuid, Path)),
    responses((status = 200), (status = 400, body = ErrorResponse)),
    tag = "Role"
)]
pub async fn delete_role(
    _auth: AuthUser,
    State(state): State<RoleMgmtState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.role_repo.find_by_id(id).await {
        Ok(Some(r)) if r.is_builtin => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "builtin role cannot be deleted".to_string() })).into_response(),
        Ok(Some(_)) => match state.role_repo.delete(id).await {
            Ok(true) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response(),
            Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "role not found".to_string() })).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "role not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}
