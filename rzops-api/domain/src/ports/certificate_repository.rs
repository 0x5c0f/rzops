use async_trait::async_trait;
use uuid::Uuid;

use crate::models::certificate::Certificate;

/// Filter criteria for listing certificates.
#[derive(Debug, Clone, Default)]
pub struct CertificateFilter {
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Certificate repository port.
#[async_trait]
pub trait CertificateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Certificate>, sqlx::Error>;
    async fn find_all(&self, filter: CertificateFilter) -> Result<Vec<Certificate>, sqlx::Error>;
    async fn count(&self, filter: CertificateFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, cert: &Certificate) -> Result<Certificate, sqlx::Error>;
    async fn update(&self, id: Uuid, cert: &Certificate) -> Result<Option<Certificate>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
