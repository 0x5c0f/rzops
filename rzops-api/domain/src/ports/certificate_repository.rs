use async_trait::async_trait;
use uuid::Uuid;

use crate::models::certificate::Certificate;

/// Filter criteria for listing certificates.
#[derive(Debug, Clone, Default)]
pub struct CertificateFilter {
    pub status: Option<String>,
    pub certificate_type: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Certificate repository port.
#[async_trait]
pub trait CertificateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Certificate>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: CertificateFilter) -> Result<Vec<Certificate>, crate::errors::RepositoryError>;
    async fn count(&self, filter: CertificateFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, cert: &Certificate) -> Result<Certificate, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, cert: &Certificate) -> Result<Option<Certificate>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
