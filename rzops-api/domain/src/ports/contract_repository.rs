use async_trait::async_trait;
use uuid::Uuid;
use crate::models::contract::Contract;

#[derive(Debug, Clone, Default)]
pub struct ContractFilter {
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[async_trait]
pub trait ContractRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Contract>, sqlx::Error>;
    async fn find_all(&self, filter: ContractFilter) -> Result<Vec<Contract>, sqlx::Error>;
    async fn count(&self, filter: ContractFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, item: &Contract) -> Result<Contract, sqlx::Error>;
    async fn update(&self, id: Uuid, item: &Contract) -> Result<Option<Contract>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
