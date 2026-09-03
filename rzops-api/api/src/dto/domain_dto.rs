use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateDomainRequest {
    pub domain_name: String,
    pub registered_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub renewal_amount: Option<String>,
    pub renewal_currency: Option<String>,
    pub provider_id: Option<Uuid>,
    pub platform_phone: Option<String>,
    pub domain_email: Option<String>,
    pub privacy_status: Option<String>,
    pub is_enabled: Option<bool>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateDomainRequest {
    pub domain_name: Option<String>,
    pub registered_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub renewal_amount: Option<String>,
    pub renewal_currency: Option<String>,
    pub provider_id: Option<Uuid>,
    pub platform_phone: Option<String>,
    pub domain_email: Option<String>,
    pub privacy_status: Option<String>,
    pub is_enabled: Option<bool>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListDomainsQuery {
    pub is_enabled: Option<bool>,
    pub provider_id: Option<Uuid>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DomainResponse {
    pub id: Uuid,
    pub domain_name: String,
    pub registered_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub renewal_amount: Option<String>,
    pub renewal_currency: String,
    pub provider_id: Option<Uuid>,
    pub platform_phone: Option<String>,
    pub domain_email: Option<String>,
    pub privacy_status: Option<String>,
    pub is_enabled: bool,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DomainListResponse {
    pub data: Vec<DomainResponse>,
    pub count: i64,
}
