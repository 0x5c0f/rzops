use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::enums::ChangeType;
use rzops_domain::models::change_record::ChangeRecord;
use rzops_domain::ports::change_record_repository::{ChangeRecordFilter, ChangeRecordRepository};

pub struct PgChangeRecordRepository { pool: Pool<Postgres> }
impl PgChangeRecordRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }

fn parse_change_type(s: &str) -> ChangeType { match s { "create" => ChangeType::Create, "update" => ChangeType::Update, "status_change" => ChangeType::StatusChange, "delete" => ChangeType::Delete, "bind" => ChangeType::Bind, "unbind" => ChangeType::Unbind, _ => ChangeType::Create } }
fn change_type_to_string(t: &ChangeType) -> String { match t { ChangeType::Create => "create", ChangeType::Update => "update", ChangeType::StatusChange => "status_change", ChangeType::Delete => "delete", ChangeType::Bind => "bind", ChangeType::Unbind => "unbind" }.to_string() }

fn row_to_entity(row: &sqlx::postgres::PgRow) -> ChangeRecord {
    ChangeRecord { id: row.get("id"), actor_id: row.get("actor_id"), change_type: parse_change_type(&row.get::<String, _>("change_type")), resource_type: row.get("resource_type"), resource_id: row.get("resource_id"), before_data: row.get("before_data"), after_data: row.get("after_data"), remarks: row.get("remarks"), created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

const COLS: &str = "id, actor_id, change_type::text, resource_type, resource_id, before_data, after_data, remarks, created_at";

#[async_trait]
impl ChangeRecordRepository for PgChangeRecordRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChangeRecord>, sqlx::Error> {
        Ok(sqlx::query(&format!("SELECT {} FROM cmdb_change_record WHERE id=$1", COLS)).bind(id).fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn find_all(&self, f: ChangeRecordFilter) -> Result<Vec<ChangeRecord>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_change_record WHERE 1=1", COLS);
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut dt_binds: Vec<DateTime<Utc>> = Vec::new();
        let mut idx = 1;
        if let Some(aid) = f.actor_id { sql.push_str(&format!(" AND actor_id = ${}", idx)); uuid_binds.push(aid); idx += 1; }
        if let Some(ref rt) = f.resource_type { sql.push_str(&format!(" AND resource_type = ${}", idx)); string_binds.push(rt.clone()); idx += 1; }
        if let Some(ref ct) = f.change_type { sql.push_str(&format!(" AND change_type = ${}", idx)); string_binds.push(ct.clone()); idx += 1; }
        if let Some(ref dt) = f.created_from { sql.push_str(&format!(" AND created_at >= ${}", idx)); dt_binds.push(dt.clone()); idx += 1; }
        if let Some(ref dt) = f.created_to { sql.push_str(&format!(" AND created_at <= ${}", idx)); dt_binds.push(dt.clone()); idx += 1; }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(l) = f.limit { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = f.offset { sql.push_str(&format!(" OFFSET {}", o)); }
        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        for d in &dt_binds { query = query.bind(d); }
        Ok(query.fetch_all(&self.pool).await?.iter().map(|r| row_to_entity(r)).collect())
    }
    async fn count(&self, f: ChangeRecordFilter) -> Result<i64, sqlx::Error> {
        let mut sql = "SELECT COUNT(*) as count FROM cmdb_change_record WHERE 1=1".to_string();
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut dt_binds: Vec<DateTime<Utc>> = Vec::new();
        let mut idx = 1;
        if let Some(aid) = f.actor_id { sql.push_str(&format!(" AND actor_id = ${}", idx)); uuid_binds.push(aid); idx += 1; }
        if let Some(ref rt) = f.resource_type { sql.push_str(&format!(" AND resource_type = ${}", idx)); string_binds.push(rt.clone()); idx += 1; }
        if let Some(ref ct) = f.change_type { sql.push_str(&format!(" AND change_type = ${}", idx)); string_binds.push(ct.clone()); idx += 1; }
        if let Some(ref dt) = f.created_from { sql.push_str(&format!(" AND created_at >= ${}", idx)); dt_binds.push(dt.clone()); idx += 1; }
        if let Some(ref dt) = f.created_to { sql.push_str(&format!(" AND created_at <= ${}", idx)); dt_binds.push(dt.clone()); idx += 1; }
        let mut query = sqlx::query(&sql);
        for u in &uuid_binds { query = query.bind(u); }
        for s in &string_binds { query = query.bind(s); }
        for d in &dt_binds { query = query.bind(d); }
        Ok(query.fetch_one(&self.pool).await?.get::<i64, _>("count"))
    }
    async fn create(&self, e: &ChangeRecord) -> Result<ChangeRecord, sqlx::Error> {
        Ok(row_to_entity(&sqlx::query(&format!("INSERT INTO cmdb_change_record (id,actor_id,change_type,resource_type,resource_id,before_data,after_data,remarks,created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING {}", COLS))
            .bind(e.id).bind(e.actor_id).bind(change_type_to_string(&e.change_type)).bind(&e.resource_type).bind(e.resource_id).bind(&e.before_data).bind(&e.after_data).bind(&e.remarks).bind(e.created_at)
            .fetch_one(&self.pool).await?))
    }
}
