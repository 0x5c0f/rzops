use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::models::audit_log::AuditLog;
use rzops_domain::ports::audit_log_repository::{AuditLogFilter, AuditLogRepository};

pub struct PgAuditLogRepository { pool: Pool<Postgres> }
impl PgAuditLogRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }

fn row_to_entity(row: &sqlx::postgres::PgRow) -> AuditLog {
    AuditLog { id: row.get("id"), actor_id: row.get("actor_id"), action: row.get("action"), resource_type: row.get("resource_type"), resource_id: row.get("resource_id"), ip_address: row.get("ip_address"), user_agent: row.get("user_agent"), extra_data: row.get("extra_data"), created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

const COLS: &str = "id, actor_id, action, resource_type, resource_id, ip_address, user_agent, extra_data, created_at";

#[async_trait]
impl AuditLogRepository for PgAuditLogRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLog>, RepositoryError> {
        Ok(sqlx::query(&format!("SELECT {} FROM cmdb_audit_log WHERE id=$1", COLS)).bind(id).fetch_optional(&self.pool).await.repo()?.map(|r| row_to_entity(&r)))
    }
    async fn find_all(&self, f: AuditLogFilter) -> Result<Vec<AuditLog>, RepositoryError> {
        let mut sql = format!("SELECT {} FROM cmdb_audit_log WHERE 1=1", COLS);
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut dt_binds: Vec<DateTime<Utc>> = Vec::new();
        let mut idx = 1;
        if let Some(aid) = f.actor_id { sql.push_str(&format!(" AND actor_id = ${}", idx)); uuid_binds.push(aid); idx += 1; }
        if let Some(ref rt) = f.resource_type { sql.push_str(&format!(" AND resource_type = ${}", idx)); string_binds.push(rt.clone()); idx += 1; }
        if let Some(ref ac) = f.action { sql.push_str(&format!(" AND action = ${}", idx)); string_binds.push(ac.clone()); idx += 1; }
        if let Some(ref dt) = f.created_from { sql.push_str(&format!(" AND created_at >= ${}", idx)); dt_binds.push(*dt); idx += 1; }
        if let Some(ref dt) = f.created_to { sql.push_str(&format!(" AND created_at <= ${}", idx)); dt_binds.push(*dt); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(l) = f.limit { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = f.offset { sql.push_str(&format!(" OFFSET {}", o)); }
        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        for d in &dt_binds { query = query.bind(d); }
        Ok(query.fetch_all(&self.pool).await.repo()?.iter().map(row_to_entity).collect())
    }
    async fn count(&self, f: AuditLogFilter) -> Result<i64, RepositoryError> {
        let mut sql = "SELECT COUNT(*) as count FROM cmdb_audit_log WHERE 1=1".to_string();
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut dt_binds: Vec<DateTime<Utc>> = Vec::new();
        let mut idx = 1;
        if let Some(aid) = f.actor_id { sql.push_str(&format!(" AND actor_id = ${}", idx)); uuid_binds.push(aid); idx += 1; }
        if let Some(ref rt) = f.resource_type { sql.push_str(&format!(" AND resource_type = ${}", idx)); string_binds.push(rt.clone()); idx += 1; }
        if let Some(ref ac) = f.action { sql.push_str(&format!(" AND action = ${}", idx)); string_binds.push(ac.clone()); idx += 1; }
        if let Some(ref dt) = f.created_from { sql.push_str(&format!(" AND created_at >= ${}", idx)); dt_binds.push(*dt); idx += 1; }
        if let Some(ref dt) = f.created_to { sql.push_str(&format!(" AND created_at <= ${}", idx)); dt_binds.push(*dt); }
        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        for d in &dt_binds { query = query.bind(d); }
        Ok(query.fetch_one(&self.pool).await.repo()?.get::<i64, _>("count"))
    }
    async fn create(&self, e: &AuditLog) -> Result<AuditLog, RepositoryError> {
        Ok(row_to_entity(&sqlx::query(&format!("INSERT INTO cmdb_audit_log (id,actor_id,action,resource_type,resource_id,ip_address,user_agent,extra_data,created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING {}", COLS))
            .bind(e.id).bind(e.actor_id).bind(&e.action).bind(&e.resource_type).bind(e.resource_id).bind(&e.ip_address).bind(&e.user_agent).bind(&e.extra_data).bind(e.created_at)
            .fetch_one(&self.pool).await.repo()?))
    }
}
