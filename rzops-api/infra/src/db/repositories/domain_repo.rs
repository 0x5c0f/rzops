use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::domain_asset::DomainAsset;
use rzops_domain::ports::domain_repository::{DomainFilter, DomainRepository};

pub struct PgDomainRepository {
    pool: Pool<Postgres>,
}

impl PgDomainRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}



fn row_to_domain(row: &sqlx::postgres::PgRow) -> DomainAsset {
    let privacy_str: Option<String> = row.get("privacy_status");
    DomainAsset {
        id: row.get("id"),
        domain_name: row.get("domain_name"),
        business_unit_id: row.get("business_unit_id"),
        company_id: row.get("company_id"),
        expiry_date: row.get("expiry_date"),
        renewal_amount: row.get::<Option<Decimal>, _>("renewal_amount"),
        renewal_currency: row.get("renewal_currency"),
        provider_id: row.get("provider_id"),
        account_credential_id: row.get("account_credential_id"),
        platform_phone: row.get("platform_phone"),
        domain_email: row.get("domain_email"),
        privacy_status: privacy_str,
        is_enabled: row.get("is_enabled"),
        remarks: row.get("remarks"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

const SELECT_COLS: &str = r#"id, domain_name, business_unit_id, company_id, expiry_date,
    renewal_amount, renewal_currency, provider_id, account_credential_id,
    platform_phone, domain_email, privacy_status::text, is_enabled, remarks,
    created_at, updated_at"#;

#[async_trait]
impl DomainRepository for PgDomainRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAsset>, sqlx::Error> {
        let row = sqlx::query(&format!("SELECT {} FROM cmdb_domain WHERE id = $1", SELECT_COLS))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| row_to_domain(&r)))
    }

    async fn find_all(&self, filter: DomainFilter) -> Result<Vec<DomainAsset>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_domain WHERE 1=1", SELECT_COLS);
        let mut binds: Vec<String> = Vec::new();
        if let Some(enabled) = filter.is_enabled {
            sql.push_str(&format!(" AND is_enabled = {}", enabled));
        }
        if let Some(ref q) = filter.q {
            sql.push_str(" AND domain_name ILIKE $1");
            binds.push(format!("%{}%", q));
        }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(limit) = filter.limit { sql.push_str(&format!(" LIMIT {}", limit)); }
        if let Some(offset) = filter.offset { sql.push_str(&format!(" OFFSET {}", offset)); }
        let mut query = sqlx::query(&sql);
        for bind in &binds { query = query.bind(bind); }
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_domain(r)).collect())
    }

    async fn count(&self, filter: DomainFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_domain WHERE 1=1");
        let mut binds: Vec<String> = Vec::new();
        if let Some(enabled) = filter.is_enabled {
            sql.push_str(&format!(" AND is_enabled = {}", enabled));
        }
        if let Some(ref q) = filter.q {
            sql.push_str(" AND domain_name ILIKE $1");
            binds.push(format!("%{}%", q));
        }
        let mut query = sqlx::query(&sql);
        for bind in &binds { query = query.bind(bind); }
        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, d: &DomainAsset) -> Result<DomainAsset, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"INSERT INTO cmdb_domain
               (id, domain_name, business_unit_id, company_id, expiry_date, renewal_amount,
                renewal_currency, provider_id, account_credential_id, platform_phone, domain_email,
                privacy_status, is_enabled, remarks, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)
               RETURNING {}"#, SELECT_COLS
        ))
        .bind(d.id).bind(&d.domain_name).bind(d.business_unit_id).bind(d.company_id)
        .bind(d.expiry_date).bind(d.renewal_amount).bind(&d.renewal_currency)
        .bind(d.provider_id).bind(d.account_credential_id).bind(&d.platform_phone)
        .bind(&d.domain_email)
        .bind(d.privacy_status.clone())
        .bind(d.is_enabled).bind(&d.remarks).bind(d.created_at).bind(d.updated_at)
        .fetch_one(&self.pool).await?;
        Ok(row_to_domain(&row))
    }

    async fn update(&self, id: Uuid, d: &DomainAsset) -> Result<Option<DomainAsset>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"UPDATE cmdb_domain SET
                domain_name=$2, business_unit_id=$3, company_id=$4, expiry_date=$5,
                renewal_amount=$6, renewal_currency=$7, provider_id=$8, account_credential_id=$9,
                platform_phone=$10, domain_email=$11, privacy_status=$12, is_enabled=$13,
                remarks=$14, updated_at=$15
               WHERE id=$1 RETURNING {}"#, SELECT_COLS
        ))
        .bind(id).bind(&d.domain_name).bind(d.business_unit_id).bind(d.company_id)
        .bind(d.expiry_date).bind(d.renewal_amount).bind(&d.renewal_currency)
        .bind(d.provider_id).bind(d.account_credential_id).bind(&d.platform_phone)
        .bind(&d.domain_email)
        .bind(d.privacy_status.clone())
        .bind(d.is_enabled).bind(&d.remarks).bind(d.updated_at)
        .fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_domain(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cmdb_domain WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }
}
