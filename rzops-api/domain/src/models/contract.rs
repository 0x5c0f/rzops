use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;


/// Contract entity — maps to cmdb_contract.
#[derive(Debug, Clone)]
pub struct Contract {
    pub id: Uuid,
    pub name: String,
    pub provider_id: Option<Uuid>,
    pub subject_type: Option<String>,
    pub subject_id: Option<Uuid>,
    pub contract_no: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub amount: Option<Decimal>,
    pub currency: String,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
