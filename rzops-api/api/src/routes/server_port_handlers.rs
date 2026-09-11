use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::server_port::ServerPort;
use rzops_domain::ports::server_port_repository::{ServerPortFilter, ServerPortRepository};
use rzops_domain::ports::server_port_template_repository::ServerPortTemplateRepository;

use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::server_port_dto::*;



fn to_response(p: &ServerPort) -> ServerPortResponse {
    ServerPortResponse {
        id: p.id,
        server_id: p.server_id,
        protocol: p.protocol.clone(),
        port: p.port,
        service_name: p.service_name.clone(),
        access_scope: p.access_scope.clone(),
        is_enabled: p.is_enabled,
        description: p.description.clone(),
        created_at: p.created_at,
        updated_at: p.updated_at,
        server_name: p.server_name.clone(),
        server_status: p.server_status.clone(),
    }
}

/// GET /server-ports/:id
#[utoipa::path(get, path = "/api/v1/server-ports/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = ServerPortResponse), (status = 404, body = ErrorResponse)), tag = "ServerPort", security(("bearer_auth" = [])))]
pub async fn get_server_port(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(p)) => (StatusCode::OK, Json(to_response(&p))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "server port not found".to_string() }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// GET /server-ports
#[utoipa::path(get, path = "/api/v1/server-ports", params(ListServerPortsQuery), responses((status = 200, body = ServerPortListResponse)), tag = "ServerPort", security(("bearer_auth" = [])))]
pub async fn list_server_ports(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortRepository>>,
    Query(query): Query<ListServerPortsQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let filter = ServerPortFilter {
        server_id: query.server_id,
        protocol: query.protocol,
        q: query.q,
        limit: Some(per_page),
        offset: Some(offset),
    };

    match repo.find_all(filter.clone()).await {
        Ok(ports) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let data = ports.iter().map(to_response).collect();
            (StatusCode::OK, Json(ServerPortListResponse { data, count })).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// POST /server-ports
#[utoipa::path(post, path = "/api/v1/server-ports", request_body = CreateServerPortRequest, responses((status = 201, body = ServerPortResponse), (status = 400, body = ErrorResponse)), tag = "ServerPort", security(("bearer_auth" = [])))]
pub async fn create_server_port(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Json(body): Json<CreateServerPortRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let port = ServerPort {
        id: Uuid::new_v4(),
        server_id: body.server_id,
        protocol: body.protocol,
        port: body.port,
        service_name: body.service_name,
        access_scope: body.access_scope,
        is_enabled: body.is_enabled.unwrap_or(true),
        description: body.description,
        created_at: now,
        updated_at: now,
        server_name: None,
        server_status: None,
    };

    match repo.create(&port).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "server_port", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("failed to create server port: {}", e) }),
        ).into_response(),
    }
}

/// PUT /server-ports/:id
#[utoipa::path(put, path = "/api/v1/server-ports/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateServerPortRequest, responses((status = 200, body = ServerPortResponse), (status = 404, body = ErrorResponse)), tag = "ServerPort", security(("bearer_auth" = [])))]
pub async fn update_server_port(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateServerPortRequest>,
) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "server port not found".to_string() })).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response()
        }
    };

    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let port = ServerPort {
        id: existing.id,
        server_id: body.server_id.unwrap_or(existing.server_id),
        protocol: body.protocol.unwrap_or(existing.protocol),
        port: body.port.unwrap_or(existing.port),
        service_name: body.service_name.unwrap_or(existing.service_name),
        access_scope: body.access_scope.or(existing.access_scope),
        is_enabled: body.is_enabled.unwrap_or(existing.is_enabled),
        description: body.description.or(existing.description),
        created_at: existing.created_at,
        updated_at: Utc::now(),
        server_name: existing.server_name,
        server_status: existing.server_status,
    };

    match repo.update(id, &port).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "server_port", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "server port not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update server port: {}", e) })).into_response(),
    }
}

/// DELETE /server-ports/:id
#[utoipa::path(delete, path = "/api/v1/server-ports/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "ServerPort", security(("bearer_auth" = [])))]
pub async fn delete_server_port(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "server_port", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "server port not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete server port: {}", e) })).into_response(),
    }
}

/// POST /server-ports/apply-template
/// 从端口模板批量实例化端口到指定服务器（每台服务器独立端口记录）.
#[utoipa::path(post, path = "/api/v1/server-ports/apply-template", request_body = ApplyTemplateRequest, responses((status = 201, body = ServerPortListResponse), (status = 400, body = ErrorResponse), (status = 404, body = ErrorResponse)), tag = "ServerPort", security(("bearer_auth" = [])))]
pub async fn apply_port_template(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerPortRepository>>,
    State(tpl_repo): State<Arc<dyn ServerPortTemplateRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Json(body): Json<ApplyTemplateRequest>,
) -> impl IntoResponse {
    if body.server_ids.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "server_ids must not be empty".to_string() }),
        ).into_response();
    }
    let tpl = match tpl_repo.find_by_id(body.template_id).await {
        Ok(Some(t)) => t,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "port template not found".to_string() })).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response()
        }
    };

    let now = Utc::now();
    let mut created: Vec<ServerPort> = Vec::new();
    for sid in &body.server_ids {
        let port = ServerPort {
            id: Uuid::new_v4(),
            server_id: *sid,
            protocol: tpl.protocol.clone(),
            port: tpl.port,
            service_name: tpl.service_name.clone(),
            access_scope: tpl.access_scope.clone(),
            is_enabled: tpl.is_enabled,
            description: tpl.description.clone(),
            created_at: now,
            updated_at: now,
            server_name: None,
            server_status: None,
        };
        match repo.create(&port).await {
            Ok(c) => created.push(c),
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse { error: format!("failed to apply template: {}", e) }),
                ).into_response()
            }
        }
    }
    record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "server_port", None, serde_json::json!(null), serde_json::json!({ "applied_template": body.template_id, "server_ids": body.server_ids }), None).await;
    let data = created.iter().map(to_response).collect::<Vec<_>>();
    let count = data.len() as i64;
    (StatusCode::CREATED, Json(ServerPortListResponse { data, count })).into_response()
}
