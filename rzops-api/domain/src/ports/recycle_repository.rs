use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::RepositoryError;
use crate::models::recycle::RecycleEntry;

/// 回收站仓储契约：软删除数据的查询、恢复与永久删除。
/// 所有 SQL 位于 `infra` 实现，领域层仅定义契约。
#[async_trait]
pub trait RecycleRepository: Send + Sync {
    /// 分页列出回收站条目；`resource_type` 为空表示全部类型。
    async fn list(
        &self,
        resource_type: Option<&str>,
        q: Option<&str>,
        per_page: i64,
        offset: i64,
    ) -> Result<(Vec<RecycleEntry>, i64), RepositoryError>;

    /// 恢复软删除条目（置空 deleted_at）；user/role 额外恢复启用状态。
    async fn restore(&self, resource_type: &str, id: Uuid) -> Result<bool, RepositoryError>;

    /// 读取软删除条目的展示名称（用于永久删除日志）。
    async fn purge_name(&self, resource_type: &str, id: Uuid) -> Result<Option<String>, RepositoryError>;

    /// 永久删除软删除条目。
    async fn purge(&self, resource_type: &str, id: Uuid) -> Result<bool, RepositoryError>;
}
