use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::data_center::DataCenter;
use rzops_domain::ports::datacenter_repository::{DataCenterFilter, DataCenterRepository};

/// PostgreSQL implementation of DataCenterRepository.
pub struct PgDataCenterRepository {
    pool: Pool<Postgres>,
}

impl PgDataCenterRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn row_to_datacenter(row: &sqlx::postgres::PgRow) -> DataCenter {
    let status_str: String = row.get("status");
    // line_type 存储为 JSONB 字符串数组
    let line_type_json: serde_json::Value = row.get("line_type");
    let line_type: Option<Vec<String>> = match line_type_json {
        serde_json::Value::Array(arr) => Some(
            arr.into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        ),
        _ => None,
    };

    DataCenter {
        id: row.get("id"),
        name: row.get("name"),
        provider_id: row.get("provider_id"),
        phone: row.get("phone"),
        address: row.get("address"),
        country: row.get("country"),
        line_type,
        description: row.get("description"),
        status: status_str,
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

#[async_trait]
impl DataCenterRepository for PgDataCenterRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DataCenter>, RepositoryError> {
        let row = sqlx::query(
            r#"SELECT id, name, provider_id, phone, address, country,
                      line_type, description, status::text, created_at, updated_at
               FROM cmdb_data_center WHERE id = $1 AND deleted_at IS NULL"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await.repo()?;

        Ok(row.map(|r| row_to_datacenter(&r)))
    }

    async fn find_all(&self, filter: DataCenterFilter) -> Result<Vec<DataCenter>, RepositoryError> {
        let mut sql = String::from(
            r#"SELECT id, name, provider_id, phone, address, country,
                      line_type, description, status::text, created_at, updated_at
               FROM cmdb_data_center WHERE deleted_at IS NULL"#,
        );
        let mut binds: Vec<String> = Vec::new();
        let mut idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status::text = ${}", idx));
            binds.push(status.clone());
            idx += 1;
        }
        if let Some(ref country) = filter.country {
            sql.push_str(&format!(" AND country = ${}", idx));
            binds.push(country.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", idx));
            binds.push(format!("%{}%", q));
        }

        sql.push_str(" ORDER BY CASE WHEN status::text IN ('inactive', 'disabled', 'offline') THEN 1 ELSE 0 END, created_at DESC");

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
        Ok(rows.iter().map(row_to_datacenter).collect())
    }

    async fn count(&self, filter: DataCenterFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_data_center WHERE deleted_at IS NULL");
        let mut binds: Vec<String> = Vec::new();
        let mut idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status::text = ${}", idx));
            binds.push(status.clone());
            idx += 1;
        }
        if let Some(ref country) = filter.country {
            sql.push_str(&format!(" AND country = ${}", idx));
            binds.push(country.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", idx));
            binds.push(format!("%{}%", q));
        }

        let mut query = sqlx::query(&sql);
        for bind in &binds {
            query = query.bind(bind);
        }

        let row = query.fetch_one(&self.pool).await.repo()?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, dc: &DataCenter) -> Result<DataCenter, RepositoryError> {
        let row = sqlx::query(
            r#"INSERT INTO cmdb_data_center
               (id, name, provider_id, phone, address, country,
                line_type, description, status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               RETURNING id, name, provider_id, phone, address, country,
                         line_type, description, status::text, created_at, updated_at"#,
        )
        .bind(dc.id)
        .bind(&dc.name)
        .bind(dc.provider_id)
        .bind(&dc.phone)
        .bind(&dc.address)
        .bind(&dc.country)
        .bind(serde_json::to_value(&dc.line_type).unwrap_or_else(|_| serde_json::json!([])))
        .bind(&dc.description)
        .bind(dc.status.clone())
        .bind(dc.created_at)
        .bind(dc.updated_at)
        .fetch_one(&self.pool)
        .await.repo()?;

        Ok(row_to_datacenter(&row))
    }

    async fn update(&self, id: Uuid, dc: &DataCenter) -> Result<Option<DataCenter>, RepositoryError> {
        let row = sqlx::query(
            r#"UPDATE cmdb_data_center SET
                name = $2, provider_id = $3, phone = $4, address = $5,
                country = $6, line_type = $7,
                description = $8, status = $9, updated_at = $10
               WHERE id = $1
               RETURNING id, name, provider_id, phone, address, country,
                         line_type, description, status::text, created_at, updated_at"#,
        )
        .bind(id)
        .bind(&dc.name)
        .bind(dc.provider_id)
        .bind(&dc.phone)
        .bind(&dc.address)
        .bind(&dc.country)
        .bind(serde_json::to_value(&dc.line_type).unwrap_or_else(|_| serde_json::json!([])))
        .bind(&dc.description)
        .bind(dc.status.clone())
        .bind(dc.updated_at)
        .fetch_optional(&self.pool)
        .await.repo()?;

        Ok(row.map(|r| row_to_datacenter(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("UPDATE cmdb_data_center SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await.repo()?;

        Ok(result.rows_affected() > 0)
    }
}
