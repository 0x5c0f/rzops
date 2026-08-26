use chrono::{DateTime, Utc};
use uuid::Uuid;


/// DataCenter entity — maps to cmdb_data_center.
#[derive(Debug, Clone)]
pub struct DataCenter {
    pub id: Uuid,
    pub name: String,
    pub provider_id: Option<Uuid>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub line_type: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
