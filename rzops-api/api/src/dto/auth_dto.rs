use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for login.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Request body for registration.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

/// Response body for successful authentication.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub user: UserInfo,
}

/// User info returned in auth response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub is_superuser: bool,
}

/// Response for GET /auth/me
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub is_superuser: bool,
}
