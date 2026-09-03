use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::models::ops_site::OpsSite;
use rzops_domain::ports::ops_site_repository::{OpsSiteFilter, OpsSiteRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::ops_site_dto::*;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};


fn to_response(s: &OpsSite) -> OpsSiteResponse {
    OpsSiteResponse { id: s.id, name: s.name.clone(), url: s.url.clone(), service_target: s.service_target.clone(), importance: s.importance.clone(), online_time: s.online_time, code_repo_type: s.code_repo_type.clone(), code_repo_url: s.code_repo_url.clone(), purpose: s.purpose.clone(), language_runtime: s.language_runtime.clone(), web_framework: s.web_framework.clone(), is_test_site: s.is_test_site, last_backup_time: s.last_backup_time, status: s.status.clone(), environment: s.environment.clone(), offline_time: s.offline_time, offline_reason: s.offline_reason.clone(), function_summary: s.function_summary.clone(), remarks: s.remarks.clone(), created_at: s.created_at, updated_at: s.updated_at, deleted_at: s.deleted_at }
}

#[utoipa::path(get, path = "/api/v1/ops-sites/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = OpsSiteResponse), (status = 404, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn get_ops_site(_auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.find_by_id(id).await { Ok(Some(s)) => (StatusCode::OK, Json(to_response(&s))).into_response(), Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(get, path = "/api/v1/ops-sites", params(ListOpsSitesQuery), responses((status = 200, body = OpsSiteListResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn list_ops_sites(_auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Query(q): Query<ListOpsSitesQuery>) -> impl IntoResponse {
    let page = q.page.unwrap_or(1).max(1); let per_page = q.per_page.unwrap_or(20).min(100);
    let filter = OpsSiteFilter { status: q.status, environment: q.environment, importance: q.importance, server_id: q.server_id, q: q.q, limit: Some(per_page), offset: Some((page - 1) * per_page) };
    match repo.find_all(filter.clone()).await { Ok(ss) => { let count = repo.count(filter).await.unwrap_or(0); (StatusCode::OK, Json(OpsSiteListResponse { data: ss.iter().map(to_response).collect(), count })).into_response() }, Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(post, path = "/api/v1/ops-sites", request_body = CreateOpsSiteRequest, responses((status = 201, body = OpsSiteResponse), (status = 400, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn create_ops_site(auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(body): Json<CreateOpsSiteRequest>) -> impl IntoResponse {
    let now = Utc::now();
    let s = OpsSite { id: Uuid::new_v4(), name: body.name, url: body.url, service_target: body.service_target, importance: body.importance, online_time: body.online_time, code_repo_type: body.code_repo_type, code_repo_url: body.code_repo_url, purpose: body.purpose, language_runtime: body.language_runtime, web_framework: body.web_framework, is_test_site: body.is_test_site.unwrap_or(false), last_backup_time: body.last_backup_time, status: body.status.unwrap_or_else(|| "active".to_string()), environment: body.environment, offline_time: body.offline_time, offline_reason: body.offline_reason, function_summary: body.function_summary, remarks: body.remarks, created_at: now, updated_at: now, deleted_at: None };
    match repo.create(&s).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "ops_site", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create ops site: {}", e) })).into_response() }
}
#[utoipa::path(put, path = "/api/v1/ops-sites/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateOpsSiteRequest, responses((status = 200, body = OpsSiteResponse), (status = 404, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn update_ops_site(auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>, Json(body): Json<UpdateOpsSiteRequest>) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await { Ok(Some(s)) => s, Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() };
    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let s = OpsSite { id: existing.id, name: body.name.unwrap_or(existing.name), url: body.url.or(existing.url), service_target: body.service_target.or(existing.service_target), importance: body.importance.or(existing.importance), online_time: body.online_time.or(existing.online_time), code_repo_type: body.code_repo_type.or(existing.code_repo_type), code_repo_url: body.code_repo_url.or(existing.code_repo_url), purpose: body.purpose.or(existing.purpose), language_runtime: body.language_runtime.or(existing.language_runtime), web_framework: body.web_framework.or(existing.web_framework), is_test_site: body.is_test_site.unwrap_or(existing.is_test_site), last_backup_time: body.last_backup_time.or(existing.last_backup_time), status: body.status.unwrap_or(existing.status), environment: body.environment.or(existing.environment), offline_time: body.offline_time.or(existing.offline_time), offline_reason: body.offline_reason.or(existing.offline_reason), function_summary: body.function_summary.or(existing.function_summary), remarks: body.remarks.or(existing.remarks), created_at: existing.created_at, updated_at: Utc::now(), deleted_at: existing.deleted_at };
    match repo.update(id, &s).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "ops_site", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update ops site: {}", e) })).into_response() }
}
#[utoipa::path(delete, path = "/api/v1/ops-sites/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn delete_ops_site(auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "ops_site", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete ops site: {}", e) })).into_response() }
}
