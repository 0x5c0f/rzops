use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateCredentialRequest {
    pub name: String,
    pub credential_type: String,
    pub username: Option<String>,
    pub secret_ref: Option<String>,
    pub owner_id: Option<Uuid>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateCredentialRequest {
    pub name: Option<String>,
    pub credential_type: Option<String>,
    pub username: Option<String>,
    pub secret_ref: Option<String>,
    pub owner_id: Option<Uuid>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListCredentialsQuery {
    pub status: Option<String>,
    pub credential_type: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CredentialResponse {
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

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CredentialListResponse {
    pub data: Vec<CredentialResponse>,
    pub count: i64,
}
