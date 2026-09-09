use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::change_record::ChangeRecord;

#[derive(Debug, Clone, Default)]
pub struct ChangeRecordFilter {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub change_type: Option<String>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// ChangeRecord is system-generated — only read operations are exposed.
#[async_trait]
pub trait ChangeRecordRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChangeRecord>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: ChangeRecordFilter) -> Result<Vec<ChangeRecord>, crate::errors::RepositoryError>;
    async fn count(&self, filter: ChangeRecordFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, item: &ChangeRecord) -> Result<ChangeRecord, crate::errors::RepositoryError>;
}
