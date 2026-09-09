use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::server_port_template::ServerPortTemplate;
use rzops_domain::ports::server_port_template_repository::{ServerPortTemplateFilter, ServerPortTemplateRepository};

pub struct PgServerPortTemplateRepository {
    pool: Pool<Postgres>,
}

impl PgServerPortTemplateRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn row_to_tpl(row: &sqlx::postgres::PgRow) -> ServerPortTemplate {
    let protocol_str: String = row.get("protocol");
    ServerPortTemplate {
        id: row.get("id"),
        name: row.get("name"),
        protocol: protocol_str,
        port: row.get("port"),
        service_name: row.get("service_name"),
        access_scope: row.get("access_scope"),
        is_enabled: row.get("is_enabled"),
        description: row.get("description"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

#[async_trait]
impl ServerPortTemplateRepository for PgServerPortTemplateRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerPortTemplate>, RepositoryError> {
        let row = sqlx::query("SELECT * FROM cmdb_server_port_template WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await.repo()?;
        Ok(row.map(|r| row_to_tpl(&r)))
    }

    async fn find_all(&self, filter: ServerPortTemplateFilter) -> Result<Vec<ServerPortTemplate>, RepositoryError> {
        let mut sql = String::from("SELECT * FROM cmdb_server_port_template WHERE 1=1");
        let mut binds: Vec<String> = Vec::new();
        let mut idx = 1;
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND (name ILIKE ${} OR service_name ILIKE ${} OR port::text ILIKE ${})", idx, idx, idx));
            binds.push(format!("%{}%", q));
        }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        let mut query = sqlx::query(&sql);
        for b in &binds { query = query.bind(b); }
        let rows = query.fetch_all(&self.pool).await.repo()?;
        Ok(rows.iter().map(|r| row_to_tpl(r)).collect())
    }

    async fn count(&self, filter: ServerPortTemplateFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_server_port_template WHERE 1=1");
        let mut binds: Vec<String> = Vec::new();
        let mut idx = 1;
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND (name ILIKE ${} OR service_name ILIKE ${} OR port::text ILIKE ${})", idx, idx, idx));
            binds.push(format!("%{}%", q));
        }
        let mut query = sqlx::query(&sql);
        for b in &binds { query = query.bind(b); }
        let row = query.fetch_one(&self.pool).await.repo()?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, tpl: &ServerPortTemplate) -> Result<ServerPortTemplate, RepositoryError> {
        sqlx::query(
            r#"INSERT INTO cmdb_server_port_template
               (id, name, protocol, port, service_name, access_scope,
                is_enabled, description, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        )
        .bind(tpl.id)
        .bind(&tpl.name)
        .bind(tpl.protocol.clone())
        .bind(tpl.port)
        .bind(&tpl.service_name)
        .bind(&tpl.access_scope)
        .bind(tpl.is_enabled)
        .bind(&tpl.description)
        .bind(tpl.created_at)
        .bind(tpl.updated_at)
        .execute(&self.pool)
        .await.repo()?;
        Ok(tpl.clone())
    }

    async fn update(&self, id: Uuid, tpl: &ServerPortTemplate) -> Result<Option<ServerPortTemplate>, RepositoryError> {
        let row = sqlx::query(
            r#"UPDATE cmdb_server_port_template SET
                name = $2, protocol = $3, port = $4, service_name = $5,
                access_scope = $6, is_enabled = $7, description = $8, updated_at = $9
               WHERE id = $1"#,
        )
        .bind(id)
        .bind(&tpl.name)
        .bind(tpl.protocol.clone())
        .bind(tpl.port)
        .bind(&tpl.service_name)
        .bind(&tpl.access_scope)
        .bind(tpl.is_enabled)
        .bind(&tpl.description)
        .bind(tpl.updated_at)
        .execute(&self.pool)
        .await.repo()?;
        if row.rows_affected() == 0 {
            return Ok(None);
        }
        Ok(self.find_by_id(id).await?)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("DELETE FROM cmdb_server_port_template WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await.repo()?;
        Ok(result.rows_affected() > 0)
    }
}
