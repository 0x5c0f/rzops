use async_trait::async_trait;
use uuid::Uuid;
use crate::models::monitor_target::MonitorTarget;

#[derive(Debug, Clone, Default)]
pub struct MonitorTargetFilter {
    pub status: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[async_trait]
pub trait MonitorTargetRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MonitorTarget>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: MonitorTargetFilter) -> Result<Vec<MonitorTarget>, crate::errors::RepositoryError>;
    async fn count(&self, filter: MonitorTargetFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, item: &MonitorTarget) -> Result<MonitorTarget, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, item: &MonitorTarget) -> Result<Option<MonitorTarget>, crate::errors::RepositoryError>;
    /// 解除监控目标与目标的关联（target_type/target_id 置空），目标记录本身保留
    async fn unbind(&self, id: Uuid) -> Result<Option<MonitorTarget>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
