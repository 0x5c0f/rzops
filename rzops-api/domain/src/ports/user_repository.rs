use async_trait::async_trait;
use uuid::Uuid;
use crate::models::user::User;

/// User repository port.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, user: &User) -> Result<User, sqlx::Error>;

    /// 分页查询用户列表（排除软删除，按创建时间倒序）。
    /// 返回 (data, total)。
    async fn list(
        &self,
        q: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<User>, i64), sqlx::Error>;

    /// 更新用户基础信息（email / full_name / is_active），不涉及密码。
    async fn update_profile(
        &self,
        id: Uuid,
        email: &str,
        full_name: Option<&str>,
        is_active: bool,
        is_superuser: bool,
    ) -> Result<Option<User>, sqlx::Error>;

    /// 更新用户密码（hashed_password）。
    async fn update_password(&self, id: Uuid, hashed_password: &str) -> Result<bool, sqlx::Error>;

    /// 软删除用户。
    async fn soft_delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
