use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::models::backup_plan::BackupPlan;
use rzops_domain::ports::backup_plan_repository::{BackupPlanFilter, BackupPlanRepository};

pub struct PgBackupPlanRepository { pool: Pool<Postgres> }
impl PgBackupPlanRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }


fn row_to_entity(row: &sqlx::postgres::PgRow) -> BackupPlan {
    BackupPlan { id: row.get("id"), name: row.get("name"), target_type: row.get("target_type"), target_id: row.get("target_id"), schedule: row.get("schedule"), retention_days: row.get("retention_days"), status: row.get::<String, _>("status"), remarks: row.get("remarks"), created_at: row.get::<DateTime<Utc>, _>("created_at"), updated_at: row.get::<DateTime<Utc>, _>("updated_at") }
}

const COLS: &str = "id, name, target_type, target_id, schedule, retention_days, status::text, remarks, created_at, updated_at";

#[async_trait]
impl BackupPlanRepository for PgBackupPlanRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BackupPlan>, RepositoryError> {
        Ok(sqlx::query(&format!("SELECT {} FROM cmdb_backup_plan WHERE id=$1 AND deleted_at IS NULL", COLS)).bind(id).fetch_optional(&self.pool).await.repo()?.map(|r| row_to_entity(&r)))
    }
    async fn find_all(&self, f: BackupPlanFilter) -> Result<Vec<BackupPlan>, RepositoryError> {
        let mut sql = format!("SELECT {} FROM cmdb_backup_plan WHERE deleted_at IS NULL", COLS);
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_q = f.q.as_ref();
        let s_tt = f.target_type.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_tt.is_some() { sql.push_str(&format!(" AND target_type = ${}", idx)); idx += 1; }
        if f.target_id.is_some() { sql.push_str(&format!(" AND target_id = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        sql.push_str(" ORDER BY CASE WHEN status::text IN ('disabled', 'paused', 'inactive') THEN 2 WHEN status::text = 'archived' THEN 1 ELSE 0 END, created_at DESC");
        if let Some(l) = f.limit { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = f.offset { sql.push_str(&format!(" OFFSET {}", o)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(t) = s_tt { query = query.bind(t); }
        if let Some(tid) = f.target_id { query = query.bind(tid); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_all(&self.pool).await.repo()?.iter().map(row_to_entity).collect())
    }
    async fn count(&self, f: BackupPlanFilter) -> Result<i64, RepositoryError> {
        let mut sql = "SELECT COUNT(*) as count FROM cmdb_backup_plan WHERE deleted_at IS NULL".to_string();
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_q = f.q.as_ref();
        let s_tt = f.target_type.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_tt.is_some() { sql.push_str(&format!(" AND target_type = ${}", idx)); idx += 1; }
        if f.target_id.is_some() { sql.push_str(&format!(" AND target_id = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(t) = s_tt { query = query.bind(t); }
        if let Some(tid) = f.target_id { query = query.bind(tid); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_one(&self.pool).await.repo()?.get::<i64, _>("count"))
    }
    async fn create(&self, e: &BackupPlan) -> Result<BackupPlan, RepositoryError> {
        Ok(row_to_entity(&sqlx::query(&format!("INSERT INTO cmdb_backup_plan (id,name,target_type,target_id,schedule,retention_days,status,remarks,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING {}", COLS))
            .bind(e.id).bind(&e.name).bind(&e.target_type).bind(e.target_id).bind(&e.schedule).bind(e.retention_days).bind(e.status.clone()).bind(&e.remarks).bind(e.created_at).bind(e.updated_at)
            .fetch_one(&self.pool).await.repo()?))
    }
    async fn update(&self, id: Uuid, e: &BackupPlan) -> Result<Option<BackupPlan>, RepositoryError> {
        Ok(sqlx::query(&format!("UPDATE cmdb_backup_plan SET name=$2,target_type=$3,target_id=$4,schedule=$5,retention_days=$6,status=$7,remarks=$8,updated_at=$9 WHERE id=$1 RETURNING {}", COLS))
            .bind(id).bind(&e.name).bind(&e.target_type).bind(e.target_id).bind(&e.schedule).bind(e.retention_days).bind(e.status.clone()).bind(&e.remarks).bind(e.updated_at)
            .fetch_optional(&self.pool).await.repo()?.map(|r| row_to_entity(&r)))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        Ok(sqlx::query("UPDATE cmdb_backup_plan SET deleted_at = now() WHERE id=$1 AND deleted_at IS NULL").bind(id).execute(&self.pool).await.repo()?.rows_affected() > 0)
    }
}
