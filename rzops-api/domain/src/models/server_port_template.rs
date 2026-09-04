use chrono::{DateTime, Utc};
use uuid::Uuid;

/// ServerPortTemplate entity — maps to cmdb_server_port_template (端口模板，用于批量录入).
#[derive(Debug, Clone)]
pub struct ServerPortTemplate {
    pub id: Uuid,
    pub name: String,
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
