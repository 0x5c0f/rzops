use async_trait::async_trait;
use uuid::Uuid;

use crate::models::server::Server;

/// Filter criteria for listing servers.
#[derive(Debug, Clone, Default)]
pub struct ServerFilter {
    pub status: Option<String>,
    pub environment: Option<String>,
    pub data_center_id: Option<Uuid>,
    pub server_type: Option<String>,
    pub q: Option<String>, // search by name
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Server repository port — domain defines the interface, infra implements it.
#[async_trait]
pub trait ServerRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Server>, sqlx::Error>;
    async fn find_all(&self, filter: ServerFilter) -> Result<Vec<Server>, sqlx::Error>;
    async fn count(&self, filter: ServerFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, server: &Server) -> Result<Server, sqlx::Error>;
    async fn update(&self, id: Uuid, server: &Server) -> Result<Option<Server>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
