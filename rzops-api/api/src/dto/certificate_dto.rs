use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateCertificateRequest {
    pub name: String,
    pub provider_id: Option<Uuid>,
    pub lease_start_date: Option<NaiveDate>,
    pub lease_end_date: Option<NaiveDate>,
    pub certificate_type: Option<String>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateCertificateRequest {
    pub name: Option<String>,
    pub provider_id: Option<Uuid>,
    pub lease_start_date: Option<NaiveDate>,
    pub lease_end_date: Option<NaiveDate>,
    pub certificate_type: Option<String>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListCertificatesQuery {
    pub status: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CertificateResponse {
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
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CertificateListResponse {
    pub data: Vec<CertificateResponse>,
    pub count: i64,
}
