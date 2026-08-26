use chrono::{DateTime, Utc};
use uuid::Uuid;


/// Provider entity — maps to cmdb_provider.
#[derive(Debug, Clone)]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub provider_types: Vec<String>, // stored as JSONB array of ProviderType codes
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_qq: Option<String>,
    pub fax: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
