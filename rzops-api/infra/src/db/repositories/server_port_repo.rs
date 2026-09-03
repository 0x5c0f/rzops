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

/// 端口查询公共列（含多对多关联聚合的服务器 id / 名称）
const PORT_SELECT: &str = r#"
    SELECT p.id, p.protocol::text, p.port, p.service_name, p.access_scope,
           p.is_enabled, p.description, p.created_at, p.updated_at,
           COALESCE(array_agg(s.id) FILTER (WHERE s.id IS NOT NULL), '{}') AS server_ids,
           COALESCE(array_agg(s.name) FILTER (WHERE s.name IS NOT NULL), '{}') AS server_names,
           COALESCE(array_agg(s.status::text) FILTER (WHERE s.status IS NOT NULL), '{}') AS server_statuses
    FROM cmdb_server_port p
    LEFT JOIN cmdb_server_port_server ps ON ps.server_port_id = p.id
    LEFT JOIN cmdb_server s ON s.id = ps.server_id AND s.deleted_at IS NULL
"#;

fn row_to_server_port(row: &sqlx::postgres::PgRow) -> ServerPort {
    let protocol_str: String = row.get("protocol");
    ServerPort {
        id: row.get("id"),
        protocol: protocol_str,
        port: row.get("port"),
        service_name: row.get("service_name"),
        access_scope: row.get("access_scope"),
        is_enabled: row.get("is_enabled"),
        description: row.get("description"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        server_ids: row.get("server_ids"),
        server_names: row.get("server_names"),
        server_statuses: row.get("server_statuses"),
    }
}

#[async_trait]
impl ServerPortRepository for PgServerPortRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerPort>, sqlx::Error> {
        let sql = format!(
            r#"{} WHERE p.id = $1 GROUP BY p.id"#,
            PORT_SELECT
        );
        let row = sqlx::query(&sql).bind(id).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_server_port(&r)))
    }

    async fn find_all(&self, filter: ServerPortFilter) -> Result<Vec<ServerPort>, sqlx::Error> {
        let mut sql = format!(r#"{} WHERE 1=1"#, PORT_SELECT);

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id {
            sql.push_str(&format!(" AND EXISTS (SELECT 1 FROM cmdb_server_port_server ps2 WHERE ps2.server_port_id = p.id AND ps2.server_id = ${})", idx));
            uuid_binds.push(sid);
            idx += 1;
        }
        if let Some(ref protocol) = filter.protocol {
            sql.push_str(&format!(" AND p.protocol::text = ${}", idx));
            string_binds.push(protocol.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND p.service_name ILIKE ${}", idx));
            string_binds.push(format!("%{}%", q));
        }

        sql.push_str(" GROUP BY p.id ORDER BY CASE WHEN p.is_enabled = false THEN 2 WHEN NOT EXISTS (SELECT 1 FROM cmdb_server_port_server ps2 JOIN cmdb_server s2 ON s2.id = ps2.server_id WHERE ps2.server_port_id = p.id AND s2.status::text NOT IN ('retired', 'offline') AND s2.deleted_at IS NULL) AND EXISTS (SELECT 1 FROM cmdb_server_port_server ps3 WHERE ps3.server_port_id = p.id) THEN 1 ELSE 0 END, p.created_at DESC");
        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_server_port(r)).collect())
    }

    async fn count(&self, filter: ServerPortFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_server_port p WHERE 1=1");

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id {
            sql.push_str(&format!(" AND EXISTS (SELECT 1 FROM cmdb_server_port_server ps2 WHERE ps2.server_port_id = p.id AND ps2.server_id = ${})", idx));
            uuid_binds.push(sid);
            idx += 1;
        }
        if let Some(ref protocol) = filter.protocol {
            sql.push_str(&format!(" AND p.protocol::text = ${}", idx));
            string_binds.push(protocol.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND p.service_name ILIKE ${}", idx));
            string_binds.push(format!("%{}%", q));
        }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, port: &ServerPort) -> Result<ServerPort, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 端口定义按 (protocol, port, service_name) 复用：已存在则复用其 id，否则新建
        let existing: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM cmdb_server_port WHERE protocol = $1 AND port = $2 AND service_name = $3",
        )
        .bind(&port.protocol)
        .bind(port.port)
        .bind(&port.service_name)
        .fetch_optional(&mut *tx)
        .await?;

        let port_id = match existing {
            Some(id) => id,
            None => {
                sqlx::query(
                    r#"INSERT INTO cmdb_server_port
                       (id, protocol, port, service_name, access_scope,
                        is_enabled, description, created_at, updated_at)
                       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
                )
                .bind(port.id)
                .bind(port.protocol.clone())
                .bind(port.port)
                .bind(&port.service_name)
                .bind(&port.access_scope)
                .bind(port.is_enabled)
                .bind(&port.description)
                .bind(port.created_at)
                .bind(port.updated_at)
                .execute(&mut *tx)
                .await?;
                port.id
            }
        };

        for sid in &port.server_ids {
            sqlx::query(
                "INSERT INTO cmdb_server_port_server (server_port_id, server_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(port_id)
            .bind(sid)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        // 返回合并后的端口（含全部关联服务器）
        match self.find_by_id(port_id).await? {
            Some(p) => Ok(p),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    async fn update(&self, id: Uuid, port: &ServerPort) -> Result<Option<ServerPort>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query(
            r#"UPDATE cmdb_server_port SET
                protocol = $2, port = $3, service_name = $4, access_scope = $5,
                is_enabled = $6, description = $7, updated_at = $8
               WHERE id = $1"#,
        )
        .bind(id)
        .bind(port.protocol.clone())
        .bind(port.port)
        .bind(&port.service_name)
        .bind(&port.access_scope)
        .bind(port.is_enabled)
        .bind(&port.description)
        .bind(port.updated_at)
        .execute(&mut *tx)
        .await?;

        if row.rows_affected() == 0 {
            tx.rollback().await?;
            return Ok(None);
        }

        // 重建关联：先清空再按 server_ids 重新绑定
        sqlx::query("DELETE FROM cmdb_server_port_server WHERE server_port_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        for sid in &port.server_ids {
            sqlx::query(
                "INSERT INTO cmdb_server_port_server (server_port_id, server_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(id)
            .bind(sid)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(self.find_by_id(id).await?)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cmdb_server_port WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
