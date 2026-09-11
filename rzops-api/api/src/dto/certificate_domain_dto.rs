use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for creating a certificate-domain binding.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateCertificateDomainRequest {
    pub certificate_id: Uuid,
    pub domain_id: Option<Uuid>,
    pub domain_pattern: String,
    pub is_primary: Option<bool>,
}

/// Request body for updating a certificate-domain binding.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateCertificateDomainRequest {
    pub domain_id: Option<Uuid>,
    pub domain_pattern: Option<String>,
    pub is_primary: Option<bool>,
}

/// Query parameters for listing certificate-domain bindings.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListCertificateDomainsQuery {
    pub certificate_id: Option<Uuid>,
    pub domain_id: Option<Uuid>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a certificate-domain binding.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CertificateDomainResponse {
    pub id: Uuid,
    pub certificate_id: Uuid,
    pub domain_id: Option<Uuid>,
    pub domain_pattern: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CertificateDomainListResponse {
    pub data: Vec<CertificateDomainResponse>,
    pub count: i64,
}
