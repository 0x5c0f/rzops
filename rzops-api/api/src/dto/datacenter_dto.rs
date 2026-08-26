use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for creating a data center.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateDataCenterRequest {
    pub name: String,
    pub provider_id: Option<Uuid>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub line_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Request body for updating a data center.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateDataCenterRequest {
    pub name: Option<String>,
    pub provider_id: Option<Uuid>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub line_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Query parameters for listing data centers.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListDataCentersQuery {
    pub status: Option<String>,
    pub country: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a data center.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DataCenterResponse {
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

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DataCenterListResponse {
    pub data: Vec<DataCenterResponse>,
    pub count: i64,
}
