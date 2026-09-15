use async_trait::async_trait;
use uuid::Uuid;

use crate::models::database_instance::DatabaseInstance;

/// Filter criteria for listing database instances.
#[derive(Debug, Clone, Default)]
pub struct DatabaseInstanceFilter {
    pub status: Option<String>,
    pub environment: Option<String>,
    pub db_type: Option<String>,
    pub server_id: Option<Uuid>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// DatabaseInstance repository port.
#[async_trait]
pub trait DatabaseInstanceRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DatabaseInstance>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: DatabaseInstanceFilter) -> Result<Vec<DatabaseInstance>, crate::errors::RepositoryError>;
    async fn count(&self, filter: DatabaseInstanceFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, db: &DatabaseInstance) -> Result<DatabaseInstance, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, db: &DatabaseInstance) -> Result<Option<DatabaseInstance>, crate::errors::RepositoryError>;
    /// 解除数据库实例与服务器的绑定（server_id 置空），实例记录本身保留
    async fn unbind(&self, id: Uuid) -> Result<Option<DatabaseInstance>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
