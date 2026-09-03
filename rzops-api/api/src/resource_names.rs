//! 资源名批量解析（Batch Resource Name Resolver）。
//!
//! 审计日志 / 变更记录列表页需要把 `resource_type + resource_id` 展示为可读的资源名。
//! 旧实现是逐条发详情请求（N+1），本模块提供按资源类型分组的批量查询：
//! 一页 20 条即使涉及 10 种资源类型，也只需 10 次 `WHERE id = ANY(...)` 查询。
//! 软删除的资源（deleted_at 非空）也一并解析（历史日志仍需显示名称）。

use std::collections::{HashMap, HashSet};

use sqlx::PgPool;
use uuid::Uuid;

/// 各资源类型：(类型名, 名称字段, 表名)。
const RESOURCE_TYPES: &[(&str, &str, &str)] = &[
    ("provider", "name", "cmdb_provider"),
    ("datacenter", "name", "cmdb_data_center"),
    ("server", "name", "cmdb_server"),
    ("server_ip", "ip_address", "cmdb_server_ip"),
    ("server_port", "service_name", "cmdb_server_port"),
    ("domain", "name", "cmdb_domain"),
    ("certificate", "name", "cmdb_certificate"),
    ("database_instance", "name", "cmdb_database_instance"),
    ("ops_site", "name", "cmdb_ops_site"),
    ("backup_plan", "name", "cmdb_backup_plan"),
    ("monitor_target", "name", "cmdb_monitor_target"),
    ("contract", "name", "cmdb_contract"),
    ("attachment", "filename", "cmdb_attachment"),
];

/// 批量解析资源名。返回 `(type, id) -> name` 映射；无法解析或名称缺失的键不出现。
pub async fn resolve_resource_names(
    pool: &PgPool,
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
            if let Some((t, _, _)) = RESOURCE_TYPES.iter().find(|(nt, _, _)| *nt == ty.as_str()) {
                grouped.entry(*t).or_default().push(*id);
            }
        }
    }

    for (ty, ids) in grouped {
        let (_, field, table) = RESOURCE_TYPES
            .iter()
            .find(|(nt, _, _)| *nt == ty)
            .expect("known resource type");
        // 分批，避免 ANY 数组参数过多
        for chunk in ids.chunks(200) {
            let sql = format!(
                "SELECT id::text AS id, {}::text AS label FROM {} WHERE id = ANY($1)",
                field, table
            );
            let ids_vec: Vec<Uuid> = chunk.to_vec();
            if let Ok(rows) = sqlx::query_as::<_, (String, String)>(&sql)
                .bind(&ids_vec)
                .fetch_all(pool)
                .await
            {
                for (id_str, label) in rows {
                    if let Ok(uid) = Uuid::parse_str(&id_str) {
                        if !label.is_empty() {
                            result.insert((ty.to_string(), uid), label);
                        }
                    }
                }
            }
        }
    }

    result
}

/// 批量解析服务器简要信息（名称 + 状态）。返回 `id -> (name, status)`。
/// 用于数据库实例 / 服务器IP 等关联服务器的响应，避免前端全量拉取服务器列表做映射。
pub async fn resolve_server_briefs(
    pool: &PgPool,
    ids: &[Option<Uuid>],
) -> HashMap<Uuid, (String, String)> {
    let mut result = HashMap::new();
    let uniq: Vec<Uuid> = {
        let mut seen = HashSet::new();
        ids.iter().flatten().filter(|id| seen.insert(**id)).copied().collect()
    };
    if uniq.is_empty() {
        return result;
    }
    let sql = "SELECT id::text AS id, name, status::text AS status FROM cmdb_server WHERE id = ANY($1)";
    match sqlx::query_as::<_, (String, String, String)>(sql)
        .bind(&uniq)
        .fetch_all(pool)
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
