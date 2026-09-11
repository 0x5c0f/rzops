use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for creating a server port template.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateServerPortTemplateRequest {
    pub name: String,
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: Option<bool>,
    pub description: Option<String>,
}

/// Request body for updating a server port template.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateServerPortTemplateRequest {
    pub name: Option<String>,
    pub protocol: Option<String>,
    pub port: Option<i32>,
    pub service_name: Option<String>,
    pub access_scope: Option<String>,
    pub is_enabled: Option<bool>,
    pub description: Option<String>,
}

/// Query parameters for listing server port templates.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListServerPortTemplatesQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a server port template.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerPortTemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerPortTemplateListResponse {
    pub data: Vec<ServerPortTemplateResponse>,
    pub count: i64,
}
