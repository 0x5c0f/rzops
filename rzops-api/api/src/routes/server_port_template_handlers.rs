use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::server_port_template::ServerPortTemplate;
use rzops_domain::ports::server_port_template_repository::{ServerPortTemplateFilter, ServerPortTemplateRepository};

use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::server_port_template_dto::*;



fn to_response(t: &ServerPortTemplate) -> ServerPortTemplateResponse {
    ServerPortTemplateResponse {
        id: t.id,
        name: t.name.clone(),
        protocol: t.protocol.clone(),
        port: t.port,
        service_name: t.service_name.clone(),
        access_scope: t.access_scope.clone(),
        is_enabled: t.is_enabled,
        description: t.description.clone(),
        created_at: t.created_at,
        updated_at: t.updated_at,
    }
}

/// GET /server-port-templates/:id
#[utoipa::path(get, path = "/api/v1/server-port-templates/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = ServerPortTemplateResponse), (status = 404, body = ErrorResponse)), tag = "ServerPortTemplate", security(("bearer_auth" = [])))]
pub async fn get_server_port_template(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortTemplateRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(t)) => (StatusCode::OK, Json(to_response(&t))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "port template not found".to_string() }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// GET /server-port-templates
#[utoipa::path(get, path = "/api/v1/server-port-templates", params(ListServerPortTemplatesQuery), responses((status = 200, body = ServerPortTemplateListResponse)), tag = "ServerPortTemplate", security(("bearer_auth" = [])))]
pub async fn list_server_port_templates(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortTemplateRepository>>,
    Query(query): Query<ListServerPortTemplatesQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let filter = ServerPortTemplateFilter {
        q: query.q,
        limit: Some(per_page),
        offset: Some(offset),
    };

    match repo.find_all(filter.clone()).await {
        Ok(items) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let data = items.iter().map(to_response).collect();
            (StatusCode::OK, Json(ServerPortTemplateListResponse { data, count })).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// POST /server-port-templates
#[utoipa::path(post, path = "/api/v1/server-port-templates", request_body = CreateServerPortTemplateRequest, responses((status = 201, body = ServerPortTemplateResponse), (status = 400, body = ErrorResponse)), tag = "ServerPortTemplate", security(("bearer_auth" = [])))]
pub async fn create_server_port_template(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortTemplateRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Json(body): Json<CreateServerPortTemplateRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let tpl = ServerPortTemplate {
        id: Uuid::new_v4(),
        name: body.name,
        protocol: body.protocol,
        port: body.port,
        service_name: body.service_name,
        access_scope: body.access_scope,
        is_enabled: body.is_enabled.unwrap_or(true),
        description: body.description,
        created_at: now,
        updated_at: now,
    };

    match repo.create(&tpl).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "server_port_template", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("failed to create port template: {}", e) }),
        ).into_response(),
    }
}

/// PUT /server-port-templates/:id
#[utoipa::path(put, path = "/api/v1/server-port-templates/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateServerPortTemplateRequest, responses((status = 200, body = ServerPortTemplateResponse), (status = 404, body = ErrorResponse)), tag = "ServerPortTemplate", security(("bearer_auth" = [])))]
pub async fn update_server_port_template(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortTemplateRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateServerPortTemplateRequest>,
) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "port template not found".to_string() })).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response()
        }
    };

    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let tpl = ServerPortTemplate {
        id: existing.id,
        name: body.name.unwrap_or(existing.name),
        protocol: body.protocol.unwrap_or(existing.protocol),
        port: body.port.unwrap_or(existing.port),
        service_name: body.service_name.unwrap_or(existing.service_name),
        access_scope: body.access_scope.or(existing.access_scope),
        is_enabled: body.is_enabled.unwrap_or(existing.is_enabled),
        description: body.description.or(existing.description),
        created_at: existing.created_at,
        updated_at: Utc::now(),
    };

    match repo.update(id, &tpl).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "server_port_template", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "port template not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update port template: {}", e) })).into_response(),
    }
}

/// DELETE /server-port-templates/:id
#[utoipa::path(delete, path = "/api/v1/server-port-templates/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "ServerPortTemplate", security(("bearer_auth" = [])))]
pub async fn delete_server_port_template(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortTemplateRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "server_port_template", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "port template not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete port template: {}", e) })).into_response(),
    }
}
