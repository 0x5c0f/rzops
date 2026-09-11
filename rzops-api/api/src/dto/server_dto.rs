use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for creating a server.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateServerRequest {
    pub name: String,
    pub asset_code: Option<String>,
    pub primary_ip: Option<String>,
    pub location: Option<String>,
    pub isp_provider_id: Option<Uuid>,
    pub data_center_id: Option<Uuid>,
    pub hosting_type: Option<String>,
    pub is_dual_line: Option<bool>,
    pub lease_start_date: Option<NaiveDate>,
    pub lease_end_date: Option<NaiveDate>,
    pub price: Option<String>, // Decimal as string
    pub price_currency: Option<String>,
    pub server_type: Option<String>,
    pub role_tags: Option<Vec<String>>,
    pub is_database_server: Option<bool>,
    pub cpu: Option<String>,
    pub memory_gb: Option<i32>,
    pub is_raid: Option<bool>,
    pub raid_level: Option<String>,
    pub disk_layout: Option<String>,
    pub hardware_config: Option<String>,
    pub architecture: Option<String>,
    pub maintainer_id: Option<Uuid>,
    pub brand: Option<String>,
    pub warranty_info: Option<String>,
    pub operating_system: Option<String>,
    pub web_server_type: Option<Vec<String>>,
    pub server_provider_id: Option<Uuid>,
    pub software_provider_id: Option<Uuid>,
    pub status: Option<String>,
    pub environment: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub offline_reason: Option<String>,
    pub remarks: Option<String>,
}

/// Request body for updating a server.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateServerRequest {
    pub name: Option<String>,
    pub asset_code: Option<String>,
    pub primary_ip: Option<String>,
    pub location: Option<String>,
    pub isp_provider_id: Option<Uuid>,
    pub data_center_id: Option<Uuid>,
    pub hosting_type: Option<String>,
    pub is_dual_line: Option<bool>,
    pub lease_start_date: Option<NaiveDate>,
    pub lease_end_date: Option<NaiveDate>,
    pub price: Option<String>,
    pub price_currency: Option<String>,
    pub server_type: Option<String>,
    pub role_tags: Option<Vec<String>>,
    pub is_database_server: Option<bool>,
    pub cpu: Option<String>,
    pub memory_gb: Option<i32>,
    pub is_raid: Option<bool>,
    pub raid_level: Option<String>,
    pub disk_layout: Option<String>,
    pub hardware_config: Option<String>,
    pub architecture: Option<String>,
    pub maintainer_id: Option<Uuid>,
    pub brand: Option<String>,
    pub warranty_info: Option<String>,
    pub operating_system: Option<String>,
    pub web_server_type: Option<Vec<String>>,
    pub server_provider_id: Option<Uuid>,
    pub software_provider_id: Option<Uuid>,
    pub status: Option<String>,
    pub environment: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub offline_reason: Option<String>,
    pub remarks: Option<String>,
}

/// Query parameters for listing servers.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListServersQuery {
    pub status: Option<String>,
    pub environment: Option<String>,
    pub data_center_id: Option<Uuid>,
    pub server_type: Option<String>,
    pub is_database_server: Option<bool>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a server.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerResponse {
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
    pub price: Option<String>,
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

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerListResponse {
    pub data: Vec<ServerResponse>,
    pub count: i64,
}
