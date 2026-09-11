use async_trait::async_trait;
use uuid::Uuid;

use crate::models::domain_asset::DomainAsset;

/// Filter criteria for listing domains.
#[derive(Debug, Clone, Default)]
pub struct DomainFilter {
    pub is_enabled: Option<bool>,
    pub provider_id: Option<Uuid>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Domain repository port.
#[async_trait]
pub trait DomainRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAsset>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: DomainFilter) -> Result<Vec<DomainAsset>, crate::errors::RepositoryError>;
    async fn count(&self, filter: DomainFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, domain: &DomainAsset) -> Result<DomainAsset, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, domain: &DomainAsset) -> Result<Option<DomainAsset>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
