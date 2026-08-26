use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListChangeRecordsQuery {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub change_type: Option<String>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ChangeRecordResponse {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub change_type: String,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub actor_email: Option<String>,
    pub before_data: serde_json::Value,
    pub after_data: serde_json::Value,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ChangeRecordListResponse {
    pub data: Vec<ChangeRecordResponse>,
    pub count: i64,
}
