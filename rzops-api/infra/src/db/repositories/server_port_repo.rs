use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::server_port::ServerPort;
use rzops_domain::ports::server_port_repository::{ServerPortFilter, ServerPortRepository};

pub struct PgServerPortRepository {
    pool: Pool<Postgres>,
}

impl PgServerPortRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

/// 端口查询公共列（一对多：每台服务器独立端口记录，聚合所属服务器名称/状态）
/// sort_rank：0=正常 1=服务器退役/删除（离线） 2=端口禁用（用于排序，避免相关子查询）
const PORT_SELECT: &str = r#"
    SELECT p.id, p.server_id, p.protocol::text, p.port, p.service_name,
           p.access_scope, p.is_enabled, p.description, p.created_at, p.updated_at,
           s.name AS server_name, s.status::text AS server_status,
           CASE WHEN p.is_enabled = false THEN 2
                WHEN (s.status::text IN ('retired', 'offline')) OR (s.id IS NULL) THEN 1
                ELSE 0 END AS sort_rank
    FROM cmdb_server_port p
    LEFT JOIN cmdb_server s ON s.id = p.server_id AND s.deleted_at IS NULL
"#;

fn row_to_server_port(row: &sqlx::postgres::PgRow) -> ServerPort {
    let protocol_str: String = row.get("protocol");
    ServerPort {
        id: row.get("id"),
        server_id: row.get("server_id"),
        protocol: protocol_str,
        port: row.get("port"),
        service_name: row.get("service_name"),
        access_scope: row.get("access_scope"),
        is_enabled: row.get("is_enabled"),
        description: row.get("description"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        server_name: row.get("server_name"),
        server_status: row.get("server_status"),
    }
}

#[async_trait]
impl ServerPortRepository for PgServerPortRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerPort>, RepositoryError> {
        let sql = format!(r#"{} WHERE p.id = $1"#, PORT_SELECT);
        let row = sqlx::query(&sql).bind(id).fetch_optional(&self.pool).await.repo()?;
        Ok(row.map(|r| row_to_server_port(&r)))
    }

    async fn find_all(&self, filter: ServerPortFilter) -> Result<Vec<ServerPort>, RepositoryError> {
        let mut sql = format!(r#"{} WHERE 1=1"#, PORT_SELECT);

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id {
            sql.push_str(&format!(" AND p.server_id = ${}", idx));
            uuid_binds.push(sid);
            idx += 1;
        }
        if let Some(ref protocol) = filter.protocol {
            sql.push_str(&format!(" AND p.protocol::text = ${}", idx));
            string_binds.push(protocol.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND (p.service_name ILIKE ${} OR p.port::text ILIKE ${})", idx, idx));
            string_binds.push(format!("%{}%", q));
        }

        sql.push_str(" ORDER BY sort_rank, p.created_at DESC");
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
        Ok(rows.iter().map(row_to_server_port).collect())
    }

    async fn count(&self, filter: ServerPortFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_server_port p WHERE 1=1");

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id {
            sql.push_str(&format!(" AND p.server_id = ${}", idx));
            uuid_binds.push(sid);
            idx += 1;
        }
        if let Some(ref protocol) = filter.protocol {
            sql.push_str(&format!(" AND p.protocol::text = ${}", idx));
            string_binds.push(protocol.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND (p.service_name ILIKE ${} OR p.port::text ILIKE ${})", idx, idx));
            string_binds.push(format!("%{}%", q));
        }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        let row = query.fetch_one(&self.pool).await.repo()?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, port: &ServerPort) -> Result<ServerPort, RepositoryError> {
        sqlx::query(
            r#"INSERT INTO cmdb_server_port
               (id, server_id, protocol, port, service_name, access_scope,
                is_enabled, description, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        )
        .bind(port.id)
        .bind(port.server_id)
        .bind(port.protocol.clone())
        .bind(port.port)
        .bind(&port.service_name)
        .bind(&port.access_scope)
        .bind(port.is_enabled)
        .bind(&port.description)
        .bind(port.created_at)
        .bind(port.updated_at)
        .execute(&self.pool)
        .await.repo()?;

        match self.find_by_id(port.id).await? {
            Some(p) => Ok(p),
            None => Err(RepositoryError::NotFound("server port not found".into())),
        }
    }

    async fn update(&self, id: Uuid, port: &ServerPort) -> Result<Option<ServerPort>, RepositoryError> {
        let row = sqlx::query(
            r#"UPDATE cmdb_server_port SET
                server_id = $2, protocol = $3, port = $4, service_name = $5,
                access_scope = $6, is_enabled = $7, description = $8, updated_at = $9
               WHERE id = $1"#,
        )
        .bind(id)
        .bind(port.server_id)
        .bind(port.protocol.clone())
        .bind(port.port)
        .bind(&port.service_name)
        .bind(&port.access_scope)
        .bind(port.is_enabled)
        .bind(&port.description)
        .bind(port.updated_at)
        .execute(&self.pool)
        .await.repo()?;

        if row.rows_affected() == 0 {
            return Ok(None);
        }
        Ok(self.find_by_id(id).await?)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("DELETE FROM cmdb_server_port WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await.repo()?;
        Ok(result.rows_affected() > 0)
    }
}
