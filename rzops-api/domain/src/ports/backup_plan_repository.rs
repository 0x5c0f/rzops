use async_trait::async_trait;
use uuid::Uuid;
use crate::models::backup_plan::BackupPlan;

#[derive(Debug, Clone, Default)]
pub struct BackupPlanFilter {
    pub status: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[async_trait]
pub trait BackupPlanRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BackupPlan>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: BackupPlanFilter) -> Result<Vec<BackupPlan>, crate::errors::RepositoryError>;
    async fn count(&self, filter: BackupPlanFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, item: &BackupPlan) -> Result<BackupPlan, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, item: &BackupPlan) -> Result<Option<BackupPlan>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
