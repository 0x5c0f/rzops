use async_trait::async_trait;
use uuid::Uuid;
use crate::models::user::User;

/// User repository port.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, user: &User) -> Result<User, sqlx::Error>;
}
