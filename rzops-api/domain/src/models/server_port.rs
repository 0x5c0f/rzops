use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::Protocol;

/// ServerPort entity — maps to cmdb_server_port.
#[derive(Debug, Clone)]
pub struct ServerPort {
    pub id: Uuid,
    pub server_id: Uuid,
    pub protocol: Protocol,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
