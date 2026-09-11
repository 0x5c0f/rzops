use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
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
        registered_date: row.get("registered_date"),
        expiry_date: row.get("expiry_date"),
        renewal_amount: row.get::<Option<Decimal>, _>("renewal_amount"),
        renewal_currency: row.get("renewal_currency"),
        provider_id: row.get("provider_id"),
        platform_phone: row.get("platform_phone"),
        domain_email: row.get("domain_email"),
        privacy_status: privacy_str,
        is_enabled: row.get("is_enabled"),
        remarks: row.get("remarks"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

const SELECT_COLS: &str = r#"id, domain_name, registered_date, expiry_date,
    renewal_amount, renewal_currency, provider_id,
    platform_phone, domain_email, privacy_status::text, is_enabled, remarks,
    created_at, updated_at, deleted_at"#;

#[async_trait]
impl DomainRepository for PgDomainRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAsset>, RepositoryError> {
        let row = sqlx::query(&format!("SELECT {} FROM cmdb_domain WHERE id = $1 AND deleted_at IS NULL", SELECT_COLS))
            .bind(id)
            .fetch_optional(&self.pool)
            .await.repo()?;
        Ok(row.map(|r| row_to_domain(&r)))
    }

    async fn find_all(&self, filter: DomainFilter) -> Result<Vec<DomainAsset>, RepositoryError> {
        let mut sql = format!("SELECT {} FROM cmdb_domain WHERE deleted_at IS NULL", SELECT_COLS);
        let mut binds: Vec<String> = Vec::new();
        let mut bind_idx = 1;
        if let Some(enabled) = filter.is_enabled {
            sql.push_str(&format!(" AND is_enabled = {}", enabled));
        }
        if let Some(provider_id) = filter.provider_id {
            sql.push_str(&format!(" AND provider_id = ${}", bind_idx));
            binds.push(provider_id.to_string());
            bind_idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND domain_name ILIKE ${}", bind_idx));
            binds.push(format!("%{}%", q));
        }
        sql.push_str(" ORDER BY CASE WHEN is_enabled = false THEN 1 ELSE 0 END, created_at DESC");
        if let Some(limit) = filter.limit { sql.push_str(&format!(" LIMIT {}", limit)); }
        if let Some(offset) = filter.offset { sql.push_str(&format!(" OFFSET {}", offset)); }
        let mut query = sqlx::query(&sql);
        for bind in &binds { query = query.bind(bind); }
        let rows = query.fetch_all(&self.pool).await.repo()?;
        Ok(rows.iter().map(|r| row_to_domain(r)).collect())
    }

    async fn count(&self, filter: DomainFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_domain WHERE deleted_at IS NULL");
        let mut binds: Vec<String> = Vec::new();
        let mut bind_idx = 1;
        if let Some(enabled) = filter.is_enabled {
            sql.push_str(&format!(" AND is_enabled = {}", enabled));
        }
        if let Some(provider_id) = filter.provider_id {
            sql.push_str(&format!(" AND provider_id = ${}", bind_idx));
            binds.push(provider_id.to_string());
            bind_idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND domain_name ILIKE ${}", bind_idx));
            binds.push(format!("%{}%", q));
        }
        let mut query = sqlx::query(&sql);
        for bind in &binds { query = query.bind(bind); }
        let row = query.fetch_one(&self.pool).await.repo()?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, d: &DomainAsset) -> Result<DomainAsset, RepositoryError> {
        let row = sqlx::query(&format!(
            r#"INSERT INTO cmdb_domain
               (id, domain_name, registered_date, expiry_date, renewal_amount,
                renewal_currency, provider_id, platform_phone, domain_email,
                privacy_status, is_enabled, remarks, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
               RETURNING {}"#, SELECT_COLS
        ))
        .bind(d.id).bind(&d.domain_name).bind(d.registered_date).bind(d.expiry_date)
        .bind(d.renewal_amount).bind(&d.renewal_currency)
        .bind(d.provider_id).bind(&d.platform_phone)
        .bind(&d.domain_email)
        .bind(d.privacy_status.clone())
        .bind(d.is_enabled).bind(&d.remarks).bind(d.created_at).bind(d.updated_at)
        .fetch_one(&self.pool).await.repo()?;
        Ok(row_to_domain(&row))
    }

    async fn update(&self, id: Uuid, d: &DomainAsset) -> Result<Option<DomainAsset>, RepositoryError> {
        let row = sqlx::query(&format!(
            r#"UPDATE cmdb_domain SET
                domain_name=$2, registered_date=$3, expiry_date=$4,
                renewal_amount=$5, renewal_currency=$6, provider_id=$7,
                platform_phone=$8, domain_email=$9, privacy_status=$10, is_enabled=$11,
                remarks=$12, updated_at=$13
               WHERE id=$1 AND deleted_at IS NULL RETURNING {}"#, SELECT_COLS
        ))
        .bind(id).bind(&d.domain_name).bind(d.registered_date).bind(d.expiry_date)
        .bind(d.renewal_amount).bind(&d.renewal_currency)
        .bind(d.provider_id).bind(&d.platform_phone)
        .bind(&d.domain_email)
        .bind(d.privacy_status.clone())
        .bind(d.is_enabled).bind(&d.remarks).bind(d.updated_at)
        .fetch_optional(&self.pool).await.repo()?;
        Ok(row.map(|r| row_to_domain(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("UPDATE cmdb_domain SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL").bind(id).execute(&self.pool).await.repo()?;
        Ok(result.rows_affected() > 0)
    }
}
