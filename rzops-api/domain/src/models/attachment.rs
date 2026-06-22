use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::ReservedStatus;

/// Attachment entity — maps to cmdb_attachment.
#[derive(Debug, Clone)]
pub struct Attachment {
    pub id: Uuid,
    pub filename: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub storage_key: Option<String>,
    pub content_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by_id: Option<Uuid>,
    pub status: ReservedStatus,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
