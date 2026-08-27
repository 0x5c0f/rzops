use chrono::{DateTime, Utc};
use uuid::Uuid;


/// DatabaseInstance entity — maps to cmdb_database_instance.
#[derive(Debug, Clone)]
pub struct DatabaseInstance {
    pub id: Uuid,
    pub server_id: Option<Uuid>,
    pub name: String,
    pub db_type: String,
    pub description: Option<String>,
    pub status: String,
    pub offline_time: Option<DateTime<Utc>>,
    pub is_self_installed: bool,
    pub importance: Option<String>,
    pub is_ops_managed: bool,
    pub backup_plan_id: Option<Uuid>,
    pub monitor_target_id: Option<Uuid>,
    pub port: Option<i32>,
    pub instance_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
