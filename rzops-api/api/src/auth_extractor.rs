use std::sync::Arc;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use rzops_domain::ports::TokenService;
use rzops_domain::ports::user_repository::UserRepository;

use crate::dto::provider_dto::ErrorResponse;

/// Extracted authenticated user from JWT token.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub is_superuser: bool,
}

impl AuthUser {
    /// Reject the request unless the authenticated user is a superuser.
    pub fn require_superuser(&self) -> Result<(), Response> {
        if self.is_superuser {
            Ok(())
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "forbidden: administrator privileges required".to_string(),
                }),
            )
                .into_response())
        }
    }
}

/// State needed by the auth extractor.
#[derive(Clone)]
pub struct AuthExtractorState {
    pub token_service: Arc<dyn TokenService>,
    pub user_repo: Arc<dyn UserRepository>,
}

/// Implement `FromRequestParts` for `AuthUser` to extract from JWT.
/// Usage in handlers: `AuthUser` as the first extractor parameter.
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    // We need access to the AppState via axum's State
    // But since AppState varies, we'll use a different approach:
    // The auth state must be accessible via axum::Extension
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => &header[7..],
            _ => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "missing or invalid Authorization header".to_string(),
                    }),
                )
                    .into_response())
            }
        };

        // Get the token service from extensions (injected by middleware)
        let token_service = parts
            .extensions
            .get::<Arc<dyn TokenService>>()
            .cloned();

        let user_repo = parts
            .extensions
            .get::<Arc<dyn UserRepository>>()
            .cloned();

        let (token_service, user_repo) = match (token_service, user_repo) {
            (Some(t), Some(u)) => (t, u),
            _ => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "auth not configured".to_string(),
                    }),
                )
                    .into_response())
            }
        };

        // Validate the token
        let claims = match token_service.validate_token(token) {
            Ok(claims) => claims,
            Err(_) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "invalid or expired token".to_string(),
                    }),
                )
                    .into_response())
            }
        };

        // Parse user ID from claims
        let user_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "invalid token claims".to_string(),
                    }),
                )
                    .into_response())
            }
        };

        // Verify user still exists and is active
        match user_repo.find_by_id(user_id).await {
            Ok(Some(user)) if user.is_active => Ok(AuthUser {
                user_id,
                email: claims.email,
                is_superuser: claims.is_superuser,
            }),
            Ok(Some(_)) => Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "account is disabled".to_string(),
                }),
            )
                .into_response()),
            Ok(None) => Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "user not found".to_string(),
                }),
            )
                .into_response()),
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "database error".to_string(),
                }),
            )
                .into_response()),
        }
    }
}
