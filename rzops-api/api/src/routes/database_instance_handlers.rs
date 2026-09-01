use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::models::database_instance::DatabaseInstance;
use rzops_domain::ports::database_instance_repository::{DatabaseInstanceFilter, DatabaseInstanceRepository};
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::database_instance_dto::*;

fn to_response(d: &DatabaseInstance) -> DatabaseInstanceResponse {
    DatabaseInstanceResponse { id: d.id, server_id: d.server_id, name: d.name.clone(), db_type: d.db_type.clone(), description: d.description.clone(), status: d.status.clone(), offline_time: d.offline_time, is_self_installed: d.is_self_installed, importance: d.importance.clone(), is_ops_managed: d.is_ops_managed, port: d.port, instance_name: d.instance_name.clone(), created_at: d.created_at, updated_at: d.updated_at, deleted_at: d.deleted_at }
}

#[utoipa::path(get, path = "/api/v1/database-instances/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = DatabaseInstanceResponse), (status = 404, body = ErrorResponse)), tag = "DatabaseInstance", security(("bearer_auth" = [])))]
pub async fn get_database_instance(_auth: AuthUser, State(repo): State<Arc<dyn DatabaseInstanceRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.find_by_id(id).await { Ok(Some(d)) => (StatusCode::OK, Json(to_response(&d))).into_response(), Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "database instance not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(get, path = "/api/v1/database-instances", params(ListDatabaseInstancesQuery), responses((status = 200, body = DatabaseInstanceListResponse)), tag = "DatabaseInstance", security(("bearer_auth" = [])))]
pub async fn list_database_instances(_auth: AuthUser, State(repo): State<Arc<dyn DatabaseInstanceRepository>>, Query(q): Query<ListDatabaseInstancesQuery>) -> impl IntoResponse {
    let page = q.page.unwrap_or(1).max(1); let per_page = q.per_page.unwrap_or(20).min(100);
    let filter = DatabaseInstanceFilter { status: q.status, db_type: q.db_type, server_id: q.server_id, q: q.q, limit: Some(per_page), offset: Some((page - 1) * per_page) };
    match repo.find_all(filter.clone()).await { Ok(ds) => { let count = repo.count(filter).await.unwrap_or(0); (StatusCode::OK, Json(DatabaseInstanceListResponse { data: ds.iter().map(to_response).collect(), count })).into_response() }, Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(post, path = "/api/v1/database-instances", request_body = CreateDatabaseInstanceRequest, responses((status = 201, body = DatabaseInstanceResponse), (status = 400, body = ErrorResponse)), tag = "DatabaseInstance", security(("bearer_auth" = [])))]
pub async fn create_database_instance(auth: AuthUser, State(repo): State<Arc<dyn DatabaseInstanceRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(body): Json<CreateDatabaseInstanceRequest>) -> impl IntoResponse {
    let now = Utc::now();
    let d = DatabaseInstance { id: Uuid::new_v4(), server_id: body.server_id, name: body.name, db_type: body.db_type, description: body.description, status: body.status.unwrap_or_else(|| "active".to_string()), offline_time: body.offline_time, is_self_installed: body.is_self_installed.unwrap_or(true), importance: body.importance, is_ops_managed: body.is_ops_managed.unwrap_or(true), port: body.port, instance_name: body.instance_name, created_at: now, updated_at: now, deleted_at: None };
    match repo.create(&d).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "database_instance", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create database instance: {}", e) })).into_response() }
}
#[utoipa::path(put, path = "/api/v1/database-instances/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateDatabaseInstanceRequest, responses((status = 200, body = DatabaseInstanceResponse), (status = 404, body = ErrorResponse)), tag = "DatabaseInstance", security(("bearer_auth" = [])))]
pub async fn update_database_instance(auth: AuthUser, State(repo): State<Arc<dyn DatabaseInstanceRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>, Json(body): Json<UpdateDatabaseInstanceRequest>) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await { Ok(Some(d)) => d, Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "database instance not found".to_string() })).into_response(), Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() };
    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let d = DatabaseInstance { id: existing.id, server_id: body.server_id.or(existing.server_id), name: body.name.unwrap_or(existing.name), db_type: body.db_type.unwrap_or(existing.db_type), description: body.description.or(existing.description), status: body.status.unwrap_or(existing.status), offline_time: body.offline_time.or(existing.offline_time), is_self_installed: body.is_self_installed.unwrap_or(existing.is_self_installed), importance: body.importance.or(existing.importance), is_ops_managed: body.is_ops_managed.unwrap_or(existing.is_ops_managed), port: body.port.or(existing.port), instance_name: body.instance_name.or(existing.instance_name), created_at: existing.created_at, updated_at: Utc::now(), deleted_at: existing.deleted_at };
    match repo.update(id, &d).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "database_instance", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "database instance not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update database instance: {}", e) })).into_response() }
}
#[utoipa::path(delete, path = "/api/v1/database-instances/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "DatabaseInstance", security(("bearer_auth" = [])))]
pub async fn delete_database_instance(auth: AuthUser, State(repo): State<Arc<dyn DatabaseInstanceRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "database_instance", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "database instance not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete database instance: {}", e) })).into_response() }
}
