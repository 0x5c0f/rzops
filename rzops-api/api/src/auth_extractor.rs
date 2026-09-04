use std::collections::HashSet;
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
/// 由权限中间件解析并注入 request extensions；handler 直接以参数使用。
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub is_superuser: bool,
    /// 用户拥有的角色 code 集合（多角色合并）
    pub roles: Vec<String>,
    /// 用户拥有的权限点集合（跨角色去重）
    pub permissions: HashSet<String>,
}

impl AuthUser {
    /// 是否拥有指定权限点（超管恒为 true）
    pub fn has_perm(&self, perm: &str) -> bool {
        self.is_superuser || self.permissions.contains(perm)
    }

    /// 校验权限点，无权限返回 403。
    pub fn require_perm(&self, perm: &str) -> Result<(), Response> {
        if self.has_perm(perm) {
            Ok(())
        } else {
            Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: format!("forbidden: permission '{}' required", perm),
                }),
            )
                .into_response())
        }
    }

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
/// 优先读取权限中间件已注入的 extension；否则回退自行解析 token。
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 优先读取权限中间件注入的 AuthUser
        if let Some(user) = parts.extensions.get::<AuthUser>() {
            return Ok(user.clone());
        }

        // 回退：自行解析 token（与旧逻辑一致）
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

        match user_repo.find_by_id(user_id).await {
            Ok(Some(user)) if user.is_active => Ok(AuthUser {
                user_id,
                email: claims.email,
                is_superuser: claims.is_superuser,
                roles: Vec::new(),
                permissions: HashSet::new(),
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
