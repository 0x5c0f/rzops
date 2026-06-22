use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::{MonitorType, ReservedStatus};

/// MonitorTarget entity — maps to cmdb_monitor_target.
#[derive(Debug, Clone)]
pub struct MonitorTarget {
    pub id: Uuid,
    pub name: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub monitor_type: Option<MonitorType>,
    pub endpoint: Option<String>,
    pub interval_seconds: Option<i32>,
    pub status: ReservedStatus,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
