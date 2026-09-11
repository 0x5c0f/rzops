use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;


/// Server entity — maps to cmdb_server.
#[derive(Debug, Clone)]
pub struct Server {
    pub id: Uuid,
    pub asset_code: Option<String>,
    pub name: String,
    pub primary_ip: Option<String>,
    pub location: Option<String>,
    pub isp_provider_id: Option<Uuid>,
    pub data_center_id: Option<Uuid>,
    pub hosting_type: Option<String>,
    pub is_dual_line: bool,
    pub lease_start_date: Option<NaiveDate>,
    pub lease_end_date: Option<NaiveDate>,
    pub price: Option<Decimal>,
    pub price_currency: String,
    pub server_type: Option<String>,
    pub role_tags: Vec<String>,
    pub is_database_server: bool,
    pub cpu: Option<String>,
    pub memory_gb: Option<i32>,
    pub is_raid: bool,
    pub raid_level: Option<String>,
    pub disk_layout: Option<String>,
    pub hardware_config: Option<String>,
    pub architecture: Option<String>,
    pub maintainer_id: Option<Uuid>,
    pub brand: Option<String>,
    pub warranty_info: Option<String>,
    pub operating_system: Option<String>,
    pub web_server_type: Vec<String>,
    pub server_provider_id: Option<Uuid>,
    pub software_provider_id: Option<Uuid>,
    pub status: String,
    pub environment: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub offline_reason: Option<String>,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
