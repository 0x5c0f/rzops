use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::user::User;
use rzops_domain::ports::user_repository::UserRepository;
use rzops_domain::ports::TokenService;

use crate::dto::auth_dto::*;
use crate::dto::provider_dto::ErrorResponse;

/// Shared state for auth routes.
#[derive(Clone)]
pub struct AuthState {
    pub user_repo: Arc<dyn UserRepository>,
    pub token_service: Arc<dyn TokenService>,
}

/// POST /auth/login — Authenticate user and return JWT token.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
    ),
    tag = "Auth"
)]
pub async fn login(
    State(state): State<AuthState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = match state.user_repo.find_by_email(&body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "invalid email or password".to_string() })).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response()
        }
    };

    if !user.is_active {
        return (StatusCode::FORBIDDEN, Json(ErrorResponse { error: "account is disabled".to_string() })).into_response();
    }

    match bcrypt::verify(&body.password, &user.hashed_password) {
        Ok(true) => {}
        _ => {
            return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "invalid email or password".to_string() })).into_response()
        }
    }

    match state.token_service.create_token(user.id, &user.email, user.is_superuser) {
        Ok(token) => (StatusCode::OK, Json(AuthResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            user: UserInfo { id: user.id, email: user.email, full_name: user.full_name, is_superuser: user.is_superuser },
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create token: {}", e) })).into_response(),
    }
}

/// POST /auth/register — Register a new user.
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = AuthResponse),
        (status = 409, description = "Email already registered", body = ErrorResponse),
    ),
    tag = "Auth"
)]
pub async fn register(
    auth: crate::auth_extractor::AuthUser,
    State(state): State<AuthState>,
    Json(body): Json<RegisterRequest>,
) -> impl IntoResponse {
    if let Err(resp) = auth.require_superuser() {
        return resp;
    }
    match state.user_repo.find_by_email(&body.email).await {
        Ok(Some(_)) => {
            return (StatusCode::CONFLICT, Json(ErrorResponse { error: "email already registered".to_string() })).into_response()
        }
        Ok(None) => {}
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response()
        }
    }

    let hashed_password = match bcrypt::hash(&body.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to hash password: {}", e) })).into_response()
        }
    };

    let now = Utc::now();
    let user = User { id: Uuid::new_v4(), email: body.email, hashed_password, is_active: true, is_superuser: false, full_name: body.full_name, created_at: now };

    match state.user_repo.create(&user).await {
        Ok(created) => {
            match state.token_service.create_token(created.id, &created.email, created.is_superuser) {
                Ok(token) => (StatusCode::CREATED, Json(AuthResponse {
                    access_token: token,
                    token_type: "Bearer".to_string(),
                    user: UserInfo { id: created.id, email: created.email, full_name: created.full_name, is_superuser: created.is_superuser },
                })).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create token: {}", e) })).into_response(),
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create user: {}", e) })).into_response(),
    }
}

/// GET /auth/me — Get current authenticated user info.
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "Current user info", body = MeResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Auth"
)]
pub async fn me(
    auth_user: crate::auth_extractor::AuthUser,
) -> impl IntoResponse {
    (StatusCode::OK, Json(MeResponse {
        id: auth_user.user_id,
        email: auth_user.email,
        full_name: None,
        is_superuser: auth_user.is_superuser,
    }))
}
