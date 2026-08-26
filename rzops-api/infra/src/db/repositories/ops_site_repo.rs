use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::enums::{CodeRepoType, Importance, ServiceTarget, SiteStatus, WebFramework};
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

fn parse_site_status(s: &str) -> SiteStatus {
    match s {
        "active" => SiteStatus::Active,
        "temporary_offline" => SiteStatus::TemporaryOffline,
        "permanent_offline" => SiteStatus::PermanentOffline,
        _ => SiteStatus::Active,
    }
}

fn site_status_to_string(s: &SiteStatus) -> String {
    match s {
        SiteStatus::Active => "active".to_string(),
        SiteStatus::TemporaryOffline => "temporary_offline".to_string(),
        SiteStatus::PermanentOffline => "permanent_offline".to_string(),
    }
}

fn parse_importance(s: &str) -> Importance {
    match s {
        "critical" => Importance::Critical,
        "high" => Importance::High,
        "medium" => Importance::Medium,
        "low" => Importance::Low,
        _ => Importance::Medium,
    }
}

fn parse_service_target(s: &str) -> ServiceTarget {
    match s {
        "internal" => ServiceTarget::Internal,
        "external" => ServiceTarget::External,
        "partner" => ServiceTarget::Partner,
        "mixed" => ServiceTarget::Mixed,
        _ => ServiceTarget::Internal,
    }
}

fn parse_code_repo_type(s: &str) -> CodeRepoType {
    match s {
        "svn" => CodeRepoType::Svn,
        "git" => CodeRepoType::Git,
        "none" => CodeRepoType::None,
        _ => CodeRepoType::Other,
    }
}

fn parse_web_framework(s: &str) -> WebFramework {
    match s {
        "django" => WebFramework::Django,
        "flask" => WebFramework::Flask,
        "fastapi" => WebFramework::Fastapi,
        "spring_boot" => WebFramework::SpringBoot,
        "express" => WebFramework::Express,
        "rails" => WebFramework::Rails,
        "laravel" => WebFramework::Laravel,
        "asp_net_mvc" => WebFramework::AspNetMvc,
        "asp_net_core" => WebFramework::AspNetCore,
        "gin" => WebFramework::Gin,
        "echo" => WebFramework::Echo,
        "nextjs" => WebFramework::Nextjs,
        "nuxtjs" => WebFramework::Nuxtjs,
        "ant_design_pro" => WebFramework::AntDesignPro,
        _ => WebFramework::Other,
    }
}

fn row_to_ops_site(row: &sqlx::postgres::PgRow) -> OpsSite {
    let status_str: String = row.get("status");
    let st_str: Option<String> = row.get("service_target");
    let imp_str: Option<String> = row.get("importance");
    let crt_str: Option<String> = row.get("code_repo_type");
    let wf_str: Option<String> = row.get("web_framework");

    OpsSite {
        id: row.get("id"),
        name: row.get("name"),
        url: row.get("url"),
        business_unit_id: row.get("business_unit_id"),
        department_id: row.get("department_id"),
        service_target: st_str.map(|s| parse_service_target(&s)),
        importance: imp_str.map(|s| parse_importance(&s)),
        online_time: row.get("online_time"),
        code_repo_type: crt_str.map(|s| parse_code_repo_type(&s)),
        code_repo_url: row.get("code_repo_url"),
        purpose: row.get("purpose"),
        is_internal_system: row.get("is_internal_system"),
        language_runtime: row.get("language_runtime"),
        web_framework: wf_str.map(|s| parse_web_framework(&s)),
        uses_cdn: row.get("uses_cdn"),
        is_test_site: row.get("is_test_site"),
        backup_plan_id: row.get("backup_plan_id"),
        last_backup_time: row.get("last_backup_time"),
        monitor_target_id: row.get("monitor_target_id"),
        status: parse_site_status(&status_str),
        offline_time: row.get("offline_time"),
        offline_reason: row.get("offline_reason"),
        function_summary: row.get("function_summary"),
        remarks: row.get("remarks"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

const SELECT_COLS: &str = r#"id, name, url, business_unit_id, department_id,
    service_target::text, importance::text, online_time, code_repo_type::text,
    code_repo_url, purpose, is_internal_system, language_runtime, web_framework::text,
    uses_cdn, is_test_site, backup_plan_id, last_backup_time, monitor_target_id,
    status::text, offline_time, offline_reason, function_summary, remarks,
    created_at, updated_at"#;

#[async_trait]
impl OpsSiteRepository for PgOpsSiteRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<OpsSite>, sqlx::Error> {
        let row = sqlx::query(&format!("SELECT {} FROM cmdb_ops_site WHERE id = $1", SELECT_COLS))
            .bind(id).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_ops_site(&r)))
    }

    async fn find_all(&self, filter: OpsSiteFilter) -> Result<Vec<OpsSite>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_ops_site WHERE 1=1", SELECT_COLS);
        let mut idx = 1;
        let s_status = filter.status.as_ref();
        let s_imp = filter.importance.as_ref();
        let s_q = filter.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_imp.is_some() { sql.push_str(&format!(" AND importance::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(limit) = filter.limit { sql.push_str(&format!(" LIMIT {}", limit)); }
        if let Some(offset) = filter.offset { sql.push_str(&format!(" OFFSET {}", offset)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(i) = s_imp { query = query.bind(i); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_ops_site(r)).collect())
    }

    async fn count(&self, filter: OpsSiteFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_ops_site WHERE 1=1");
        let mut idx = 1;
        let s_status = filter.status.as_ref();
        let s_imp = filter.importance.as_ref();
        let s_q = filter.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_imp.is_some() { sql.push_str(&format!(" AND importance::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(i) = s_imp { query = query.bind(i); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, s: &OpsSite) -> Result<OpsSite, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"INSERT INTO cmdb_ops_site
               (id, name, url, business_unit_id, department_id, service_target, importance,
                online_time, code_repo_type, code_repo_url, purpose, is_internal_system,
                language_runtime, web_framework, uses_cdn, is_test_site, backup_plan_id,
                last_backup_time, monitor_target_id, status, offline_time, offline_reason,
                function_summary, remarks, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26)
               RETURNING {}"#, SELECT_COLS
        ))
        .bind(s.id).bind(&s.name).bind(&s.url).bind(s.business_unit_id).bind(s.department_id)
        .bind(s.service_target.as_ref().map(|v| match v {
            ServiceTarget::Internal => "internal", ServiceTarget::External => "external",
            ServiceTarget::Partner => "partner", ServiceTarget::Mixed => "mixed",
        }.to_string()))
        .bind(s.importance.as_ref().map(|v| match v {
            Importance::Critical => "critical", Importance::High => "high",
            Importance::Medium => "medium", Importance::Low => "low",
        }.to_string()))
        .bind(s.online_time)
        .bind(s.code_repo_type.as_ref().map(|v| match v {
            CodeRepoType::Svn => "svn", CodeRepoType::Git => "git",
            CodeRepoType::None => "none", CodeRepoType::Other => "other",
        }.to_string()))
        .bind(&s.code_repo_url).bind(&s.purpose).bind(s.is_internal_system)
        .bind(&s.language_runtime)
        .bind(s.web_framework.as_ref().map(|v| match v {
            WebFramework::Django => "django", WebFramework::Flask => "flask",
            WebFramework::Fastapi => "fastapi", WebFramework::SpringBoot => "spring_boot",
            WebFramework::Express => "express", WebFramework::Rails => "rails",
            WebFramework::Laravel => "laravel", WebFramework::AspNetMvc => "asp_net_mvc",
            WebFramework::AspNetCore => "asp_net_core", WebFramework::Gin => "gin",
            WebFramework::Echo => "echo", WebFramework::Nextjs => "nextjs",
            WebFramework::Nuxtjs => "nuxtjs", WebFramework::AntDesignPro => "ant_design_pro",
            WebFramework::Other => "other",
        }.to_string()))
        .bind(s.uses_cdn).bind(s.is_test_site).bind(s.backup_plan_id).bind(s.last_backup_time)
        .bind(s.monitor_target_id).bind(site_status_to_string(&s.status))
        .bind(s.offline_time).bind(&s.offline_reason).bind(&s.function_summary).bind(&s.remarks)
        .bind(s.created_at).bind(s.updated_at)
        .fetch_one(&self.pool).await?;
        Ok(row_to_ops_site(&row))
    }

    async fn update(&self, id: Uuid, s: &OpsSite) -> Result<Option<OpsSite>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"UPDATE cmdb_ops_site SET
                name=$2, url=$3, business_unit_id=$4, department_id=$5, service_target=$6,
                importance=$7, online_time=$8, code_repo_type=$9, code_repo_url=$10,
                purpose=$11, is_internal_system=$12, language_runtime=$13, web_framework=$14,
                uses_cdn=$15, is_test_site=$16, backup_plan_id=$17, last_backup_time=$18,
                monitor_target_id=$19, status=$20, offline_time=$21, offline_reason=$22,
                function_summary=$23, remarks=$24, updated_at=$25
               WHERE id=$1 RETURNING {}"#, SELECT_COLS
        ))
        .bind(id).bind(&s.name).bind(&s.url).bind(s.business_unit_id).bind(s.department_id)
        .bind(s.service_target.as_ref().map(|v| match v {
            ServiceTarget::Internal => "internal", ServiceTarget::External => "external",
            ServiceTarget::Partner => "partner", ServiceTarget::Mixed => "mixed",
        }.to_string()))
        .bind(s.importance.as_ref().map(|v| match v {
            Importance::Critical => "critical", Importance::High => "high",
            Importance::Medium => "medium", Importance::Low => "low",
        }.to_string()))
        .bind(s.online_time)
        .bind(s.code_repo_type.as_ref().map(|v| match v {
            CodeRepoType::Svn => "svn", CodeRepoType::Git => "git",
            CodeRepoType::None => "none", CodeRepoType::Other => "other",
        }.to_string()))
        .bind(&s.code_repo_url).bind(&s.purpose).bind(s.is_internal_system)
        .bind(&s.language_runtime)
        .bind(s.web_framework.as_ref().map(|v| match v {
            WebFramework::Django => "django", WebFramework::Flask => "flask",
            WebFramework::Fastapi => "fastapi", WebFramework::SpringBoot => "spring_boot",
            WebFramework::Express => "express", WebFramework::Rails => "rails",
            WebFramework::Laravel => "laravel", WebFramework::AspNetMvc => "asp_net_mvc",
            WebFramework::AspNetCore => "asp_net_core", WebFramework::Gin => "gin",
            WebFramework::Echo => "echo", WebFramework::Nextjs => "nextjs",
            WebFramework::Nuxtjs => "nuxtjs", WebFramework::AntDesignPro => "ant_design_pro",
            WebFramework::Other => "other",
        }.to_string()))
        .bind(s.uses_cdn).bind(s.is_test_site).bind(s.backup_plan_id).bind(s.last_backup_time)
        .bind(s.monitor_target_id).bind(site_status_to_string(&s.status))
        .bind(s.offline_time).bind(&s.offline_reason).bind(&s.function_summary).bind(&s.remarks)
        .bind(s.updated_at)
        .fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_ops_site(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cmdb_ops_site WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }
}
