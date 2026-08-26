use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::enums::{MonitorType, ReservedStatus};
use rzops_domain::models::monitor_target::MonitorTarget;
use rzops_domain::ports::monitor_target_repository::{MonitorTargetFilter, MonitorTargetRepository};

pub struct PgMonitorTargetRepository { pool: Pool<Postgres> }
impl PgMonitorTargetRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }

fn parse_monitor_type(s: &str) -> MonitorType { match s { "ping" => MonitorType::Ping, "http" => MonitorType::Http, "tcp" => MonitorType::Tcp, "tls" => MonitorType::Tls, _ => MonitorType::Custom } }
fn monitor_type_to_string(t: &MonitorType) -> String { match t { MonitorType::Ping => "ping", MonitorType::Http => "http", MonitorType::Tcp => "tcp", MonitorType::Tls => "tls", MonitorType::Custom => "custom" }.to_string() }
fn parse_status(s: &str) -> ReservedStatus { match s { "draft" => ReservedStatus::Draft, "active" => ReservedStatus::Active, "inactive" => ReservedStatus::Inactive, "archived" => ReservedStatus::Archived, _ => ReservedStatus::Draft } }
fn status_to_string(s: &ReservedStatus) -> String { match s { ReservedStatus::Draft => "draft", ReservedStatus::Active => "active", ReservedStatus::Inactive => "inactive", ReservedStatus::Archived => "archived" }.to_string() }

fn row_to_entity(row: &sqlx::postgres::PgRow) -> MonitorTarget {
    let mt_str: Option<String> = row.get("monitor_type");
    MonitorTarget { id: row.get("id"), name: row.get("name"), target_type: row.get("target_type"), target_id: row.get("target_id"), monitor_type: mt_str.map(|s| parse_monitor_type(&s)), endpoint: row.get("endpoint"), interval_seconds: row.get("interval_seconds"), status: parse_status(&row.get::<String, _>("status")), remarks: row.get("remarks"), created_at: row.get::<DateTime<Utc>, _>("created_at"), updated_at: row.get::<DateTime<Utc>, _>("updated_at") }
}

const COLS: &str = "id, name, target_type, target_id, monitor_type::text, endpoint, interval_seconds, status::text, remarks, created_at, updated_at";

#[async_trait]
impl MonitorTargetRepository for PgMonitorTargetRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MonitorTarget>, sqlx::Error> {
        Ok(sqlx::query(&format!("SELECT {} FROM cmdb_monitor_target WHERE id=$1", COLS)).bind(id).fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn find_all(&self, f: MonitorTargetFilter) -> Result<Vec<MonitorTarget>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_monitor_target WHERE 1=1", COLS);
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_q = f.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(l) = f.limit { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = f.offset { sql.push_str(&format!(" OFFSET {}", o)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_all(&self.pool).await?.iter().map(|r| row_to_entity(r)).collect())
    }
    async fn count(&self, f: MonitorTargetFilter) -> Result<i64, sqlx::Error> {
        let mut sql = "SELECT COUNT(*) as count FROM cmdb_monitor_target WHERE 1=1".to_string();
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_q = f.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_one(&self.pool).await?.get::<i64, _>("count"))
    }
    async fn create(&self, e: &MonitorTarget) -> Result<MonitorTarget, sqlx::Error> {
        Ok(row_to_entity(&sqlx::query(&format!("INSERT INTO cmdb_monitor_target (id,name,target_type,target_id,monitor_type,endpoint,interval_seconds,status,remarks,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) RETURNING {}", COLS))
            .bind(e.id).bind(&e.name).bind(&e.target_type).bind(e.target_id).bind(e.monitor_type.as_ref().map(monitor_type_to_string)).bind(&e.endpoint).bind(e.interval_seconds).bind(status_to_string(&e.status)).bind(&e.remarks).bind(e.created_at).bind(e.updated_at)
            .fetch_one(&self.pool).await?))
    }
    async fn update(&self, id: Uuid, e: &MonitorTarget) -> Result<Option<MonitorTarget>, sqlx::Error> {
        Ok(sqlx::query(&format!("UPDATE cmdb_monitor_target SET name=$2,target_type=$3,target_id=$4,monitor_type=$5,endpoint=$6,interval_seconds=$7,status=$8,remarks=$9,updated_at=$10 WHERE id=$1 RETURNING {}", COLS))
            .bind(id).bind(&e.name).bind(&e.target_type).bind(e.target_id).bind(e.monitor_type.as_ref().map(monitor_type_to_string)).bind(&e.endpoint).bind(e.interval_seconds).bind(status_to_string(&e.status)).bind(&e.remarks).bind(e.updated_at)
            .fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_monitor_target WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}
