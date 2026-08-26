use async_trait::async_trait;
use uuid::Uuid;

use crate::models::server_ip::ServerIP;

/// Filter criteria for listing server IPs.
#[derive(Debug, Clone, Default)]
pub struct ServerIpFilter {
    pub server_id: Option<Uuid>,
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// ServerIP repository port.
#[async_trait]
pub trait ServerIpRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerIP>, sqlx::Error>;
    async fn find_all(&self, filter: ServerIpFilter) -> Result<Vec<ServerIP>, sqlx::Error>;
    async fn count(&self, filter: ServerIpFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, ip: &ServerIP) -> Result<ServerIP, sqlx::Error>;
    async fn update(&self, id: Uuid, ip: &ServerIP) -> Result<Option<ServerIP>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
