use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::ops_site::OpsSite;
use rzops_domain::ports::ops_site_repository::{OpsSiteFilter, OpsSiteRepository};

pub struct PgOpsSiteRepository {
    pool: Pool<Postgres>,
}

impl PgOpsSiteRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn row_to_ops_site(row: &sqlx::postgres::PgRow) -> OpsSite {
    OpsSite {
        id: row.get("id"),
        name: row.get("name"),
        url: row.get("url"),
        service_target: row.get("service_target"),
        importance: row.get("importance"),
        online_time: row.get("online_time"),
        code_repo_type: row.get("code_repo_type"),
        code_repo_url: row.get("code_repo_url"),
        purpose: row.get("purpose"),
        language_runtime: row.get("language_runtime"),
        web_framework: row.get("web_framework"),
        is_test_site: row.get("is_test_site"),
        last_backup_time: row.get("last_backup_time"),
        status: row.get("status"),
        environment: row.get("environment"),
        offline_time: row.get("offline_time"),
        offline_reason: row.get("offline_reason"),
        function_summary: row.get("function_summary"),
        remarks: row.get("remarks"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

const SELECT_COLS: &str = r#"id, name, url, service_target, importance, online_time,
    code_repo_type, code_repo_url, purpose, language_runtime, web_framework,
    is_test_site, last_backup_time, status, environment, offline_time, offline_reason,
    function_summary, remarks, created_at, updated_at, deleted_at"#;

#[async_trait]
impl OpsSiteRepository for PgOpsSiteRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<OpsSite>, sqlx::Error> {
        let row = sqlx::query(&format!("SELECT {} FROM cmdb_ops_site WHERE id = $1 AND deleted_at IS NULL", SELECT_COLS))
            .bind(id).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_ops_site(&r)))
    }

    async fn find_all(&self, filter: OpsSiteFilter) -> Result<Vec<OpsSite>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_ops_site WHERE deleted_at IS NULL", SELECT_COLS);
        let mut idx = 1;
        let s_status = filter.status.as_ref();
        let s_env = filter.environment.as_ref();
        let s_imp = filter.importance.as_ref();
        let s_server = filter.server_id.as_ref();
        let s_q = filter.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_env.is_some() { sql.push_str(&format!(" AND environment = ${}", idx)); idx += 1; }
        if s_imp.is_some() { sql.push_str(&format!(" AND importance::text = ${}", idx)); idx += 1; }
        if s_server.is_some() { sql.push_str(&format!(" AND id IN (SELECT site_id FROM cmdb_ops_site_server WHERE server_id = ${})", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(limit) = filter.limit { sql.push_str(&format!(" LIMIT {}", limit)); }
        if let Some(offset) = filter.offset { sql.push_str(&format!(" OFFSET {}", offset)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(e) = s_env { query = query.bind(e); }
        if let Some(i) = s_imp { query = query.bind(i); }
        if let Some(srv) = s_server { query = query.bind(srv); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_ops_site(r)).collect())
    }

    async fn count(&self, filter: OpsSiteFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_ops_site WHERE deleted_at IS NULL");
        let mut idx = 1;
        let s_status = filter.status.as_ref();
        let s_env = filter.environment.as_ref();
        let s_imp = filter.importance.as_ref();
        let s_server = filter.server_id.as_ref();
        let s_q = filter.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_env.is_some() { sql.push_str(&format!(" AND environment = ${}", idx)); idx += 1; }
        if s_imp.is_some() { sql.push_str(&format!(" AND importance::text = ${}", idx)); idx += 1; }
        if s_server.is_some() { sql.push_str(&format!(" AND id IN (SELECT site_id FROM cmdb_ops_site_server WHERE server_id = ${})", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(e) = s_env { query = query.bind(e); }
        if let Some(i) = s_imp { query = query.bind(i); }
        if let Some(srv) = s_server { query = query.bind(srv); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, s: &OpsSite) -> Result<OpsSite, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"INSERT INTO cmdb_ops_site
               (id, name, url, service_target, importance, online_time, code_repo_type,
                code_repo_url, purpose, language_runtime, web_framework, is_test_site,
                last_backup_time, status, environment, offline_time, offline_reason,
                function_summary, remarks, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21)
               RETURNING {}"#, SELECT_COLS
        ))
        .bind(s.id).bind(&s.name).bind(&s.url)
        .bind(s.service_target.clone())
        .bind(s.importance.clone())
        .bind(s.online_time)
        .bind(s.code_repo_type.clone())
        .bind(&s.code_repo_url).bind(&s.purpose)
        .bind(&s.language_runtime)
        .bind(s.web_framework.clone())
        .bind(s.is_test_site).bind(s.last_backup_time)
        .bind(s.status.clone())
        .bind(&s.environment)
        .bind(s.offline_time).bind(&s.offline_reason).bind(&s.function_summary).bind(&s.remarks)
        .bind(s.created_at).bind(s.updated_at)
        .fetch_one(&self.pool).await?;
        Ok(row_to_ops_site(&row))
    }

    async fn update(&self, id: Uuid, s: &OpsSite) -> Result<Option<OpsSite>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"UPDATE cmdb_ops_site SET
                name=$2, url=$3, service_target=$4, importance=$5, online_time=$6,
                code_repo_type=$7, code_repo_url=$8, purpose=$9, language_runtime=$10,
                web_framework=$11, is_test_site=$12, last_backup_time=$13,
                status=$14, environment=$15, offline_time=$16, offline_reason=$17,
                function_summary=$18, remarks=$19
               WHERE id=$1 AND deleted_at IS NULL RETURNING {}"#, SELECT_COLS
        ))
        .bind(id).bind(&s.name).bind(&s.url)
        .bind(s.service_target.clone())
        .bind(s.importance.clone())
        .bind(s.online_time)
        .bind(s.code_repo_type.clone())
        .bind(&s.code_repo_url).bind(&s.purpose)
        .bind(&s.language_runtime)
        .bind(s.web_framework.clone())
        .bind(s.is_test_site).bind(s.last_backup_time)
        .bind(s.status.clone())
        .bind(&s.environment)
        .bind(s.offline_time).bind(&s.offline_reason).bind(&s.function_summary).bind(&s.remarks)
        .fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_ops_site(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE cmdb_ops_site SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }
}
