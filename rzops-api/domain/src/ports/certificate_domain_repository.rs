use async_trait::async_trait;
use uuid::Uuid;

use crate::models::certificate_domain::CertificateDomain;

/// Filter criteria for listing certificate-domain bindings.
#[derive(Debug, Clone, Default)]
pub struct CertificateDomainFilter {
    pub certificate_id: Option<Uuid>,
    pub domain_id: Option<Uuid>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// CertificateDomain repository port.
#[async_trait]
pub trait CertificateDomainRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CertificateDomain>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: CertificateDomainFilter) -> Result<Vec<CertificateDomain>, crate::errors::RepositoryError>;
    async fn count(&self, filter: CertificateDomainFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, cd: &CertificateDomain) -> Result<CertificateDomain, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, cd: &CertificateDomain) -> Result<Option<CertificateDomain>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
