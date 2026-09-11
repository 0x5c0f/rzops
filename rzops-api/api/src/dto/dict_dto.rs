use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Request body for creating a dict item.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateDictRequest {
    pub dict_type: String,
    pub dict_code: String,
    pub dict_label: String,
    pub sort_order: Option<i32>,
    pub enabled: Option<bool>,
    pub remark: Option<String>,
    pub extra_data: Option<Value>,
}

/// Request body for updating a dict item.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateDictRequest {
    pub dict_label: Option<String>,
    pub sort_order: Option<i32>,
    pub enabled: Option<bool>,
    pub remark: Option<String>,
    pub extra_data: Option<Value>,
}

/// Query parameters for listing dict items.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListDictsQuery {
    pub dict_type: Option<String>,
    pub enabled_only: Option<bool>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a dict item.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DictResponse {
    pub id: Uuid,
    pub dict_type: String,
    pub dict_code: String,
    pub dict_label: String,
    pub sort_order: i32,
    pub enabled: bool,
    pub remark: Option<String>,
    pub extra_data: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DictListResponse {
    pub data: Vec<DictResponse>,
    pub count: i64,
}
