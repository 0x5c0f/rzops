use chrono::{DateTime, Utc};
use uuid::Uuid;


/// ServerPort entity — maps to cmdb_server_port (端口定义，可关联多台服务器).
#[derive(Debug, Clone)]
pub struct ServerPort {
    pub id: Uuid,
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// 关联的服务器 id 列表（多对多，通过 cmdb_server_port_server）
    pub server_ids: Vec<Uuid>,
    /// 关联的服务器名称（查询时聚合，用于展示）
    pub server_names: Vec<String>,
}
