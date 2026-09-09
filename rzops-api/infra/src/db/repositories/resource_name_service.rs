use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use rzops_domain::ports::resource_name_service::ResourceNameService;

/// 各资源类型：(类型名, 名称字段, 表名)。
/// 白名单常量 —— 不允许任何用户输入进入表名/列名。
const RESOURCE_TYPES: &[(&str, &str, &str)] = &[
    ("provider", "name", "cmdb_provider"),
    ("datacenter", "name", "cmdb_data_center"),
    ("server", "name", "cmdb_server"),
    ("server_ip", "ip_address", "cmdb_server_ip"),
    ("server_port", "service_name", "cmdb_server_port"),
    ("domain", "domain_name", "cmdb_domain"),
    ("certificate", "name", "cmdb_certificate"),
    ("database_instance", "name", "cmdb_database_instance"),
    ("ops_site", "name", "cmdb_ops_site"),
    ("backup_plan", "name", "cmdb_backup_plan"),
    ("monitor_target", "name", "cmdb_monitor_target"),
    ("contract", "name", "cmdb_contract"),
    ("attachment", "filename", "cmdb_attachment"),
];

/// 关联目标名称解析白名单（backup_plan / monitor_target / attachment）。
const TARGET_TYPES: &[(&str, &str, &str)] = &[
    ("server", "name", "cmdb_server"),
    ("database", "name", "cmdb_database_instance"),
    ("site", "name", "cmdb_ops_site"),
    ("domain", "domain_name", "cmdb_domain"),
    ("certificate", "name", "cmdb_certificate"),
    ("provider", "name", "cmdb_provider"),
    ("data_center", "name", "cmdb_data_center"),
    ("monitor_target", "name", "cmdb_monitor_target"),
    ("backup_plan", "name", "cmdb_backup_plan"),
    ("contract", "name", "cmdb_contract"),
];

/// 统一资源名称解析服务（PostgreSQL 实现）。
pub struct PgResourceNameService {
    pool: Pool<Postgres>,
}

impl PgResourceNameService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResourceNameService for PgResourceNameService {
    async fn resolve_resource_names(
        &self,
        items: &[(String, Uuid)],
    ) -> HashMap<(String, Uuid), String> {
        let mut result = HashMap::new();
        if items.is_empty() {
            return result;
        }

        // 按资源类型分组并去重
        let mut grouped: HashMap<&'static str, Vec<Uuid>> = HashMap::new();
        let mut seen_keys: HashSet<(String, Uuid)> = HashSet::new();
        for (ty, id) in items {
            if seen_keys.insert((ty.clone(), *id)) {
                if let Some((t, _, _)) = RESOURCE_TYPES.iter().find(|(nt, _, _)| *nt == ty.as_str())
                {
                    grouped.entry(*t).or_default().push(*id);
                }
            }
        }

        for (ty, ids) in grouped {
            if let Some((_, field, table)) =
                RESOURCE_TYPES.iter().find(|(nt, _, _)| *nt == ty)
            {
                // 分批，避免 ANY 数组参数过多
                for chunk in ids.chunks(200) {
                    let sql = format!(
                        "SELECT id::text AS id, {}::text AS label FROM {} WHERE id = ANY($1)",
                        field, table
                    );
                    let ids_vec: Vec<Uuid> = chunk.to_vec();
                    match sqlx::query_as::<_, (String, String)>(&sql)
                        .bind(&ids_vec)
                        .fetch_all(&self.pool)
                        .await
                    {
                        Ok(rows) => {
                            for (id_str, label) in rows {
                                if let Ok(uid) = Uuid::parse_str(&id_str) {
                                    if !label.is_empty() {
                                        result.insert((ty.to_string(), uid), label);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!("resolve_resource_names query failed: {}", e);
                        }
                    }
                }
            }
        }

        result
    }

    async fn resolve_server_briefs(&self, ids: &[Option<Uuid>]) -> HashMap<Uuid, (String, String)> {
        let mut result = HashMap::new();
        let uniq: Vec<Uuid> = {
            let mut seen = HashSet::new();
            ids.iter()
                .flatten()
                .filter(|id| seen.insert(**id))
                .copied()
                .collect()
        };
        if uniq.is_empty() {
            return result;
        }
        let sql = "SELECT id::text AS id, name, status::text AS status FROM cmdb_server WHERE id = ANY($1)";
        match sqlx::query_as::<_, (String, String, String)>(sql)
            .bind(&uniq)
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => {
                for (id_str, name, status) in rows {
                    if let Ok(uid) = Uuid::parse_str(&id_str) {
                        result.insert(uid, (name, status));
                    }
                }
            }
            Err(e) => {
                tracing::warn!("resolve_server_briefs query failed: {}", e);
            }
        }
        result
    }

    async fn resolve_target_name(&self, target_type: &str, target_id: Uuid) -> Option<String> {
        let (table, col) = match TARGET_TYPES
            .iter()
            .find(|(t, _, _)| *t == target_type)
        {
            Some((_, col, table)) => (table, col),
            None => return None,
        };
        let sql = format!("SELECT {col} FROM {table} WHERE id = $1");
        sqlx::query_scalar::<_, String>(&sql)
            .bind(target_id)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
    }

    async fn resolve_user_name(&self, user_id: Uuid) -> Option<String> {
        sqlx::query_scalar::<_, String>("SELECT full_name FROM \"user\" WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()
    }

    async fn resolve_user_emails(&self, ids: &[Option<Uuid>]) -> HashMap<Uuid, String> {
        let mut map = HashMap::new();
        let uniq: Vec<Uuid> = {
            let mut seen = HashSet::new();
            ids.iter()
                .flatten()
                .filter(|id| seen.insert(**id))
                .copied()
                .collect()
        };
        if uniq.is_empty() {
            return map;
        }
        let sql = "SELECT id::text AS id, email FROM \"user\" WHERE id = ANY($1)";
        match sqlx::query_as::<_, (String, String)>(sql)
            .bind(&uniq)
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => {
                for (id_str, email) in rows {
                    if let Ok(uid) = Uuid::parse_str(&id_str) {
                        map.insert(uid, email);
                    }
                }
            }
            Err(e) => {
                tracing::warn!("resolve_user_emails query failed: {}", e);
            }
        }
        map
    }
}
