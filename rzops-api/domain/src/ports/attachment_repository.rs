use async_trait::async_trait;
use uuid::Uuid;
use crate::models::attachment::Attachment;

#[derive(Debug, Clone, Default)]
pub struct AttachmentFilter {
    pub status: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[async_trait]
pub trait AttachmentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Attachment>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: AttachmentFilter) -> Result<Vec<Attachment>, crate::errors::RepositoryError>;
    async fn count(&self, filter: AttachmentFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, item: &Attachment) -> Result<Attachment, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, item: &Attachment) -> Result<Option<Attachment>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
