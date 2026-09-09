use async_trait::async_trait;
use uuid::Uuid;

use crate::models::server_port::ServerPort;

/// Filter criteria for listing server ports.
#[derive(Debug, Clone, Default)]
pub struct ServerPortFilter {
    pub server_id: Option<Uuid>,
    pub protocol: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// ServerPort repository port.
#[async_trait]
pub trait ServerPortRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerPort>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: ServerPortFilter) -> Result<Vec<ServerPort>, crate::errors::RepositoryError>;
    async fn count(&self, filter: ServerPortFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, port: &ServerPort) -> Result<ServerPort, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, port: &ServerPort) -> Result<Option<ServerPort>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
