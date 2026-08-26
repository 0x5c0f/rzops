use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateContractRequest {
    pub name: String,
    pub provider_id: Option<Uuid>,
    pub subject_type: Option<String>,
    pub subject_id: Option<Uuid>,
    pub contract_no: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateContractRequest {
    pub name: Option<String>,
    pub provider_id: Option<Uuid>,
    pub subject_type: Option<String>,
    pub subject_id: Option<Uuid>,
    pub contract_no: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListContractsQuery {
    pub status: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ContractResponse {
    pub id: Uuid,
    pub name: String,
    pub provider_id: Option<Uuid>,
    pub subject_type: Option<String>,
    pub subject_id: Option<Uuid>,
    pub contract_no: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub amount: Option<String>,
    pub currency: String,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ContractListResponse {
    pub data: Vec<ContractResponse>,
    pub count: i64,
}
