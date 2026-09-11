use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::user::User;
use rzops_domain::ports::role_repository::RoleRepository;
use rzops_domain::ports::user_repository::UserRepository;
use rzops_domain::ports::TokenService;

use crate::auth_extractor::AuthUser;
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::role_dto::RoleResponse;
use crate::dto::user_dto::*;

/// Shared state for user management routes.
#[derive(Clone)]
pub struct UserMgmtState {
    pub user_repo: Arc<dyn UserRepository>,
    pub role_repo: Arc<dyn RoleRepository>,
    pub token_service: Arc<dyn TokenService>,
}

fn role_to_response(r: &rzops_domain::models::role::Role) -> RoleResponse {
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

fn user_to_response(
    u: &User,
    roles: Vec<rzops_domain::models::role::Role>,
) -> UserResponse {
    UserResponse {
        id: u.id,
        email: u.email.clone(),
        is_active: u.is_active,
        is_superuser: u.is_superuser,
        full_name: u.full_name.clone(),
        roles: roles.iter().map(role_to_response).collect(),
        created_at: u.created_at,
    }
}

/// GET /users — list users with pagination and keyword search.
#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(("q" = Option<String>, Query), ("page" = Option<i64>, Query), ("per_page" = Option<i64>, Query)),
    responses((status = 200, body = UserListResponse)),
    tag = "User"
)]
pub async fn list_users(
    _auth: AuthUser,
    State(state): State<UserMgmtState>,
    Query(query): Query<ListUsersQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    match state.user_repo.list(query.q.as_deref(), page, per_page).await {
        Ok((users, total)) => {
            let mut data = Vec::with_capacity(users.len());
            for u in &users {
                let roles = state.role_repo.get_user_roles(u.id).await.unwrap_or_default();
                data.push(user_to_response(u, roles));
            }
            (StatusCode::OK, Json(UserListResponse { data, count: total })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// GET /users/:id — get a user with roles.
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(("id" = Uuid, Path, description = "User ID")),
    responses((status = 200, body = UserResponse), (status = 404, body = ErrorResponse)),
    tag = "User"
)]
pub async fn get_user(
    _auth: AuthUser,
    State(state): State<UserMgmtState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    match state.user_repo.find_by_id(id).await {
        Ok(Some(u)) => {
            let roles = state.role_repo.get_user_roles(id).await.unwrap_or_default();
            (StatusCode::OK, Json(user_to_response(&u, roles))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "user not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// POST /users — create a user (admin only).
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses((status = 201, body = UserResponse), (status = 409, body = ErrorResponse)),
    tag = "User"
)]
pub async fn create_user(
    _auth: AuthUser,
    State(state): State<UserMgmtState>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let email = body.email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "invalid email".to_string() })).into_response();
    }
    if body.password.len() < 6 {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "password must be at least 6 characters".to_string() })).into_response();
    }
    // 邮箱唯一性
    if let Ok(Some(_)) = state.user_repo.find_by_email(&email).await {
        return (StatusCode::CONFLICT, Json(ErrorResponse { error: "email already registered".to_string() })).into_response();
    }
    let hashed = match bcrypt::hash(&body.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "password hashing failed".to_string() })).into_response(),
    };
    let user = User {
        id: Uuid::new_v4(),
        email,
        hashed_password: hashed,
        is_active: body.is_active.unwrap_or(true),
        is_superuser: body.is_superuser.unwrap_or(false),
        full_name: body.full_name.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        created_at: Utc::now(),
        deleted_at: None,
    };
    match state.user_repo.create(&user).await {
        Ok(created) => {
            // 分配角色
            let role_ids = body.role_ids;
            if !role_ids.is_empty() {
                let _ = state.role_repo.set_user_roles(created.id, &role_ids).await;
            }
            let roles = state.role_repo.get_user_roles(created.id).await.unwrap_or_default();
            (StatusCode::CREATED, Json(user_to_response(&created, roles))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// PUT /users/:id — update user profile + roles.
#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    params(("id" = Uuid, Path)),
    request_body = UpdateUserRequest,
    responses((status = 200, body = UserResponse), (status = 404, body = ErrorResponse)),
    tag = "User"
)]
pub async fn update_user(
    auth: AuthUser,
    State(state): State<UserMgmtState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(body): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    // 超管不允许自己停用或降级（防止锁死）
    if id == auth.user_id && (body.is_active == Some(false) || body.is_superuser == Some(false)) {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "cannot disable or demote yourself".to_string() })).into_response();
    }
    // 目标用户必须存在
    let target = match state.user_repo.find_by_id(id).await {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "user not found".to_string() })).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    };
    let is_active = body.is_active.unwrap_or(target.is_active);
    let is_superuser = body.is_superuser.unwrap_or(target.is_superuser);
    let full_name = match body.full_name {
        Some(name) => {
            let t = name.trim().to_string();
            if t.is_empty() { None } else { Some(t) }
        }
        None => target.full_name.clone(),
    };
    match state.user_repo.update_profile(id, &target.email, full_name.as_deref(), is_active, is_superuser).await {
        Ok(Some(updated)) => {
            if let Some(role_ids) = body.role_ids {
                let _ = state.role_repo.set_user_roles(id, &role_ids).await;
            }
            let roles = state.role_repo.get_user_roles(id).await.unwrap_or_default();
            (StatusCode::OK, Json(user_to_response(&updated, roles))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "user not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// POST /users/:id/reset-password — reset a user's password (admin only).
#[utoipa::path(
    post,
    path = "/api/v1/users/{id}/reset-password",
    params(("id" = Uuid, Path)),
    request_body = ResetPasswordRequest,
    responses((status = 200), (status = 404, body = ErrorResponse)),
    tag = "User"
)]
pub async fn reset_password(
    _auth: AuthUser,
    State(state): State<UserMgmtState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(body): Json<ResetPasswordRequest>,
) -> impl IntoResponse {
    if body.new_password.len() < 6 {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "password must be at least 6 characters".to_string() })).into_response();
    }
    let hashed = match bcrypt::hash(&body.new_password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "password hashing failed".to_string() })).into_response(),
    };
    match state.user_repo.update_password(id, &hashed).await {
        Ok(true) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "user not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// DELETE /users/:id — soft delete a user (admin only).
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    params(("id" = Uuid, Path)),
    responses((status = 200), (status = 400, body = ErrorResponse)),
    tag = "User"
)]
pub async fn delete_user(
    auth: AuthUser,
    State(state): State<UserMgmtState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    if id == auth.user_id {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "cannot delete yourself".to_string() })).into_response();
    }
    match state.user_repo.soft_delete(id).await {
        Ok(true) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "user not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}
