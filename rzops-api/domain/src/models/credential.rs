use chrono::{DateTime, Utc};
use uuid::Uuid;


/// Credential entity — maps to cmdb_credential.
#[derive(Debug, Clone)]
pub struct Credential {
    pub id: Uuid,
    pub name: String,
    pub credential_type: String,
    pub username: Option<String>,
    pub secret_ref: Option<String>,
    pub owner_id: Option<Uuid>,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
