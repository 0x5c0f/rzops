use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;


/// Domain entity — maps to cmdb_domain.
/// Named `DomainAsset` to avoid collision with Rust's `domain` keyword.
#[derive(Debug, Clone)]
pub struct DomainAsset {
    pub id: Uuid,
    pub domain_name: String,
    pub business_unit_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub expiry_date: Option<NaiveDate>,
    pub renewal_amount: Option<Decimal>,
    pub renewal_currency: String,
    pub provider_id: Option<Uuid>,
    pub account_credential_id: Option<Uuid>,
    pub platform_phone: Option<String>,
    pub domain_email: Option<String>,
    pub privacy_status: Option<String>,
    pub is_enabled: bool,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
