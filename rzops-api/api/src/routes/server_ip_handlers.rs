use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use rzops_domain::ports::resource_name_service::ResourceNameService;
use uuid::Uuid;

use rzops_domain::models::server_ip::ServerIP;
use rzops_domain::ports::server_ip_repository::{ServerIpFilter, ServerIpRepository};

use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::server_ip_dto::*;


fn to_response(ip: &ServerIP, server_name: Option<String>, server_status: Option<String>) -> ServerIpResponse {
    ServerIpResponse {
        id: ip.id,
        server_id: ip.server_id,
        server_name,
        server_status,
        ip_address: ip.ip_address.clone(),
        nic_name: ip.nic_name.clone(),
        ip_type: ip.ip_type.clone(),
        is_primary: ip.is_primary,
        isp_provider_id: ip.isp_provider_id,
        description: ip.description.clone(),
        status: ip.status.clone(),
        created_at: ip.created_at,
        updated_at: ip.updated_at,
    }
}

/// 批量解析服务器名称与状态映射。
async fn resolve_servers(ns: &Arc<dyn ResourceNameService>, ips: &[ServerIP]) -> HashMap<Uuid, (String, String)> {
    let ids: Vec<Option<Uuid>> = ips.iter().map(|ip| ip.server_id).collect();
    ns.resolve_server_briefs(&ids).await
}

/// GET /server-ips/:id
#[utoipa::path(get, path = "/api/v1/server-ips/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = ServerIpResponse), (status = 404, body = ErrorResponse)), tag = "ServerIp", security(("bearer_auth" = [])))]
pub async fn get_server_ip(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerIpRepository>>,
    Extension(ns): Extension<Arc<dyn ResourceNameService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(ip)) => {
            let briefs = resolve_servers(&ns, &[ip.clone()]).await;
            let (sname, sstatus) = ip.server_id.as_ref()
                .and_then(|sid| briefs.get(sid))
                .cloned()
                .map(|(n, s)| (Some(n), Some(s)))
                .unwrap_or((None, None));
            (StatusCode::OK, Json(to_response(&ip, sname, sstatus))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "server IP not found".to_string() }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// GET /server-ips
#[utoipa::path(get, path = "/api/v1/server-ips", params(ListServerIpsQuery), responses((status = 200, body = ServerIpListResponse)), tag = "ServerIp", security(("bearer_auth" = [])))]
pub async fn list_server_ips(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerIpRepository>>,
    Extension(ns): Extension<Arc<dyn ResourceNameService>>,
    Query(query): Query<ListServerIpsQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let filter = ServerIpFilter {
        server_id: query.server_id,
        status: query.status,
        ip_type: query.ip_type,
        q: query.q,
        limit: Some(per_page),
        offset: Some(offset),
    };

    match repo.find_all(filter.clone()).await {
        Ok(ips) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let briefs = resolve_servers(&ns, &ips).await;
            let data = ips.iter().map(|ip| {
                let (sname, sstatus) = ip.server_id.as_ref()
                    .and_then(|sid| briefs.get(sid))
                    .cloned()
                    .map(|(n, s)| (Some(n), Some(s)))
                    .unwrap_or((None, None));
                to_response(ip, sname, sstatus)
            }).collect();
            (StatusCode::OK, Json(ServerIpListResponse { data, count })).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// POST /server-ips
#[utoipa::path(post, path = "/api/v1/server-ips", request_body = CreateServerIpRequest, responses((status = 201, body = ServerIpResponse), (status = 400, body = ErrorResponse)), tag = "ServerIp", security(("bearer_auth" = [])))]
pub async fn create_server_ip(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerIpRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Json(body): Json<CreateServerIpRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let ip = ServerIP {
        id: Uuid::new_v4(),
        server_id: body.server_id,
        ip_address: body.ip_address,
        nic_name: body.nic_name,
        ip_type: body.ip_type.unwrap_or_else(|| "ipv4".to_string()),
        is_primary: body.is_primary.unwrap_or(false),
        isp_provider_id: body.isp_provider_id,
        description: body.description,
        status: body.status.unwrap_or_else(|| "enabled".to_string()),
        created_at: now,
        updated_at: now,
    };

    match repo.create(&ip).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "server_ip", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created, None, None)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created, None, None))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("failed to create server IP: {}", e) }),
        ).into_response(),
    }
}

/// PUT /server-ips/:id
#[utoipa::path(put, path = "/api/v1/server-ips/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateServerIpRequest, responses((status = 200, body = ServerIpResponse), (status = 404, body = ErrorResponse)), tag = "ServerIp", security(("bearer_auth" = [])))]
pub async fn update_server_ip(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerIpRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateServerIpRequest>,
) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(ip)) => ip,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "server IP not found".to_string() })).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response()
        }
    };

    let before_value = serde_json::to_value(to_response(&existing, None, None)).unwrap_or(serde_json::json!({}));
    let ip = ServerIP {
        id: existing.id,
        server_id: existing.server_id,
        ip_address: body.ip_address.unwrap_or(existing.ip_address),
        nic_name: body.nic_name.or(existing.nic_name),
        ip_type: body.ip_type.unwrap_or(existing.ip_type),
        is_primary: body.is_primary.unwrap_or(existing.is_primary),
        isp_provider_id: body.isp_provider_id.or(existing.isp_provider_id),
        description: body.description.or(existing.description),
        status: body.status.unwrap_or(existing.status),
        created_at: existing.created_at,
        updated_at: Utc::now(),
    };

    match repo.update(id, &ip).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "server_ip", Some(updated.id), before_value, serde_json::to_value(to_response(&updated, None, None)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated, None, None))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "server IP not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update server IP: {}", e) })).into_response(),
    }
}

/// DELETE /server-ips/:id
#[utoipa::path(delete, path = "/api/v1/server-ips/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "ServerIp", security(("bearer_auth" = [])))]
pub async fn delete_server_ip(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerIpRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "server_ip", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "server IP not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete server IP: {}", e) })).into_response(),
    }
}
