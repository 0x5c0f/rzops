use chrono::{DateTime, Utc};
use uuid::Uuid;

/// CertificateDomain entity — maps to cmdb_certificate_domain.
#[derive(Debug, Clone)]
pub struct CertificateDomain {
    pub id: Uuid,
    pub certificate_id: Uuid,
    pub domain_id: Option<Uuid>,
    pub domain_pattern: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}
