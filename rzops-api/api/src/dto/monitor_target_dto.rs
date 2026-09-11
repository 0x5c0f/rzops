use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateMonitorTargetRequest {
    pub name: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub monitor_type: Option<String>,
    pub endpoint: Option<String>,
    pub interval_seconds: Option<i32>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateMonitorTargetRequest {
    pub name: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub monitor_type: Option<String>,
    pub endpoint: Option<String>,
    pub interval_seconds: Option<i32>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListMonitorTargetsQuery {
    pub status: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MonitorTargetResponse {
    pub id: Uuid,
    pub name: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub target_name: Option<String>,
    pub monitor_type: Option<String>,
    pub endpoint: Option<String>,
    pub interval_seconds: Option<i32>,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MonitorTargetListResponse {
    pub data: Vec<MonitorTargetResponse>,
    pub count: i64,
}
