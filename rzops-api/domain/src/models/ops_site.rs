use chrono::{DateTime, Utc};
use uuid::Uuid;


/// OpsSite entity — maps to cmdb_ops_site.
#[derive(Debug, Clone)]
pub struct OpsSite {
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
    pub language_runtime: Option<String>,
    pub web_framework: Option<String>,
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
