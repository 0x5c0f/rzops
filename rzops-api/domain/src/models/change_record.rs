use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::enums::ChangeType;

/// ChangeRecord entity — maps to cmdb_change_record.
#[derive(Debug, Clone)]
pub struct ChangeRecord {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub change_type: ChangeType,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub before_data: Value,
    pub after_data: Value,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
}
