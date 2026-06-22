use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::enums::Protocol;
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

fn parse_protocol(s: &str) -> Protocol {
    match s {
        "tcp" => Protocol::Tcp,
        "udp" => Protocol::Udp,
        "http" => Protocol::Http,
        "https" => Protocol::Https,
        _ => Protocol::Tcp,
    }
}

fn protocol_to_string(p: &Protocol) -> String {
    match p {
        Protocol::Tcp => "tcp".to_string(),
        Protocol::Udp => "udp".to_string(),
        Protocol::Http => "http".to_string(),
        Protocol::Https => "https".to_string(),
    }
}

fn row_to_server_port(row: &sqlx::postgres::PgRow) -> ServerPort {
    let protocol_str: String = row.get("protocol");
    ServerPort {
        id: row.get("id"),
        server_id: row.get("server_id"),
        protocol: parse_protocol(&protocol_str),
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
impl ServerPortRepository for PgServerPortRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerPort>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT id, server_id, protocol::text, port, service_name,
                      access_scope, is_enabled, description, created_at, updated_at
               FROM cmdb_server_port WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| row_to_server_port(&r)))
    }

    async fn find_all(&self, filter: ServerPortFilter) -> Result<Vec<ServerPort>, sqlx::Error> {
        let mut sql = String::from(
            r#"SELECT id, server_id, protocol::text, port, service_name,
                      access_scope, is_enabled, description, created_at, updated_at
               FROM cmdb_server_port WHERE 1=1"#,
        );

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id { sql.push_str(&format!(" AND server_id = ${}", idx)); uuid_binds.push(sid); idx += 1; }
        if let Some(ref protocol) = filter.protocol { sql.push_str(&format!(" AND protocol::text = ${}", idx)); string_binds.push(protocol.clone()); idx += 1; }
        if let Some(ref q) = filter.q { sql.push_str(&format!(" AND service_name ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }

        sql.push_str(" ORDER BY created_at DESC");
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
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_server_port WHERE 1=1");

        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(sid) = filter.server_id { sql.push_str(&format!(" AND server_id = ${}", idx)); uuid_binds.push(sid); idx += 1; }
        if let Some(ref protocol) = filter.protocol { sql.push_str(&format!(" AND protocol::text = ${}", idx)); string_binds.push(protocol.clone()); idx += 1; }
        if let Some(ref q) = filter.q { sql.push_str(&format!(" AND service_name ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }

        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, port: &ServerPort) -> Result<ServerPort, sqlx::Error> {
        let row = sqlx::query(
            r#"INSERT INTO cmdb_server_port
               (id, server_id, protocol, port, service_name, access_scope,
                is_enabled, description, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
               RETURNING id, server_id, protocol::text, port, service_name,
                         access_scope, is_enabled, description, created_at, updated_at"#,
        )
        .bind(port.id)
        .bind(port.server_id)
        .bind(protocol_to_string(&port.protocol))
        .bind(port.port)
        .bind(&port.service_name)
        .bind(&port.access_scope)
        .bind(port.is_enabled)
        .bind(&port.description)
        .bind(port.created_at)
        .bind(port.updated_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(row_to_server_port(&row))
    }

    async fn update(&self, id: Uuid, port: &ServerPort) -> Result<Option<ServerPort>, sqlx::Error> {
        let row = sqlx::query(
            r#"UPDATE cmdb_server_port SET
                protocol = $2, port = $3, service_name = $4, access_scope = $5,
                is_enabled = $6, description = $7, updated_at = $8
               WHERE id = $1
               RETURNING id, server_id, protocol::text, port, service_name,
                         access_scope, is_enabled, description, created_at, updated_at"#,
        )
        .bind(id)
        .bind(protocol_to_string(&port.protocol))
        .bind(port.port)
        .bind(&port.service_name)
        .bind(&port.access_scope)
        .bind(port.is_enabled)
        .bind(&port.description)
        .bind(port.updated_at)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| row_to_server_port(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cmdb_server_port WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
