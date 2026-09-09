use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::change_record::ChangeRecord;
use rzops_domain::ports::audit_log_repository::AuditLogRepository;
use rzops_domain::ports::change_record_repository::ChangeRecordRepository;
use rzops_domain::ports::role_repository::RoleRepository;
use rzops_domain::ports::user_repository::UserRepository;
use rzops_domain::enums::ChangeType;

use crate::auth_extractor::AuthUser;
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::recycle_dto::*;

/// Shared state for recycle bin routes.
#[derive(Clone)]
pub struct RecycleState {
    pub pool: Pool<Postgres>,
    pub user_repo: Arc<dyn UserRepository>,
    pub role_repo: Arc<dyn RoleRepository>,
    pub change_repo: Arc<dyn ChangeRecordRepository>,
    pub audit_repo: Arc<dyn AuditLogRepository>,
}

/// 资源类型 → (表名, 名称列 SQL 表达式)
/// 仅包含有 deleted_at 且纳入回收站的表。
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

fn table_for(resource_type: &str) -> Option<(&'static str, &'static str)> {
    RESOURCE_TABLE
        .iter()
        .find(|(r, _, _)| *r == resource_type)
        .map(|(_, t, name_col)| (*t, *name_col))
}

/// GET /recycle — list soft-deleted items across resources.
#[utoipa::path(
    get,
    path = "/api/v1/recycle",
    params(
        ("resource_type" = Option<String>, Query),
        ("q" = Option<String>, Query),
        ("page" = Option<i64>, Query),
        ("per_page" = Option<i64>, Query),
    ),
    responses((status = 200, body = RecycleListResponse)),
    tag = "Recycle"
)]
pub async fn list_recycle(
    _auth: AuthUser,
    State(state): State<RecycleState>,
    Query(query): Query<ListRecycleQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // 构建各表 UNION 子查询
    let mut parts: Vec<String> = Vec::new();
    let mut filter_resources: Vec<&'static str> = Vec::new();
    for (rtype, table, name_col) in RESOURCE_TABLE {
        if let Some(filter) = query.resource_type.as_deref() {
            if filter != *rtype {
                continue;
            }
        }
        filter_resources.push(rtype);
        parts.push(format!(
            "SELECT '{}' AS resource_type, t.id, ({} )::text AS name, t.deleted_at, row_to_json(t) AS data FROM {} t WHERE t.deleted_at IS NOT NULL",
            rtype, name_col, table
        ));
    }
    if parts.is_empty() {
        return (StatusCode::OK, Json(RecycleListResponse { data: vec![], count: 0 })).into_response();
    }

    let mut sql = format!("SELECT resource_type, id, name, deleted_at, data FROM (\n{}\n) AS rc", parts.join("\nUNION ALL\n"));

    // 搜索过滤（按名称 / id 文本）
    let mut conds: Vec<String> = Vec::new();
    if let Some(q) = query.q.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let esc = q.replace('\'', "''");
        conds.push(format!("name ILIKE '%{}%'", esc));
    }
    if !conds.is_empty() {
        sql.push_str(&format!(" WHERE {}", conds.join(" AND ")));
    }

    // 排序：删除时间倒序
    sql.push_str(" ORDER BY deleted_at DESC");

    // 分页
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    match sqlx::query(&sql).fetch_all(&state.pool).await {
        Ok(rows) => {
            let data: Vec<RecycleItem> = rows
                .iter()
                .map(|r| RecycleItem {
                    resource_type: r.get("resource_type"),
                    id: r.get("id"),
                    name: r.get("name"),
                    deleted_at: r.get::<DateTime<Utc>, _>("deleted_at"),
                    data: r.get("data"),
                })
                .collect();

            // count
            let mut count_sql = format!(
                "SELECT COUNT(*) AS count FROM (\n{}\n) AS rc",
                parts.join("\nUNION ALL\n")
            );
            if !conds.is_empty() {
                count_sql.push_str(&format!(" WHERE {}", conds.join(" AND ")));
            }
            let count = match sqlx::query(&count_sql).fetch_one(&state.pool).await {
                Ok(r) => r.get::<i64, _>("count"),
                Err(_) => 0,
            };

            (StatusCode::OK, Json(RecycleListResponse { data, count })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// POST /recycle/restore — restore a soft-deleted item.
#[utoipa::path(
    post,
    path = "/api/v1/recycle/restore",
    request_body = RestoreRequest,
    responses((status = 200), (status = 400, body = ErrorResponse), (status = 404, body = ErrorResponse)),
    tag = "Recycle"
)]
pub async fn restore_item(
    _auth: AuthUser,
    State(state): State<RecycleState>,
    Json(body): Json<RestoreRequest>,
) -> impl IntoResponse {
    let (table, _) = match table_for(&body.resource_type) {
        Some(t) => t,
        None => return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: format!("unsupported resource type: {}", body.resource_type) })).into_response(),
    };
    // 恢复唯一约束冲突时需要从回收站删除原记录（记录冲突的具体资源）
    // user / role 软删除时被置为 is_active=false，恢复时一并恢复为启用状态
    let sql = if body.resource_type == "user" || body.resource_type == "role" {
        format!("UPDATE {} SET deleted_at = NULL, is_active = true, updated_at = now() WHERE id = $1 AND deleted_at IS NOT NULL", table)
    } else {
        format!("UPDATE {} SET deleted_at = NULL, updated_at = now() WHERE id = $1 AND deleted_at IS NOT NULL", table)
    };
    match sqlx::query(&sql).bind(body.id).execute(&state.pool).await {
        Ok(result) if result.rows_affected() > 0 => {
            (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
        }
        Ok(_) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "item not found in recycle bin".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("restore failed: {}", e) })).into_response(),
    }
}

/// DELETE /recycle/:resource_type/:id — permanently delete an item.
#[utoipa::path(
    delete,
    path = "/api/v1/recycle/{resource_type}/{id}",
    params(
        ("resource_type" = String, Path, description = "Resource type"),
        ("id" = Uuid, Path, description = "Item ID"),
    ),
    responses((status = 200), (status = 400, body = ErrorResponse), (status = 404, body = ErrorResponse)),
    tag = "Recycle"
)]
pub async fn purge_item(
    auth: AuthUser,
    State(state): State<RecycleState>,
    axum::extract::Path((resource_type, id)): axum::extract::Path<(String, Uuid)>,
) -> impl IntoResponse {
    let (table, name_col) = match table_for(&resource_type) {
        Some(t) => t,
        None => return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: format!("unsupported resource type: {}", resource_type) })).into_response(),
    };

    // 尝试取资源名称（用于日志；失败不影响主流程）
    let name_sql = format!("SELECT {} AS name FROM {} WHERE id = $1 AND deleted_at IS NOT NULL", name_col, table);
    let resource_name: Option<String> = sqlx::query_scalar(&name_sql).bind(id).fetch_optional(&state.pool).await.ok().flatten();

    let sql = format!("DELETE FROM {} WHERE id = $1 AND deleted_at IS NOT NULL", table);
    match sqlx::query(&sql).bind(id).execute(&state.pool).await {
        Ok(result) if result.rows_affected() > 0 => {
            // 永久删除：记录变更日志（purge）与审计日志（purge），与软删除（delete）区分
            let change = ChangeRecord {
                id: Uuid::new_v4(),
                actor_id: Some(auth.user_id),
                change_type: ChangeType::Purge,
                resource_type: resource_type.clone(),
                resource_id: Some(id),
                before_data: serde_json::json!({ "name": resource_name }),
                after_data: serde_json::json!(null),
                remarks: None,
                created_at: chrono::Utc::now(),
            };
            if let Err(e) = state.change_repo.create(&change).await {
                tracing::error!("failed to record purge change for {}: {}", resource_type, e);
            }
            // 审计日志由审计中间件统一记录（action=purge），此处不再重复写入
            (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
        }
        Ok(_) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "item not found in recycle bin".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("delete failed: {}", e) })).into_response(),
    }
}
