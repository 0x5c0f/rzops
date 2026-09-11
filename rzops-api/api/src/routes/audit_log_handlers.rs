use std::sync::Arc;
use axum::{extract::{Path, Query, State, Extension}, http::StatusCode, response::IntoResponse, Json};
use rzops_domain::ports::resource_name_service::ResourceNameService;
use uuid::Uuid;
use rzops_domain::ports::audit_log_repository::{AuditLogFilter, AuditLogRepository};

use crate::dto::provider_dto::ErrorResponse;
use crate::dto::audit_log_dto::*;
use crate::auth_extractor::AuthUser;

fn to_resp(e: &rzops_domain::models::audit_log::AuditLog, actor_email: Option<String>, resource_name: Option<String>) -> AuditLogResponse {
    AuditLogResponse {
        id: e.id,
        actor_id: e.actor_id,
        action: e.action.clone(),
        resource_type: e.resource_type.clone(),
        resource_id: e.resource_id,
        ip_address: e.ip_address.clone(),
        user_agent: e.user_agent.clone(),
        actor_email,
        resource_name,
        extra_data: e.extra_data.clone(),
        created_at: e.created_at,
    }
}

#[utoipa::path(get, path = "/api/v1/audit-logs/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = AuditLogResponse), (status = 404, body = ErrorResponse)), tag = "AuditLog", security(("bearer_auth" = [])))]
pub async fn get_audit_log(
    auth: AuthUser,
    State(r): State<Arc<dyn AuditLogRepository>>,
    Extension(ns): Extension<Arc<dyn ResourceNameService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(resp) = auth.require_superuser() { return resp }
    match r.find_by_id(id).await {
        Ok(Some(e)) => {
            let emails = ns.resolve_user_emails(&[e.actor_id]).await;
            let email = e.actor_id.and_then(|aid| emails.get(&aid).cloned());
            let names = ns.resolve_resource_names(&[(e.resource_type.clone(), e.resource_id.unwrap_or_default())]).await;
            let name = e.resource_id.as_ref().and_then(|rid| names.get(&(e.resource_type.clone(), *rid)).cloned());
            (StatusCode::OK, Json(to_resp(&e, email, name))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "not found".into() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

#[utoipa::path(get, path = "/api/v1/audit-logs", params(ListAuditLogsQuery), responses((status = 200, body = AuditLogListResponse)), tag = "AuditLog", security(("bearer_auth" = [])))]
pub async fn list_audit_logs(
    auth: AuthUser,
    State(r): State<Arc<dyn AuditLogRepository>>,
    Extension(ns): Extension<Arc<dyn ResourceNameService>>,
    Query(q): Query<ListAuditLogsQuery>,
) -> impl IntoResponse {
    if let Err(resp) = auth.require_superuser() { return resp }
    let p = q.page.unwrap_or(1).max(1);
    let pp = q.per_page.unwrap_or(20).min(100);
    let f = AuditLogFilter { actor_id: q.actor_id, resource_type: q.resource_type, action: q.action, created_from: q.created_from, created_to: q.created_to, limit: Some(pp), offset: Some((p - 1) * pp) };
    match r.find_all(f.clone()).await {
        Ok(v) => {
            let c = r.count(f).await.unwrap_or(0);
            // 批量解析：一次查 user 邮箱 + 按类型批量查资源名
            let actor_ids: Vec<Option<Uuid>> = v.iter().map(|e| e.actor_id).collect();
            let emails = ns.resolve_user_emails(&actor_ids).await;
            let name_items: Vec<(String, Uuid)> = v.iter()
                .filter_map(|e| e.resource_id.map(|rid| (e.resource_type.clone(), rid)))
                .collect();
            let names = ns.resolve_resource_names(&name_items).await;
            let mut data = Vec::with_capacity(v.len());
            for e in &v {
                let email = e.actor_id.and_then(|aid| emails.get(&aid).cloned());
                let name = e.resource_id.as_ref().and_then(|rid| names.get(&(e.resource_type.clone(), *rid)).cloned());
                data.push(to_resp(e, email, name));
            }
            (StatusCode::OK, Json(AuditLogListResponse { data, count: c })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}
