use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for creating a provider.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateProviderRequest {
    pub name: String,
    pub provider_types: Vec<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_qq: Option<String>,
    pub fax: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Request body for updating a provider.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateProviderRequest {
    pub name: Option<String>,
    pub provider_types: Option<Vec<String>>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_qq: Option<String>,
    pub fax: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Query parameters for listing providers.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListProvidersQuery {
    pub status: Option<String>,
    pub country: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a provider.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ProviderResponse {
    pub id: Uuid,
    pub name: String,
    pub provider_types: Vec<String>,
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

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ProviderListResponse {
    pub data: Vec<ProviderResponse>,
    pub count: i64,
}

/// Generic error response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}
