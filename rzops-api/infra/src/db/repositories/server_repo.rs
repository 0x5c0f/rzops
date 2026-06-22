use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::enums::{HostingType, ServerRole, ServerStatus, ServerType, WebServerSoftware};
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

// ── Enum serialization helpers ──

fn parse_server_status(s: &str) -> ServerStatus {
    match s {
        "active" => ServerStatus::Active,
        "retired" => ServerStatus::Retired,
        _ => ServerStatus::Active,
    }
}

fn server_status_to_string(s: &ServerStatus) -> String {
    match s {
        ServerStatus::Active => "active".to_string(),
        ServerStatus::Retired => "retired".to_string(),
    }
}

fn parse_hosting_type(s: &str) -> HostingType {
    match s {
        "colocation" => HostingType::Colocation,
        "rental" => HostingType::Rental,
        "cloud" => HostingType::Cloud,
        "self_owned" => HostingType::SelfOwned,
        _ => HostingType::Other,
    }
}

fn hosting_type_to_string(ht: &HostingType) -> String {
    match ht {
        HostingType::Colocation => "colocation".to_string(),
        HostingType::Rental => "rental".to_string(),
        HostingType::Cloud => "cloud".to_string(),
        HostingType::SelfOwned => "self_owned".to_string(),
        HostingType::Other => "other".to_string(),
    }
}

fn parse_server_type(s: &str) -> ServerType {
    match s {
        "physical" => ServerType::Physical,
        "virtual" => ServerType::Virtual,
        "cloud" => ServerType::Cloud,
        "container" => ServerType::Container,
        _ => ServerType::Other,
    }
}

fn server_type_to_string(st: &ServerType) -> String {
    match st {
        ServerType::Physical => "physical".to_string(),
        ServerType::Virtual => "virtual".to_string(),
        ServerType::Cloud => "cloud".to_string(),
        ServerType::Container => "container".to_string(),
        ServerType::Other => "other".to_string(),
    }
}

fn parse_server_role(s: &str) -> ServerRole {
    match s {
        "web" => ServerRole::Web,
        "db" => ServerRole::Db,
        "cache" => ServerRole::Cache,
        "worker" => ServerRole::Worker,
        "file" => ServerRole::File,
        "monitor" => ServerRole::Monitor,
        "backup" => ServerRole::Backup,
        _ => ServerRole::Other,
    }
}

fn server_role_to_string(r: &ServerRole) -> String {
    match r {
        ServerRole::Web => "web".to_string(),
        ServerRole::Db => "db".to_string(),
        ServerRole::Cache => "cache".to_string(),
        ServerRole::Worker => "worker".to_string(),
        ServerRole::File => "file".to_string(),
        ServerRole::Monitor => "monitor".to_string(),
        ServerRole::Backup => "backup".to_string(),
        ServerRole::Other => "other".to_string(),
    }
}

fn parse_web_server_software(s: &str) -> WebServerSoftware {
    match s {
        "nginx" => WebServerSoftware::Nginx,
        "apache" => WebServerSoftware::Apache,
        "iis" => WebServerSoftware::Iis,
        "openresty" => WebServerSoftware::OpenResty,
        "caddy" => WebServerSoftware::Caddy,
        "traefik" => WebServerSoftware::Traefik,
        "tomcat" => WebServerSoftware::Tomcat,
        _ => WebServerSoftware::Other,
    }
}

fn web_server_software_to_string(w: &WebServerSoftware) -> String {
    match w {
        WebServerSoftware::Nginx => "nginx".to_string(),
        WebServerSoftware::Apache => "apache".to_string(),
        WebServerSoftware::Iis => "iis".to_string(),
        WebServerSoftware::OpenResty => "openresty".to_string(),
        WebServerSoftware::Caddy => "caddy".to_string(),
        WebServerSoftware::Traefik => "traefik".to_string(),
        WebServerSoftware::Tomcat => "tomcat".to_string(),
        WebServerSoftware::Other => "other".to_string(),
    }
}

// ── JSONB ↔ Vec helpers ──

fn json_to_server_roles(val: serde_json::Value) -> Vec<ServerRole> {
    match val {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(parse_server_role))
            .collect(),
        _ => Vec::new(),
    }
}

fn json_to_web_server_types(val: serde_json::Value) -> Vec<WebServerSoftware> {
    match val {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(parse_web_server_software))
            .collect(),
        _ => Vec::new(),
    }
}

fn roles_to_json(roles: &[ServerRole]) -> serde_json::Value {
    serde_json::Value::Array(
        roles
            .iter()
            .map(|r| serde_json::Value::String(server_role_to_string(r)))
            .collect(),
    )
}

fn web_types_to_json(types: &[WebServerSoftware]) -> serde_json::Value {
    serde_json::Value::Array(
        types
            .iter()
            .map(|w| serde_json::Value::String(web_server_software_to_string(w)))
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
        hosting_type: hosting_type_str.map(|s| parse_hosting_type(&s)),
        is_dual_line: row.get("is_dual_line"),
        lease_start_date: row.get("lease_start_date"),
        lease_end_date: row.get("lease_end_date"),
        price: price_val,
        price_currency: row.get("price_currency"),
        server_type: server_type_str.map(|s| parse_server_type(&s)),
        role_tags: json_to_server_roles(role_tags_json),
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
        web_server_type: json_to_web_server_types(web_server_type_json),
        server_provider_id: row.get("server_provider_id"),
        software_provider_id: row.get("software_provider_id"),
        status: parse_server_status(&status_str),
        offline_time: row.get("offline_time"),
        offline_reason: row.get("offline_reason"),
        remarks: row.get("remarks"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

const SELECT_COLS: &str = r#"id, asset_code, name, primary_ip, location,
    isp_provider_id, data_center_id, hosting_type::text, is_dual_line,
    lease_start_date, lease_end_date, price, price_currency,
    server_type::text, role_tags, is_database_server,
    cpu, memory_gb, is_raid, raid_level, disk_layout, hardware_config,
    architecture, maintainer_id, brand, warranty_info, operating_system,
    web_server_type, server_provider_id, software_provider_id,
    status::text, offline_time, offline_reason, remarks, created_at, updated_at"#;

#[async_trait]
impl ServerRepository for PgServerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Server>, sqlx::Error> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM cmdb_server WHERE id = $1",
            SELECT_COLS
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| row_to_server(&r)))
    }

    async fn find_all(&self, filter: ServerFilter) -> Result<Vec<Server>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_server WHERE 1=1", SELECT_COLS);
        let mut binds: Vec<String> = Vec::new();
        let mut idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status::text = ${}", idx));
            binds.push(status.clone());
            idx += 1;
        }
        if let Some(_dc_id) = filter.data_center_id {
            sql.push_str(&format!(" AND data_center_id = ${}", idx));
            // bind separately below
            let _ = idx;
            // We can't push Uuid into Vec<String>, so use a different approach
            // Actually, let's restructure to handle mixed types
            return self.find_all_with_uuid_filter(filter).await;
        }
        if let Some(ref server_type) = filter.server_type {
            sql.push_str(&format!(" AND server_type::text = ${}", idx));
            binds.push(server_type.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", idx));
            binds.push(format!("%{}%", q));
            // idx += 1;
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query(&sql);
        for bind in &binds {
            query = query.bind(bind);
        }

        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_server(r)).collect())
    }

    async fn count(&self, filter: ServerFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_server WHERE 1=1");
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status::text = ${}", idx));
            string_binds.push(status.clone());
            idx += 1;
        }
        if let Some(dc_id) = filter.data_center_id {
            sql.push_str(&format!(" AND data_center_id = ${}", idx));
            uuid_binds.push(dc_id);
            idx += 1;
        }
        if let Some(ref server_type) = filter.server_type {
            sql.push_str(&format!(" AND server_type::text = ${}", idx));
            string_binds.push(server_type.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", idx));
            string_binds.push(format!("%{}%", q));
        }

        let mut query = sqlx::query(&sql);
        for s in &string_binds { query = query.bind(s); }
        for u in &uuid_binds { query = query.bind(u); }

        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, server: &Server) -> Result<Server, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"INSERT INTO cmdb_server
               (id, asset_code, name, primary_ip, location, isp_provider_id, data_center_id,
                hosting_type, is_dual_line, lease_start_date, lease_end_date, price, price_currency,
                server_type, role_tags, is_database_server, cpu, memory_gb, is_raid, raid_level,
                disk_layout, hardware_config, architecture, maintainer_id, brand, warranty_info,
                operating_system, web_server_type, server_provider_id, software_provider_id,
                status, offline_time, offline_reason, remarks, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,
                       $21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36)
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
        .bind(server.hosting_type.as_ref().map(hosting_type_to_string))
        .bind(server.is_dual_line)
        .bind(server.lease_start_date)
        .bind(server.lease_end_date)
        .bind(server.price)
        .bind(&server.price_currency)
        .bind(server.server_type.as_ref().map(server_type_to_string))
        .bind(roles_to_json(&server.role_tags))
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
        .bind(web_types_to_json(&server.web_server_type))
        .bind(server.server_provider_id)
        .bind(server.software_provider_id)
        .bind(server_status_to_string(&server.status))
        .bind(server.offline_time)
        .bind(&server.offline_reason)
        .bind(&server.remarks)
        .bind(server.created_at)
        .bind(server.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row_to_server(&row))
    }

    async fn update(&self, id: Uuid, server: &Server) -> Result<Option<Server>, sqlx::Error> {
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
                software_provider_id = $30, status = $31, offline_time = $32,
                offline_reason = $33, remarks = $34, updated_at = $35
               WHERE id = $1
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
        .bind(server.hosting_type.as_ref().map(hosting_type_to_string))
        .bind(server.is_dual_line)
        .bind(server.lease_start_date)
        .bind(server.lease_end_date)
        .bind(server.price)
        .bind(&server.price_currency)
        .bind(server.server_type.as_ref().map(server_type_to_string))
        .bind(roles_to_json(&server.role_tags))
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
        .bind(web_types_to_json(&server.web_server_type))
        .bind(server.server_provider_id)
        .bind(server.software_provider_id)
        .bind(server_status_to_string(&server.status))
        .bind(server.offline_time)
        .bind(&server.offline_reason)
        .bind(&server.remarks)
        .bind(server.updated_at)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| row_to_server(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cmdb_server WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

impl PgServerRepository {
    /// Helper for find_all when data_center_id filter is present (mixed bind types).
    async fn find_all_with_uuid_filter(&self, filter: ServerFilter) -> Result<Vec<Server>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_server WHERE 1=1", SELECT_COLS);
        let mut idx = 1;

        // Store filter values to bind in the same order as $N placeholders
        let status_val = filter.status.as_ref();
        let dc_id_val = filter.data_center_id;
        let server_type_val = filter.server_type.as_ref();
        let q_val = filter.q.as_ref();

        if status_val.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if dc_id_val.is_some() { sql.push_str(&format!(" AND data_center_id = ${}", idx)); idx += 1; }
        if server_type_val.is_some() { sql.push_str(&format!(" AND server_type::text = ${}", idx)); idx += 1; }
        if q_val.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        // Bind in the same order as $N placeholders
        let mut query = sqlx::query(&sql);
        if let Some(s) = status_val { query = query.bind(s); }
        if let Some(dc) = dc_id_val { query = query.bind(dc); }
        if let Some(st) = server_type_val { query = query.bind(st); }
        if let Some(q) = q_val { query = query.bind(format!("%{}%", q)); }

        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_server(r)).collect())
    }
}
