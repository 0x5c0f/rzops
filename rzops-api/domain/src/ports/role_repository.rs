use async_trait::async_trait;
use uuid::Uuid;

use crate::models::role::Role;

/// 角色仓储端口：角色 CRUD + 用户-角色关联 + 角色-权限关联。
#[async_trait]
pub trait RoleRepository: Send + Sync {
    // ── 角色 CRUD ──
    async fn create(&self, role: &Role) -> Result<Role, crate::errors::RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, crate::errors::RepositoryError>;
    async fn find_by_code(&self, code: &str) -> Result<Option<Role>, crate::errors::RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Role>, crate::errors::RepositoryError>;
    async fn update(
        &self,
        id: Uuid,
        name: &str,
        description: Option<&str>,
        is_active: bool,
    ) -> Result<Option<Role>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;

    // ── 角色-权限关联 ──
    async fn get_role_permissions(&self, role_id: Uuid) -> Result<Vec<String>, crate::errors::RepositoryError>;
    /// 全量替换角色的权限点（删旧插新）
    async fn set_role_permissions(&self, role_id: Uuid, perms: &[String]) -> Result<(), crate::errors::RepositoryError>;

    // ── 用户-角色关联（多对多） ──
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, crate::errors::RepositoryError>;
    /// 全量替换用户的角色（删旧插新）
    async fn set_user_roles(&self, user_id: Uuid, role_ids: &[Uuid]) -> Result<(), crate::errors::RepositoryError>;
    /// 查询用户拥有的全部角色 code
    async fn get_user_role_codes(&self, user_id: Uuid) -> Result<Vec<String>, crate::errors::RepositoryError>;
    /// 查询用户全部权限点（跨角色去重）
    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<String>, crate::errors::RepositoryError>;
}
