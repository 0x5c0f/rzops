use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateOpsSiteRequest {
    pub name: String,
    pub url: Option<String>,
    pub business_unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub service_target: Option<String>,
    pub importance: Option<String>,
    pub online_time: Option<DateTime<Utc>>,
    pub code_repo_type: Option<String>,
    pub code_repo_url: Option<String>,
    pub purpose: Option<String>,
    pub is_internal_system: Option<bool>,
    pub language_runtime: Option<String>,
    pub web_framework: Option<String>,
    pub uses_cdn: Option<bool>,
    pub is_test_site: Option<bool>,
    pub backup_plan_id: Option<Uuid>,
    pub last_backup_time: Option<DateTime<Utc>>,
    pub monitor_target_id: Option<Uuid>,
    pub status: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub offline_reason: Option<String>,
    pub function_summary: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateOpsSiteRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub business_unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub service_target: Option<String>,
    pub importance: Option<String>,
    pub online_time: Option<DateTime<Utc>>,
    pub code_repo_type: Option<String>,
    pub code_repo_url: Option<String>,
    pub purpose: Option<String>,
    pub is_internal_system: Option<bool>,
    pub language_runtime: Option<String>,
    pub web_framework: Option<String>,
    pub uses_cdn: Option<bool>,
    pub is_test_site: Option<bool>,
    pub backup_plan_id: Option<Uuid>,
    pub last_backup_time: Option<DateTime<Utc>>,
    pub monitor_target_id: Option<Uuid>,
    pub status: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub offline_reason: Option<String>,
    pub function_summary: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListOpsSitesQuery {
    pub status: Option<String>,
    pub importance: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OpsSiteResponse {
    pub id: Uuid,
    pub name: String,
    pub url: Option<String>,
    pub business_unit_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub service_target: Option<String>,
    pub importance: Option<String>,
    pub online_time: Option<DateTime<Utc>>,
    pub code_repo_type: Option<String>,
    pub code_repo_url: Option<String>,
    pub purpose: Option<String>,
    pub is_internal_system: bool,
    pub language_runtime: Option<String>,
    pub web_framework: Option<String>,
    pub uses_cdn: Option<bool>,
    pub is_test_site: bool,
    pub backup_plan_id: Option<Uuid>,
    pub last_backup_time: Option<DateTime<Utc>>,
    pub monitor_target_id: Option<Uuid>,
    pub status: String,
    pub offline_time: Option<DateTime<Utc>>,
    pub offline_reason: Option<String>,
    pub function_summary: Option<String>,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OpsSiteListResponse {
    pub data: Vec<OpsSiteResponse>,
    pub count: i64,
}
