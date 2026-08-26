use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for creating a server IP.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateServerIpRequest {
    pub server_id: Uuid,
    pub ip_address: String,
    pub ip_type: Option<String>,
    pub is_primary: Option<bool>,
    pub isp_provider_id: Option<Uuid>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Request body for updating a server IP.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateServerIpRequest {
    pub ip_address: Option<String>,
    pub ip_type: Option<String>,
    pub is_primary: Option<bool>,
    pub isp_provider_id: Option<Uuid>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Query parameters for listing server IPs.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListServerIpsQuery {
    pub server_id: Option<Uuid>,
    pub status: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a server IP.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerIpResponse {
    pub id: Uuid,
    pub server_id: Uuid,
    pub ip_address: String,
    pub ip_type: String,
    pub is_primary: bool,
    pub isp_provider_id: Option<Uuid>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerIpListResponse {
    pub data: Vec<ServerIpResponse>,
    pub count: i64,
}
