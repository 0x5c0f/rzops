use chrono::{DateTime, Utc};
use uuid::Uuid;

/// ServerPort entity — maps to cmdb_server_port (端口，每台服务器独立，一对多).
#[derive(Debug, Clone)]
pub struct ServerPort {
    pub id: Uuid,
    /// 所属服务器 id（一对多，每台服务器拥有独立的端口记录）
    pub server_id: Uuid,
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// 所属服务器名称（查询时聚合，用于展示）
    pub server_name: Option<String>,
    /// 所属服务器状态（查询时聚合，用于展示）
    pub server_status: Option<String>,
}
