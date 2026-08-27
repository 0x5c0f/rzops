use async_trait::async_trait;
use uuid::Uuid;

use crate::models::provider::Provider;

/// Filter criteria for listing providers.
#[derive(Debug, Clone, Default)]
pub struct ProviderFilter {
    pub status: Option<String>,
    pub q: Option<String>, // search by name
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Provider repository port — domain defines the interface, infra implements it.
#[async_trait]
pub trait ProviderRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Provider>, sqlx::Error>;
    async fn find_all(&self, filter: ProviderFilter) -> Result<Vec<Provider>, sqlx::Error>;
    async fn count(&self, filter: ProviderFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, provider: &Provider) -> Result<Provider, sqlx::Error>;
    async fn update(&self, id: Uuid, provider: &Provider) -> Result<Option<Provider>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
