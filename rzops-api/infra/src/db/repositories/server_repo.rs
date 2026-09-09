use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::server::Server;
use rzops_domain::ports::server_repository::{ServerFilter, ServerRepository};

/// PostgreSQL implementation of ServerRepository.
pub struct PgServerRepository {
    pool: Pool<Postgres>,
}

impl PgServerRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

// ── JSONB ↔ Vec<String> helpers ──

fn json_to_strings(val: serde_json::Value) -> Vec<String> {
    match val {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    }
}

fn strings_to_json(vals: &[String]) -> serde_json::Value {
    serde_json::Value::Array(
        vals.iter()
            .map(|s| serde_json::Value::String(s.clone()))
            .collect(),
    )
}

// ── Row mapper ──

fn row_to_server(row: &sqlx::postgres::PgRow) -> Server {
    let status_str: String = row.get("status");
    let hosting_type_str: Option<String> = row.get("hosting_type");
    let server_type_str: Option<String> = row.get("server_type");
    let role_tags_json: serde_json::Value = row.get("role_tags");
    let web_server_type_json: serde_json::Value = row.get("web_server_type");
    let price_val: Option<Decimal> = row.get("price");

    Server {
        id: row.get("id"),
        asset_code: row.get("asset_code"),
        name: row.get("name"),
        primary_ip: row.get("primary_ip"),
        location: row.get("location"),
        isp_provider_id: row.get("isp_provider_id"),
        data_center_id: row.get("data_center_id"),
        hosting_type: hosting_type_str,
        is_dual_line: row.get("is_dual_line"),
        lease_start_date: row.get("lease_start_date"),
        lease_end_date: row.get("lease_end_date"),
        price: price_val,
        price_currency: row.get("price_currency"),
        server_type: server_type_str,
        role_tags: json_to_strings(role_tags_json),
        is_database_server: row.get("is_database_server"),
        cpu: row.get("cpu"),
        memory_gb: row.get("memory_gb"),
        is_raid: row.get("is_raid"),
        raid_level: row.get("raid_level"),
        disk_layout: row.get("disk_layout"),
        hardware_config: row.get("hardware_config"),
        architecture: row.get("architecture"),
        maintainer_id: row.get("maintainer_id"),
        brand: row.get("brand"),
        warranty_info: row.get("warranty_info"),
        operating_system: row.get("operating_system"),
        web_server_type: json_to_strings(web_server_type_json),
        server_provider_id: row.get("server_provider_id"),
        software_provider_id: row.get("software_provider_id"),
        status: status_str,
        environment: row.get("environment"),
        offline_time: row.get("offline_time"),
        offline_reason: row.get("offline_reason"),
        remarks: row.get("remarks"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

const SELECT_COLS: &str = r#"id, asset_code, name, primary_ip, location,
    isp_provider_id, data_center_id, hosting_type::text, is_dual_line,
    lease_start_date, lease_end_date, price, price_currency,
    server_type::text, role_tags, is_database_server,
    cpu, memory_gb, is_raid, raid_level, disk_layout, hardware_config,
    architecture, maintainer_id, brand, warranty_info, operating_system,
    web_server_type, server_provider_id, software_provider_id,
    status::text, environment, offline_time, offline_reason, remarks, created_at, updated_at, deleted_at"#;

#[async_trait]
impl ServerRepository for PgServerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Server>, RepositoryError> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM cmdb_server WHERE id = $1 AND deleted_at IS NULL",
            SELECT_COLS
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await.repo()?;

        Ok(row.map(|r| row_to_server(&r)))
    }

    async fn find_all(&self, filter: ServerFilter) -> Result<Vec<Server>, RepositoryError> {
        let mut sql = format!("SELECT {} FROM cmdb_server WHERE deleted_at IS NULL", SELECT_COLS);
        let mut idx = 1;

        // Store filter values to bind in the same order as $N placeholders
        let status_val = filter.status.as_ref();
        let environment_val = filter.environment.as_ref();
        let dc_id_val = filter.data_center_id;
        let server_type_val = filter.server_type.as_ref();
        let is_db_val = filter.is_database_server;
        let q_val = filter.q.as_ref();

        if status_val.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if environment_val.is_some() { sql.push_str(&format!(" AND environment = ${}", idx)); idx += 1; }
        if dc_id_val.is_some() { sql.push_str(&format!(" AND data_center_id = ${}", idx)); idx += 1; }
        if server_type_val.is_some() { sql.push_str(&format!(" AND server_type::text = ${}", idx)); idx += 1; }
        if is_db_val.is_some() { sql.push_str(&format!(" AND is_database_server = ${}", idx)); idx += 1; }
        if q_val.is_some() { sql.push_str(&format!(" AND (name ILIKE ${idx} OR primary_ip ILIKE ${idx} OR asset_code ILIKE ${idx})", idx = idx)); }

        sql.push_str(" ORDER BY CASE WHEN status::text = 'retired' THEN 1 ELSE 0 END, created_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        // Bind in the same order as $N placeholders
        let mut query = sqlx::query(&sql);
        if let Some(s) = status_val { query = query.bind(s); }
        if let Some(e) = environment_val { query = query.bind(e); }
        if let Some(dc) = dc_id_val { query = query.bind(dc); }
        if let Some(st) = server_type_val { query = query.bind(st); }
        if let Some(is_db) = is_db_val { query = query.bind(is_db); }
        if let Some(q) = q_val { query = query.bind(format!("%{}%", q)); }

        let rows = query.fetch_all(&self.pool).await.repo()?;
        Ok(rows.iter().map(|r| row_to_server(r)).collect())
    }

    async fn count(&self, filter: ServerFilter) -> Result<i64, RepositoryError> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_server WHERE deleted_at IS NULL");
        let mut idx = 1;

        // Store filter values to bind in the same order as $N placeholders
        let status_val = filter.status.as_ref();
        let environment_val = filter.environment.as_ref();
        let dc_id_val = filter.data_center_id;
        let server_type_val = filter.server_type.as_ref();
        let is_db_val = filter.is_database_server;
        let q_val = filter.q.as_ref();

        if status_val.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if environment_val.is_some() { sql.push_str(&format!(" AND environment = ${}", idx)); idx += 1; }
        if dc_id_val.is_some() { sql.push_str(&format!(" AND data_center_id = ${}", idx)); idx += 1; }
        if server_type_val.is_some() { sql.push_str(&format!(" AND server_type::text = ${}", idx)); idx += 1; }
        if is_db_val.is_some() { sql.push_str(&format!(" AND is_database_server = ${}", idx)); idx += 1; }
        if q_val.is_some() { sql.push_str(&format!(" AND (name ILIKE ${idx} OR primary_ip ILIKE ${idx} OR asset_code ILIKE ${idx})", idx = idx)); }

        // Bind in the same order as $N placeholders
        let mut query = sqlx::query(&sql);
        if let Some(s) = status_val { query = query.bind(s); }
        if let Some(e) = environment_val { query = query.bind(e); }
        if let Some(dc) = dc_id_val { query = query.bind(dc); }
        if let Some(st) = server_type_val { query = query.bind(st); }
        if let Some(is_db) = is_db_val { query = query.bind(is_db); }
        if let Some(q) = q_val { query = query.bind(format!("%{}%", q)); }

        let row = query.fetch_one(&self.pool).await.repo()?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, server: &Server) -> Result<Server, RepositoryError> {
        let row = sqlx::query(&format!(
            r#"INSERT INTO cmdb_server
               (id, asset_code, name, primary_ip, location, isp_provider_id, data_center_id,
                hosting_type, is_dual_line, lease_start_date, lease_end_date, price, price_currency,
                server_type, role_tags, is_database_server, cpu, memory_gb, is_raid, raid_level,
                disk_layout, hardware_config, architecture, maintainer_id, brand, warranty_info,
                operating_system, web_server_type, server_provider_id, software_provider_id,
                status, environment, offline_time, offline_reason, remarks, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,
                       $21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36,$37)
               RETURNING {}"#,
            SELECT_COLS
        ))
        .bind(server.id)
        .bind(&server.asset_code)
        .bind(&server.name)
        .bind(&server.primary_ip)
        .bind(&server.location)
        .bind(server.isp_provider_id)
        .bind(server.data_center_id)
        .bind(server.hosting_type.clone())
        .bind(server.is_dual_line)
        .bind(server.lease_start_date)
        .bind(server.lease_end_date)
        .bind(server.price)
        .bind(&server.price_currency)
        .bind(server.server_type.clone())
        .bind(strings_to_json(&server.role_tags))
        .bind(server.is_database_server)
        .bind(&server.cpu)
        .bind(server.memory_gb)
        .bind(server.is_raid)
        .bind(&server.raid_level)
        .bind(&server.disk_layout)
        .bind(&server.hardware_config)
        .bind(&server.architecture)
        .bind(server.maintainer_id)
        .bind(&server.brand)
        .bind(&server.warranty_info)
        .bind(&server.operating_system)
        .bind(strings_to_json(&server.web_server_type))
        .bind(server.server_provider_id)
        .bind(server.software_provider_id)
        .bind(server.status.clone())
        .bind(&server.environment)
        .bind(server.offline_time)
        .bind(&server.offline_reason)
        .bind(&server.remarks)
        .bind(server.created_at)
        .bind(server.updated_at)
        .fetch_one(&self.pool)
        .await.repo()?;

        Ok(row_to_server(&row))
    }

    async fn update(&self, id: Uuid, server: &Server) -> Result<Option<Server>, RepositoryError> {
        let row = sqlx::query(&format!(
            r#"UPDATE cmdb_server SET
                asset_code = $2, name = $3, primary_ip = $4, location = $5,
                isp_provider_id = $6, data_center_id = $7, hosting_type = $8,
                is_dual_line = $9, lease_start_date = $10, lease_end_date = $11,
                price = $12, price_currency = $13, server_type = $14,
                role_tags = $15, is_database_server = $16, cpu = $17, memory_gb = $18,
                is_raid = $19, raid_level = $20, disk_layout = $21, hardware_config = $22,
                architecture = $23, maintainer_id = $24, brand = $25, warranty_info = $26,
                operating_system = $27, web_server_type = $28, server_provider_id = $29,
                software_provider_id = $30, status = $31, environment = $32, offline_time = $33,
                offline_reason = $34, remarks = $35, updated_at = $36
               WHERE id = $1 AND deleted_at IS NULL
               RETURNING {}"#,
            SELECT_COLS
        ))
        .bind(id)
        .bind(&server.asset_code)
        .bind(&server.name)
        .bind(&server.primary_ip)
        .bind(&server.location)
        .bind(server.isp_provider_id)
        .bind(server.data_center_id)
        .bind(server.hosting_type.clone())
        .bind(server.is_dual_line)
        .bind(server.lease_start_date)
        .bind(server.lease_end_date)
        .bind(server.price)
        .bind(&server.price_currency)
        .bind(server.server_type.clone())
        .bind(strings_to_json(&server.role_tags))
        .bind(server.is_database_server)
        .bind(&server.cpu)
        .bind(server.memory_gb)
        .bind(server.is_raid)
        .bind(&server.raid_level)
        .bind(&server.disk_layout)
        .bind(&server.hardware_config)
        .bind(&server.architecture)
        .bind(server.maintainer_id)
        .bind(&server.brand)
        .bind(&server.warranty_info)
        .bind(&server.operating_system)
        .bind(strings_to_json(&server.web_server_type))
        .bind(server.server_provider_id)
        .bind(server.software_provider_id)
        .bind(server.status.clone())
        .bind(&server.environment)
        .bind(server.offline_time)
        .bind(&server.offline_reason)
        .bind(&server.remarks)
        .bind(server.updated_at)
        .fetch_optional(&self.pool)
        .await.repo()?;

        Ok(row.map(|r| row_to_server(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query("UPDATE cmdb_server SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .execute(&self.pool)
            .await.repo()?;

        Ok(result.rows_affected() > 0)
    }
}

impl PgServerRepository {
    /// Helper for find_all when data_center_id filter is present (mixed bind types).
    async fn find_all_with_uuid_filter(&self, filter: ServerFilter) -> Result<Vec<Server>, RepositoryError> {
        let mut sql = format!("SELECT {} FROM cmdb_server WHERE deleted_at IS NULL", SELECT_COLS);
        let mut idx = 1;

        // Store filter values to bind in the same order as $N placeholders
        let status_val = filter.status.as_ref();
        let environment_val = filter.environment.as_ref();
        let dc_id_val = filter.data_center_id;
        let server_type_val = filter.server_type.as_ref();
        let is_db_val = filter.is_database_server;
        let q_val = filter.q.as_ref();

        if status_val.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if environment_val.is_some() { sql.push_str(&format!(" AND environment = ${}", idx)); idx += 1; }
        if dc_id_val.is_some() { sql.push_str(&format!(" AND data_center_id = ${}", idx)); idx += 1; }
        if server_type_val.is_some() { sql.push_str(&format!(" AND server_type::text = ${}", idx)); idx += 1; }
        if is_db_val.is_some() { sql.push_str(&format!(" AND is_database_server = ${}", idx)); idx += 1; }
        if q_val.is_some() { sql.push_str(&format!(" AND (name ILIKE ${idx} OR primary_ip ILIKE ${idx} OR asset_code ILIKE ${idx})", idx = idx)); }

        sql.push_str(" ORDER BY CASE WHEN status::text = 'retired' THEN 1 ELSE 0 END, created_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        // Bind in the same order as $N placeholders
        let mut query = sqlx::query(&sql);
        if let Some(s) = status_val { query = query.bind(s); }
        if let Some(e) = environment_val { query = query.bind(e); }
        if let Some(dc) = dc_id_val { query = query.bind(dc); }
        if let Some(st) = server_type_val { query = query.bind(st); }
        if let Some(is_db) = is_db_val { query = query.bind(is_db); }
        if let Some(q) = q_val { query = query.bind(format!("%{}%", q)); }

        let rows = query.fetch_all(&self.pool).await.repo()?;
        Ok(rows.iter().map(|r| row_to_server(r)).collect())
    }
}
