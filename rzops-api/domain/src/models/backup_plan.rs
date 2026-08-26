use chrono::{DateTime, Utc};
use uuid::Uuid;


/// BackupPlan entity — maps to cmdb_backup_plan.
#[derive(Debug, Clone)]
pub struct BackupPlan {
    pub id: Uuid,
    pub name: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub schedule: Option<String>,
    pub retention_days: Option<i32>,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
