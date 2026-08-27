use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;


/// Certificate entity — maps to cmdb_certificate.
#[derive(Debug, Clone)]
pub struct Certificate {
    pub id: Uuid,
    pub name: String,
    pub provider_id: Option<Uuid>,
    pub lease_start_date: Option<NaiveDate>,
    pub lease_end_date: Option<NaiveDate>,
    pub certificate_type: Option<String>,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
