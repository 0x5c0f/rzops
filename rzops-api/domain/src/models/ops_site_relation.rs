use chrono::{DateTime, Utc};
use uuid::Uuid;


/// OpsSiteServer relation — maps to cmdb_ops_site_server.
#[derive(Debug, Clone)]
pub struct OpsSiteServer {
    pub id: Uuid,
    pub site_id: Uuid,
    pub server_id: Uuid,
    pub deploy_role: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

/// OpsSiteDatabase relation — maps to cmdb_ops_site_database.
#[derive(Debug, Clone)]
pub struct OpsSiteDatabase {
    pub id: Uuid,
    pub site_id: Uuid,
    pub database_instance_id: Uuid,
    pub usage_type: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

/// OpsSiteDomain relation — maps to cmdb_ops_site_domain.
#[derive(Debug, Clone)]
pub struct OpsSiteDomain {
    pub id: Uuid,
    pub site_id: Uuid,
    pub domain_id: Uuid,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}
