use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::IpStatus;

/// ServerIP entity — maps to cmdb_server_ip.
#[derive(Debug, Clone)]
pub struct ServerIP {
    pub id: Uuid,
    pub server_id: Uuid,
    pub ip_address: String,
    pub ip_type: String,
    pub is_primary: bool,
    pub isp_provider_id: Option<Uuid>,
    pub description: Option<String>,
    pub status: IpStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
