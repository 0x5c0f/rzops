use async_trait::async_trait;
use uuid::Uuid;

use crate::models::data_center::DataCenter;

/// Filter criteria for listing data centers.
#[derive(Debug, Clone, Default)]
pub struct DataCenterFilter {
    pub status: Option<String>,
    pub country: Option<String>,
    pub q: Option<String>, // search by name
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// DataCenter repository port — domain defines the interface, infra implements it.
#[async_trait]
pub trait DataCenterRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DataCenter>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: DataCenterFilter) -> Result<Vec<DataCenter>, crate::errors::RepositoryError>;
    async fn count(&self, filter: DataCenterFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, datacenter: &DataCenter) -> Result<DataCenter, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, datacenter: &DataCenter) -> Result<Option<DataCenter>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
