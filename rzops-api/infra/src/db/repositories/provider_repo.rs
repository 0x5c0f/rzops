use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::provider::Provider;
use rzops_domain::ports::provider_repository::{ProviderFilter, ProviderRepository};

/// PostgreSQL implementation of ProviderRepository.
pub struct PgProviderRepository {
    pool: Pool<Postgres>,
}

impl PgProviderRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

/// Helper: parse a PgRow into a Provider domain model.
fn row_to_provider(row: &PgRow) -> Result<Provider, RepositoryError> {
    let status_str: String = row.get("status");
    let status = status_str;

    // Read JSONB column as serde_json::Value, then convert to Vec<String>
    let provider_types_json: serde_json::Value = row.get("provider_types");
    let provider_types: Vec<String> = match provider_types_json {
        serde_json::Value::Array(arr) => arr
            .into_iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    };

    Ok(Provider {
        id: row.get("id"),
        name: row.get("name"),
        provider_types,
        contact_name: row.get("contact_name"),
        contact_phone: row.get("contact_phone"),
        contact_qq: row.get("contact_qq"),
        fax: row.get("fax"),
        address: row.get("address"),
        website: row.get("website"),
        description: row.get("description"),
        status,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[async_trait]
impl ProviderRepository for PgProviderRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Provider>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, provider_types, contact_name, contact_phone, contact_qq,
                   fax, address, website, description, status,
                   created_at, updated_at
            FROM cmdb_provider
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await.repo()?;

        match row {
            Some(r) => Ok(Some(row_to_provider(&r)?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self, filter: ProviderFilter) -> Result<Vec<Provider>, RepositoryError> {
        let mut sql = String::from(
            r#"
            SELECT id, name, provider_types, contact_name, contact_phone, contact_qq,
                   fax, address, website, description, status,
                   created_at, updated_at
            FROM cmdb_provider
            WHERE deleted_at IS NULL
            "#,
        );

        let mut binds: Vec<String> = Vec::new();
        let mut bind_idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status = ${}", bind_idx));
            binds.push(status.clone());
            bind_idx += 1;
        }
        if let Some(ref provider_type) = filter.provider_type {
            sql.push_str(&format!(" AND provider_types ? ${}", bind_idx));
            binds.push(provider_type.clone());
            bind_idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", bind_idx));
            binds.push(format!("%{}%", q));
        }

        sql.push_str(" ORDER BY CASE WHEN status::text IN ('inactive', 'disabled', 'terminated') THEN 1 ELSE 0 END, created_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query(&sql);
        for bind in &binds {
            query = query.bind(bind);
        }

        let rows = query.fetch_all(&self.pool).await.repo()?;
        let mut providers = Vec::new();
        for row in &rows {
            providers.push(row_to_provider(row)?);
        }
        Ok(providers)
    }

    async fn count(&self, filter: ProviderFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_provider WHERE deleted_at IS NULL");

        let mut binds: Vec<String> = Vec::new();
        let mut bind_idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status = ${}", bind_idx));
            binds.push(status.clone());
            bind_idx += 1;
        }
        if let Some(ref provider_type) = filter.provider_type {
            sql.push_str(&format!(" AND provider_types ? ${}", bind_idx));
            binds.push(provider_type.clone());
            bind_idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", bind_idx));
            binds.push(format!("%{}%", q));
        }

        let mut query = sqlx::query(&sql);
        for bind in &binds {
            query = query.bind(bind);
        }

        let row = query.fetch_one(&self.pool).await.repo()?;
        let count: i64 = row.get("count");
        Ok(count)
    }

    async fn create(&self, provider: &Provider) -> Result<Provider, RepositoryError> {
        let status_str = provider.status.clone();

        let row = sqlx::query(
            r#"
            INSERT INTO cmdb_provider (
                id, name, provider_types, contact_name, contact_phone, contact_qq,
                fax, address, website, description, status,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, name, provider_types, contact_name, contact_phone, contact_qq,
                      fax, address, website, description, status,
                      created_at, updated_at
            "#,
        )
        .bind(provider.id)
        .bind(&provider.name)
        .bind(serde_json::to_value(&provider.provider_types).unwrap_or_default())
        .bind(&provider.contact_name)
        .bind(&provider.contact_phone)
        .bind(&provider.contact_qq)
        .bind(&provider.fax)
        .bind(&provider.address)
        .bind(&provider.website)
        .bind(&provider.description)
        .bind(status_str)
        .bind(provider.created_at)
        .bind(provider.updated_at)
        .fetch_one(&self.pool)
        .await.repo()?;

        row_to_provider(&row)
    }

    async fn update(&self, id: Uuid, provider: &Provider) -> Result<Option<Provider>, RepositoryError> {
        let status_str = provider.status.clone();

        let row = sqlx::query(
            r#"
            UPDATE cmdb_provider SET
                name = $2,
                provider_types = $3,
                contact_name = $4,
                contact_phone = $5,
                contact_qq = $6,
                fax = $7,
                address = $8,
                website = $9,
                description = $10,
                status = $11,
                updated_at = $12
            WHERE id = $1
            RETURNING id, name, provider_types, contact_name, contact_phone, contact_qq,
                      fax, address, website, description, status,
                      created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&provider.name)
        .bind(serde_json::to_value(&provider.provider_types).unwrap_or_default())
        .bind(&provider.contact_name)
        .bind(&provider.contact_phone)
        .bind(&provider.contact_qq)
        .bind(&provider.fax)
        .bind(&provider.address)
        .bind(&provider.website)
        .bind(&provider.description)
        .bind(status_str)
        .bind(Utc::now())
        .fetch_optional(&self.pool)
        .await.repo()?;

        match row {
            Some(r) => Ok(Some(row_to_provider(&r)?)),
            None => Ok(None),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("UPDATE cmdb_provider SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await.repo()?;

        Ok(result.rows_affected() > 0)
    }
}
