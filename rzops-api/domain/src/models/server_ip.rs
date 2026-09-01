use chrono::{DateTime, Utc};
use uuid::Uuid;


/// ServerIP entity — maps to cmdb_server_ip.
#[derive(Debug, Clone)]
pub struct ServerIP {
    pub id: Uuid,
    pub server_id: Option<Uuid>,
    pub ip_address: String,
    pub nic_name: Option<String>,
    pub ip_type: String,
    pub is_primary: bool,
    pub isp_provider_id: Option<Uuid>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
