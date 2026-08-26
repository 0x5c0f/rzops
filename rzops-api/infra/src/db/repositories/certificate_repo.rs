use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::enums::{CertificateStatus, CertificateType};
use rzops_domain::models::certificate::Certificate;
use rzops_domain::ports::certificate_repository::{CertificateFilter, CertificateRepository};

pub struct PgCertificateRepository {
    pool: Pool<Postgres>,
}

impl PgCertificateRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn parse_cert_status(s: &str) -> CertificateStatus {
    match s {
        "active" => CertificateStatus::Active,
        "expired" => CertificateStatus::Expired,
        "archived" => CertificateStatus::Archived,
        _ => CertificateStatus::Active,
    }
}

fn cert_status_to_string(s: &CertificateStatus) -> String {
    match s {
        CertificateStatus::Active => "active".to_string(),
        CertificateStatus::Expired => "expired".to_string(),
        CertificateStatus::Archived => "archived".to_string(),
    }
}

fn parse_cert_type(s: &str) -> CertificateType {
    match s {
        "single" => CertificateType::Single,
        "multi_domain" => CertificateType::MultiDomain,
        "wildcard" => CertificateType::Wildcard,
        _ => CertificateType::Other,
    }
}

fn row_to_certificate(row: &sqlx::postgres::PgRow) -> Certificate {
    let status_str: String = row.get("status");
    let type_str: Option<String> = row.get("certificate_type");
    Certificate {
        id: row.get("id"),
        name: row.get("name"),
        provider_id: row.get("provider_id"),
        lease_start_date: row.get("lease_start_date"),
        lease_end_date: row.get("lease_end_date"),
        certificate_type: type_str.map(|s| parse_cert_type(&s)),
        status: parse_cert_status(&status_str),
        private_key_credential_id: row.get("private_key_credential_id"),
        remarks: row.get("remarks"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

#[async_trait]
impl CertificateRepository for PgCertificateRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Certificate>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT id, name, provider_id, lease_start_date, lease_end_date,
                      certificate_type::text, status::text, private_key_credential_id,
                      remarks, created_at, updated_at
               FROM cmdb_certificate WHERE id = $1"#,
        )
        .bind(id).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_certificate(&r)))
    }

    async fn find_all(&self, filter: CertificateFilter) -> Result<Vec<Certificate>, sqlx::Error> {
        let mut sql = String::from(
            r#"SELECT id, name, provider_id, lease_start_date, lease_end_date,
                      certificate_type::text, status::text, private_key_credential_id,
                      remarks, created_at, updated_at
               FROM cmdb_certificate WHERE 1=1"#,
        );
        let mut idx = 1;
        let s_status = filter.status.as_ref();
        let s_q = filter.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(limit) = filter.limit { sql.push_str(&format!(" LIMIT {}", limit)); }
        if let Some(offset) = filter.offset { sql.push_str(&format!(" OFFSET {}", offset)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_certificate(r)).collect())
    }

    async fn count(&self, filter: CertificateFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_certificate WHERE 1=1");
        let mut idx = 1;
        let s_status = filter.status.as_ref();
        let s_q = filter.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, c: &Certificate) -> Result<Certificate, sqlx::Error> {
        let row = sqlx::query(
            r#"INSERT INTO cmdb_certificate
               (id, name, provider_id, lease_start_date, lease_end_date, certificate_type,
                status, private_key_credential_id, remarks, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
               RETURNING id, name, provider_id, lease_start_date, lease_end_date,
                         certificate_type::text, status::text, private_key_credential_id,
                         remarks, created_at, updated_at"#,
        )
        .bind(c.id).bind(&c.name).bind(c.provider_id).bind(c.lease_start_date)
        .bind(c.lease_end_date).bind(c.certificate_type.as_ref().map(|t| match t {
            CertificateType::Single => "single".to_string(),
            CertificateType::MultiDomain => "multi_domain".to_string(),
            CertificateType::Wildcard => "wildcard".to_string(),
            CertificateType::Other => "other".to_string(),
        }))
        .bind(cert_status_to_string(&c.status))
        .bind(c.private_key_credential_id).bind(&c.remarks).bind(c.created_at).bind(c.updated_at)
        .fetch_one(&self.pool).await?;
        Ok(row_to_certificate(&row))
    }

    async fn update(&self, id: Uuid, c: &Certificate) -> Result<Option<Certificate>, sqlx::Error> {
        let row = sqlx::query(
            r#"UPDATE cmdb_certificate SET
                name=$2, provider_id=$3, lease_start_date=$4, lease_end_date=$5,
                certificate_type=$6, status=$7, private_key_credential_id=$8,
                remarks=$9, updated_at=$10
               WHERE id=$1
               RETURNING id, name, provider_id, lease_start_date, lease_end_date,
                         certificate_type::text, status::text, private_key_credential_id,
                         remarks, created_at, updated_at"#,
        )
        .bind(id).bind(&c.name).bind(c.provider_id).bind(c.lease_start_date)
        .bind(c.lease_end_date).bind(c.certificate_type.as_ref().map(|t| match t {
            CertificateType::Single => "single".to_string(),
            CertificateType::MultiDomain => "multi_domain".to_string(),
            CertificateType::Wildcard => "wildcard".to_string(),
            CertificateType::Other => "other".to_string(),
        }))
        .bind(cert_status_to_string(&c.status))
        .bind(c.private_key_credential_id).bind(&c.remarks).bind(c.updated_at)
        .fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_certificate(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cmdb_certificate WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }
}
