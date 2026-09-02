use async_trait::async_trait;
use uuid::Uuid;

use crate::models::ops_site::OpsSite;

/// Filter criteria for listing ops sites.
#[derive(Debug, Clone, Default)]
pub struct OpsSiteFilter {
    pub status: Option<String>,
    pub environment: Option<String>,
    pub importance: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// OpsSite repository port.
#[async_trait]
pub trait OpsSiteRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<OpsSite>, sqlx::Error>;
    async fn find_all(&self, filter: OpsSiteFilter) -> Result<Vec<OpsSite>, sqlx::Error>;
    async fn count(&self, filter: OpsSiteFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, site: &OpsSite) -> Result<OpsSite, sqlx::Error>;
    async fn update(&self, id: Uuid, site: &OpsSite) -> Result<Option<OpsSite>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
