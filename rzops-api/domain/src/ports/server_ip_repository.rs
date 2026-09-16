use async_trait::async_trait;
use uuid::Uuid;

use crate::models::server_ip::ServerIP;

/// Filter criteria for listing server IPs.
#[derive(Debug, Clone, Default)]
pub struct ServerIpFilter {
    pub server_id: Option<Uuid>,
    pub status: Option<String>,
    pub ip_type: Option<String>,
    pub q: Option<String>,
    pub server_bound: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// ServerIP repository port.
#[async_trait]
pub trait ServerIpRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerIP>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: ServerIpFilter) -> Result<Vec<ServerIP>, crate::errors::RepositoryError>;
    async fn count(&self, filter: ServerIpFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, ip: &ServerIP) -> Result<ServerIP, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, ip: &ServerIP) -> Result<Option<ServerIP>, crate::errors::RepositoryError>;
    /// 解除 IP 与服务器的绑定（server_id 置空），IP 记录本身保留
    async fn unbind(&self, id: Uuid) -> Result<Option<ServerIP>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
