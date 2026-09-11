use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::certificate_domain::CertificateDomain;
use rzops_domain::ports::certificate_domain_repository::{
    CertificateDomainFilter, CertificateDomainRepository,
};

pub struct PgCertificateDomainRepository {
    pool: Pool<Postgres>,
}

impl PgCertificateDomainRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn row_to_certificate_domain(row: &sqlx::postgres::PgRow) -> CertificateDomain {
    CertificateDomain {
        id: row.get("id"),
        certificate_id: row.get("certificate_id"),
        domain_id: row.get("domain_id"),
        domain_pattern: row.get("domain_pattern"),
        is_primary: row.get("is_primary"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
    }
}

#[async_trait]
impl CertificateDomainRepository for PgCertificateDomainRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CertificateDomain>, RepositoryError> {
        let row = sqlx::query(
            r#"SELECT id, certificate_id, domain_id, domain_pattern, is_primary, created_at
               FROM cmdb_certificate_domain WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await.repo()?;
        Ok(row.map(|r| row_to_certificate_domain(&r)))
    }

    async fn find_all(&self, filter: CertificateDomainFilter) -> Result<Vec<CertificateDomain>, RepositoryError> {
        let mut sql = String::from(
            r#"SELECT id, certificate_id, domain_id, domain_pattern, is_primary, created_at
               FROM cmdb_certificate_domain WHERE 1=1"#,
        );

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(cid) = filter.certificate_id {
            sql.push_str(&format!(" AND certificate_id = ${}", idx));
            uuid_binds.push(cid);
            idx += 1;
        }
        if let Some(did) = filter.domain_id {
            sql.push_str(&format!(" AND domain_id = ${}", idx));
            uuid_binds.push(did);
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND domain_pattern ILIKE ${}", idx));
            string_binds.push(format!("%{}%", q));
        }

        sql.push_str(" ORDER BY is_primary DESC, created_at ASC");
        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds {
            query = query.bind(u);
        }
        for s in &string_binds {
            query = query.bind(s);
        }
        let rows = query.fetch_all(&self.pool).await.repo()?;
        Ok(rows.iter().map(|r| row_to_certificate_domain(r)).collect())
    }

    async fn count(&self, filter: CertificateDomainFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_certificate_domain WHERE 1=1");

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(cid) = filter.certificate_id {
            sql.push_str(&format!(" AND certificate_id = ${}", idx));
            uuid_binds.push(cid);
            idx += 1;
        }
        if let Some(did) = filter.domain_id {
            sql.push_str(&format!(" AND domain_id = ${}", idx));
            uuid_binds.push(did);
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND domain_pattern ILIKE ${}", idx));
            string_binds.push(format!("%{}%", q));
        }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds {
            query = query.bind(u);
        }
        for s in &string_binds {
            query = query.bind(s);
        }
        let row = query.fetch_one(&self.pool).await.repo()?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, cd: &CertificateDomain) -> Result<CertificateDomain, RepositoryError> {
        let row = sqlx::query(
            r#"INSERT INTO cmdb_certificate_domain
               (id, certificate_id, domain_id, domain_pattern, is_primary, created_at)
               VALUES ($1, $2, $3, $4, $5, $6)
               RETURNING id, certificate_id, domain_id, domain_pattern, is_primary, created_at"#,
        )
        .bind(cd.id)
        .bind(cd.certificate_id)
        .bind(cd.domain_id)
        .bind(&cd.domain_pattern)
        .bind(cd.is_primary)
        .bind(cd.created_at)
        .fetch_one(&self.pool)
        .await.repo()?;
        Ok(row_to_certificate_domain(&row))
    }

    async fn update(&self, id: Uuid, cd: &CertificateDomain) -> Result<Option<CertificateDomain>, RepositoryError> {
        let row = sqlx::query(
            r#"UPDATE cmdb_certificate_domain SET
                certificate_id = $2, domain_id = $3, domain_pattern = $4, is_primary = $5
               WHERE id = $1
               RETURNING id, certificate_id, domain_id, domain_pattern, is_primary, created_at"#,
        )
        .bind(id)
        .bind(cd.certificate_id)
        .bind(cd.domain_id)
        .bind(&cd.domain_pattern)
        .bind(cd.is_primary)
        .fetch_optional(&self.pool)
        .await.repo()?;
        Ok(row.map(|r| row_to_certificate_domain(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("DELETE FROM cmdb_certificate_domain WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await.repo()?;
        Ok(result.rows_affected() > 0)
    }
}
