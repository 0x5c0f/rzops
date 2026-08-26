use std::sync::Arc;
use axum::{extract::{Path, Query, State, Extension}, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;
use rzops_domain::ports::audit_log_repository::{AuditLogFilter, AuditLogRepository};
use rzops_domain::ports::user_repository::UserRepository;
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::audit_log_dto::*;
use crate::auth_extractor::AuthUser;

fn to_resp(e: &rzops_domain::models::audit_log::AuditLog, actor_email: Option<String>) -> AuditLogResponse {
    AuditLogResponse {
        id: e.id,
        actor_id: e.actor_id,
        action: e.action.clone(),
        resource_type: e.resource_type.clone(),
        resource_id: e.resource_id,
        ip_address: e.ip_address.clone(),
        user_agent: e.user_agent.clone(),
        actor_email,
        extra_data: e.extra_data.clone(),
        created_at: e.created_at,
    }
}

async fn resolve_actor_email(
    user_repo: &Arc<dyn UserRepository>,
    id: Option<Uuid>,
) -> Option<String> {
    match id {
        Some(aid) => match user_repo.find_by_id(aid).await {
            Ok(Some(u)) => Some(u.email),
            _ => None,
        },
        None => None,
    }
}

#[utoipa::path(get, path = "/api/v1/audit-logs/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = AuditLogResponse), (status = 404, body = ErrorResponse)), tag = "AuditLog", security(("bearer_auth" = [])))]
pub async fn get_audit_log(
    auth: AuthUser,
    State(r): State<Arc<dyn AuditLogRepository>>,
    Extension(user_repo): Extension<Arc<dyn UserRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(resp) = auth.require_superuser() { return resp }
    match r.find_by_id(id).await {
        Ok(Some(e)) => {
            let email = resolve_actor_email(&user_repo, e.actor_id).await;
            (StatusCode::OK, Json(to_resp(&e, email))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "not found".into() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

#[utoipa::path(get, path = "/api/v1/audit-logs", params(ListAuditLogsQuery), responses((status = 200, body = AuditLogListResponse)), tag = "AuditLog", security(("bearer_auth" = [])))]
pub async fn list_audit_logs(
    auth: AuthUser,
    State(r): State<Arc<dyn AuditLogRepository>>,
    Extension(user_repo): Extension<Arc<dyn UserRepository>>,
    Query(q): Query<ListAuditLogsQuery>,
) -> impl IntoResponse {
    if let Err(resp) = auth.require_superuser() { return resp }
    let p = q.page.unwrap_or(1).max(1);
    let pp = q.per_page.unwrap_or(20).min(100);
    let f = AuditLogFilter { actor_id: q.actor_id, resource_type: q.resource_type, action: q.action, created_from: q.created_from, created_to: q.created_to, limit: Some(pp), offset: Some((p - 1) * pp) };
    match r.find_all(f.clone()).await {
        Ok(v) => {
            let c = r.count(f).await.unwrap_or(0);
            let mut data = Vec::with_capacity(v.len());
            for e in &v {
                let email = resolve_actor_email(&user_repo, e.actor_id).await;
                data.push(to_resp(e, email));
            }
            (StatusCode::OK, Json(AuditLogListResponse { data, count: c })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}
