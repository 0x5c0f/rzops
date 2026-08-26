use async_trait::async_trait;
use uuid::Uuid;

use crate::models::claims::Claims;

/// Port for JWT token creation and validation.
/// Implemented in infra, consumed by api.
#[async_trait]
pub trait TokenService: Send + Sync {
    /// Create a JWT token for the given user.
    fn create_token(&self, user_id: Uuid, email: &str, is_superuser: bool) -> Result<String, String>;

    /// Validate a JWT token and return the claims.
    fn validate_token(&self, token: &str) -> Result<Claims, String>;
}
