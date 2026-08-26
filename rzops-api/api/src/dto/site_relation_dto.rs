use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateSiteServerRelationRequest {
    pub site_id: Uuid,
    pub server_id: Uuid,
    pub deploy_role: Option<String>,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SiteServerRelationResponse {
    pub id: Uuid,
    pub site_id: Uuid,
    pub server_id: Uuid,
    pub deploy_role: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateSiteDatabaseRelationRequest {
    pub site_id: Uuid,
    pub database_instance_id: Uuid,
    pub usage_type: Option<String>,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SiteDatabaseRelationResponse {
    pub id: Uuid,
    pub site_id: Uuid,
    pub database_instance_id: Uuid,
    pub usage_type: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateSiteDomainRelationRequest {
    pub site_id: Uuid,
    pub domain_id: Uuid,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SiteDomainRelationResponse {
    pub id: Uuid,
    pub site_id: Uuid,
    pub domain_id: Uuid,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}
