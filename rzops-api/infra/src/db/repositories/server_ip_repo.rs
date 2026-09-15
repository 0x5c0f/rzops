use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::server_ip::ServerIP;
use rzops_domain::ports::server_ip_repository::{ServerIpFilter, ServerIpRepository};

pub struct PgServerIpRepository {
    pool: Pool<Postgres>,
}

impl PgServerIpRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}



fn row_to_server_ip(row: &sqlx::postgres::PgRow) -> ServerIP {
    let status_str: String = row.get("status");
    ServerIP {
        id: row.get("id"),
        server_id: row.get("server_id"),
        ip_address: row.get("ip_address"),
        nic_name: row.get("nic_name"),
        ip_type: row.get("ip_type"),
        is_primary: row.get("is_primary"),
        isp_provider_id: row.get("isp_provider_id"),
        description: row.get("description"),
        status: status_str,
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

#[async_trait]
impl ServerIpRepository for PgServerIpRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerIP>, RepositoryError> {
        let row = sqlx::query(
            r#"SELECT id, server_id, ip_address, nic_name, ip_type, is_primary,
                      isp_provider_id, description, status::text, created_at, updated_at
               FROM cmdb_server_ip WHERE id = $1 AND deleted_at IS NULL"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await.repo()?;
        Ok(row.map(|r| row_to_server_ip(&r)))
    }

    async fn find_all(&self, filter: ServerIpFilter) -> Result<Vec<ServerIP>, RepositoryError> {
        let mut sql = String::from(
            r#"SELECT id, server_id, ip_address, nic_name, ip_type, is_primary,
                      isp_provider_id, description, status::text, created_at, updated_at
               FROM cmdb_server_ip WHERE deleted_at IS NULL"#,
        );

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id { sql.push_str(&format!(" AND server_id = ${}", idx)); uuid_binds.push(sid); idx += 1; }
        if let Some(ref status) = filter.status { sql.push_str(&format!(" AND status::text = ${}", idx)); string_binds.push(status.clone()); idx += 1; }
        if let Some(ref ip_type) = filter.ip_type { sql.push_str(&format!(" AND ip_type = ${}", idx)); string_binds.push(ip_type.clone()); idx += 1; }
        if let Some(ref q) = filter.q { sql.push_str(&format!(" AND ip_address ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }

        sql.push_str(" ORDER BY CASE WHEN status::text NOT IN ('enabled') THEN 2 WHEN server_id IS NOT NULL AND server_id IN (SELECT id FROM cmdb_server WHERE deleted_at IS NOT NULL OR status::text IN ('retired', 'offline')) THEN 1 ELSE 0 END, created_at DESC");
        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        let rows = query.fetch_all(&self.pool).await.repo()?;
        Ok(rows.iter().map(row_to_server_ip).collect())
    }

    async fn count(&self, filter: ServerIpFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_server_ip WHERE deleted_at IS NULL");

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id { sql.push_str(&format!(" AND server_id = ${}", idx)); uuid_binds.push(sid); idx += 1; }
        if let Some(ref status) = filter.status { sql.push_str(&format!(" AND status::text = ${}", idx)); string_binds.push(status.clone()); idx += 1; }
        if let Some(ref ip_type) = filter.ip_type { sql.push_str(&format!(" AND ip_type = ${}", idx)); string_binds.push(ip_type.clone()); idx += 1; }
        if let Some(ref q) = filter.q { sql.push_str(&format!(" AND ip_address ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        let row = query.fetch_one(&self.pool).await.repo()?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, ip: &ServerIP) -> Result<ServerIP, RepositoryError> {
        let result = sqlx::query(
            r#"INSERT INTO cmdb_server_ip
               (id, server_id, ip_address, nic_name, ip_type, is_primary, isp_provider_id,
                description, status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               RETURNING id, server_id, ip_address, nic_name, ip_type, is_primary,
                         isp_provider_id, description, status::text, created_at, updated_at"#,
        )
        .bind(ip.id)
        .bind(ip.server_id)
        .bind(&ip.ip_address)
        .bind(&ip.nic_name)
        .bind(&ip.ip_type)
        .bind(ip.is_primary)
        .bind(ip.isp_provider_id)
        .bind(&ip.description)
        .bind(ip.status.clone())
        .bind(ip.created_at)
        .bind(ip.updated_at)
        .fetch_one(&self.pool)
        .await;
        match result {
            Ok(row) => Ok(row_to_server_ip(&row)),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(RepositoryError::Constraint(
                    "该 IP 地址已存在（可能已被删除，可在回收站处理）".to_string(),
                ))
            }
            Err(e) => Err(RepositoryError::Database(e.to_string())),
        }
    }

    async fn update(&self, id: Uuid, ip: &ServerIP) -> Result<Option<ServerIP>, RepositoryError> {
        let row = sqlx::query(
            r#"UPDATE cmdb_server_ip SET
                ip_address = $2, nic_name = $3, ip_type = $4, is_primary = $5, isp_provider_id = $6,
                description = $7, status = $8, updated_at = $9
               WHERE id = $1
               RETURNING id, server_id, ip_address, nic_name, ip_type, is_primary,
                         isp_provider_id, description, status::text, created_at, updated_at"#,
        )
        .bind(id)
        .bind(&ip.ip_address)
        .bind(&ip.nic_name)
        .bind(&ip.ip_type)
        .bind(ip.is_primary)
        .bind(ip.isp_provider_id)
        .bind(&ip.description)
        .bind(ip.status.clone())
        .bind(ip.updated_at)
        .fetch_optional(&self.pool)
        .await.repo()?;
        Ok(row.map(|r| row_to_server_ip(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("UPDATE cmdb_server_ip SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await.repo()?;
        Ok(result.rows_affected() > 0)
    }
}
