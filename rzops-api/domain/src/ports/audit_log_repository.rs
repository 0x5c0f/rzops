use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::audit_log::AuditLog;

#[derive(Debug, Clone, Default)]
pub struct AuditLogFilter {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub action: Option<String>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// AuditLog is system-generated — only read operations are exposed.
#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLog>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: AuditLogFilter) -> Result<Vec<AuditLog>, crate::errors::RepositoryError>;
    async fn count(&self, filter: AuditLogFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, item: &AuditLog) -> Result<AuditLog, crate::errors::RepositoryError>;
}
