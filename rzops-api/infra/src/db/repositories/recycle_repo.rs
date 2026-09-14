use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::errors::RepositoryError;
use rzops_domain::models::recycle::RecycleEntry;
use rzops_domain::ports::recycle_repository::RecycleRepository;

use crate::db::IntoRepoResult;

/// 资源类型 → (表名, 名称列 SQL 表达式)。
/// 仅包含有 deleted_at 且纳入回收站的表。白名单常量——不允许用户输入进入表名/列名。
const RESOURCE_TABLE: &[(&str, &str, &str)] = &[
    ("server", "cmdb_server", "name"),
    ("datacenter", "cmdb_data_center", "name"),
    ("provider", "cmdb_provider", "name"),
    ("domain", "cmdb_domain", "domain_name"),
    ("certificate", "cmdb_certificate", "name"),
    ("server_ip", "cmdb_server_ip", "ip_address"),
    ("server_port", "cmdb_server_port", "port::text || ' (' || COALESCE(service_name,'') || ')'"),
    ("server_port_template", "cmdb_server_port_template", "name"),
    ("ops_site", "cmdb_ops_site", "name"),
    ("database_instance", "cmdb_database_instance", "instance_name"),
    ("backup_plan", "cmdb_backup_plan", "name"),
    ("monitor_target", "cmdb_monitor_target", "name"),
    ("attachment", "cmdb_attachment", "filename"),
    ("contract", "cmdb_contract", "name"),
    ("dict", "cmdb_dict", "dict_label"),
    ("user", "\"user\"", "email"),
    ("role", "role", "name"),
];

/// 回收站仓储（PostgreSQL 实现）。
pub struct PgRecycleRepository {
    pool: Pool<Postgres>,
}

impl PgRecycleRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn table_for(resource_type: &str) -> Option<(&'static str, &'static str)> {
    RESOURCE_TABLE
        .iter()
        .find(|(r, _, _)| *r == resource_type)
        .map(|(_, t, name_col)| (*t, *name_col))
}

#[async_trait]
impl RecycleRepository for PgRecycleRepository {
    async fn list(
        &self,
        resource_type: Option<&str>,
        q: Option<&str>,
        per_page: i64,
        offset: i64,
    ) -> Result<(Vec<RecycleEntry>, i64), RepositoryError> {
        // 构建各表 UNION 子查询
        let mut parts: Vec<String> = Vec::new();
        for (rtype, table, name_col) in RESOURCE_TABLE {
            if let Some(filter) = resource_type {
                if filter != *rtype {
                    continue;
                }
            }
            parts.push(format!(
                "SELECT '{}' AS resource_type, t.id, COALESCE(({} )::text, '') AS name, t.deleted_at, row_to_json(t) AS data FROM {} t WHERE t.deleted_at IS NOT NULL",
                rtype, name_col, table
            ));
        }
        if parts.is_empty() {
            return Ok((Vec::new(), 0));
        }

        let mut sql = format!("SELECT resource_type, id, name, deleted_at, data FROM (\n{}\n) AS rc", parts.join("\nUNION ALL\n"));

        // 搜索过滤（按名称 / id 文本）——参数化绑定，杜绝 SQL 注入
        let mut conds: Vec<String> = Vec::new();
        let mut binds: Vec<String> = Vec::new();
        if let Some(q) = q.map(|s| s.trim()).filter(|s| !s.is_empty()) {
            conds.push(format!("name ILIKE ${}", binds.len() + 1));
            binds.push(format!("%{}%", q));
        }
        if !conds.is_empty() {
            sql.push_str(&format!(" WHERE {}", conds.join(" AND ")));
        }

        // 排序：删除时间倒序
        sql.push_str(" ORDER BY deleted_at DESC");

        // 分页（参数化）
        sql.push_str(&format!(
            " LIMIT ${} OFFSET ${}",
            binds.len() + 1,
            binds.len() + 2
        ));

        let mut qry = sqlx::query(&sql);
        for v in &binds {
            qry = qry.bind(v);
        }
        let qry = qry.bind(per_page).bind(offset);

        let rows = qry.fetch_all(&self.pool).await.repo()?;
        let data: Vec<RecycleEntry> = rows
            .iter()
            .map(|r| RecycleEntry {
                resource_type: r.get("resource_type"),
                id: r.get("id"),
                name: r.get("name"),
                deleted_at: r.get::<DateTime<Utc>, _>("deleted_at"),
                data: r.get("data"),
            })
            .collect();

        // count（搜索条件参数化）
        let mut count_sql = format!(
            "SELECT COUNT(*) AS count FROM (\n{}\n) AS rc",
            parts.join("\nUNION ALL\n")
        );
        if !conds.is_empty() {
            count_sql.push_str(&format!(" WHERE {}", conds.join(" AND ")));
        }
        let mut count_qry = sqlx::query(&count_sql);
        for v in &binds {
            count_qry = count_qry.bind(v);
        }
        let count = match count_qry.fetch_one(&self.pool).await {
            Ok(r) => r.get::<i64, _>("count"),
            Err(_) => 0,
        };

        Ok((data, count))
    }

    async fn restore(&self, resource_type: &str, id: Uuid) -> Result<bool, RepositoryError> {
        let (table, _) = match table_for(resource_type) {
            Some(t) => t,
            None => return Ok(false),
        };
        // 恢复唯一约束冲突时需要从回收站删除原记录（记录冲突的具体资源）
        // user / role 软删除时被置为 is_active=false，恢复时一并恢复为启用状态
        let sql = if resource_type == "user" || resource_type == "role" {
            format!("UPDATE {} SET deleted_at = NULL, is_active = true, updated_at = now() WHERE id = $1 AND deleted_at IS NOT NULL", table)
        } else {
            format!("UPDATE {} SET deleted_at = NULL, updated_at = now() WHERE id = $1 AND deleted_at IS NOT NULL", table)
        };
        let result = sqlx::query(&sql).bind(id).execute(&self.pool).await.repo()?;
        Ok(result.rows_affected() > 0)
    }

    async fn purge_name(&self, resource_type: &str, id: Uuid) -> Result<Option<String>, RepositoryError> {
        let (table, name_col) = match table_for(resource_type) {
            Some(t) => t,
            None => return Ok(None),
        };
        let name_sql = format!("SELECT {} AS name FROM {} WHERE id = $1 AND deleted_at IS NOT NULL", name_col, table);
        Ok(sqlx::query_scalar(&name_sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .repo()?)
    }

    async fn purge(&self, resource_type: &str, id: Uuid) -> Result<bool, RepositoryError> {
        let (table, _) = match table_for(resource_type) {
            Some(t) => t,
            None => return Ok(false),
        };
        let sql = format!("DELETE FROM {} WHERE id = $1 AND deleted_at IS NOT NULL", table);
        let result = sqlx::query(&sql).bind(id).execute(&self.pool).await.repo()?;
        Ok(result.rows_affected() > 0)
    }
}
