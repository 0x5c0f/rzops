use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

/// AuditLog entity — maps to cmdb_audit_log.
#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub extra_data: Value,
    pub created_at: DateTime<Utc>,
}
