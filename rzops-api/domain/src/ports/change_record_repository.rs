use async_trait::async_trait;
use uuid::Uuid;
use crate::models::change_record::ChangeRecord;

#[derive(Debug, Clone, Default)]
pub struct ChangeRecordFilter {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// ChangeRecord is system-generated — only read operations are exposed.
#[async_trait]
pub trait ChangeRecordRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChangeRecord>, sqlx::Error>;
    async fn find_all(&self, filter: ChangeRecordFilter) -> Result<Vec<ChangeRecord>, sqlx::Error>;
    async fn count(&self, filter: ChangeRecordFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, item: &ChangeRecord) -> Result<ChangeRecord, sqlx::Error>;
}
