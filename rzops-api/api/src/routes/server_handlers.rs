use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use rzops_domain::enums::{HostingType, ServerRole, ServerStatus, ServerType, WebServerSoftware};
use rzops_domain::models::server::Server;
use rzops_domain::ports::server_repository::{ServerFilter, ServerRepository};

use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::server_dto::*;

// ── Enum converters ──

fn to_status_string(s: &ServerStatus) -> String {
    match s {
        ServerStatus::Active => "active".to_string(),
        ServerStatus::Retired => "retired".to_string(),
    }
}

fn parse_status(s: &str) -> ServerStatus {
    match s {
        "active" => ServerStatus::Active,
        "retired" => ServerStatus::Retired,
        _ => ServerStatus::Active,
    }
}

fn parse_hosting_type(s: &str) -> HostingType {
    match s {
        "colocation" => HostingType::Colocation,
        "rental" => HostingType::Rental,
        "cloud" => HostingType::Cloud,
        "self_owned" => HostingType::SelfOwned,
        _ => HostingType::Other,
    }
}

fn to_hosting_type_string(ht: &HostingType) -> String {
    match ht {
        HostingType::Colocation => "colocation".to_string(),
        HostingType::Rental => "rental".to_string(),
        HostingType::Cloud => "cloud".to_string(),
        HostingType::SelfOwned => "self_owned".to_string(),
        HostingType::Other => "other".to_string(),
    }
}

fn parse_server_type(s: &str) -> ServerType {
    match s {
        "physical" => ServerType::Physical,
        "virtual" => ServerType::Virtual,
        "cloud" => ServerType::Cloud,
        "container" => ServerType::Container,
        _ => ServerType::Other,
    }
}

fn to_server_type_string(st: &ServerType) -> String {
    match st {
        ServerType::Physical => "physical".to_string(),
        ServerType::Virtual => "virtual".to_string(),
        ServerType::Cloud => "cloud".to_string(),
        ServerType::Container => "container".to_string(),
        ServerType::Other => "other".to_string(),
    }
}

fn parse_server_role(s: &str) -> ServerRole {
    match s {
        "web" => ServerRole::Web,
        "db" => ServerRole::Db,
        "cache" => ServerRole::Cache,
        "worker" => ServerRole::Worker,
        "file" => ServerRole::File,
        "monitor" => ServerRole::Monitor,
        "backup" => ServerRole::Backup,
        _ => ServerRole::Other,
    }
}

fn to_server_role_string(r: &ServerRole) -> String {
    match r {
        ServerRole::Web => "web".to_string(),
        ServerRole::Db => "db".to_string(),
        ServerRole::Cache => "cache".to_string(),
        ServerRole::Worker => "worker".to_string(),
        ServerRole::File => "file".to_string(),
        ServerRole::Monitor => "monitor".to_string(),
        ServerRole::Backup => "backup".to_string(),
        ServerRole::Other => "other".to_string(),
    }
}

fn parse_web_server_software(s: &str) -> WebServerSoftware {
    match s {
        "nginx" => WebServerSoftware::Nginx,
        "apache" => WebServerSoftware::Apache,
        "iis" => WebServerSoftware::Iis,
        "openresty" => WebServerSoftware::OpenResty,
        "caddy" => WebServerSoftware::Caddy,
        "traefik" => WebServerSoftware::Traefik,
        "tomcat" => WebServerSoftware::Tomcat,
        _ => WebServerSoftware::Other,
    }
}

fn to_web_server_string(w: &WebServerSoftware) -> String {
    match w {
        WebServerSoftware::Nginx => "nginx".to_string(),
        WebServerSoftware::Apache => "apache".to_string(),
        WebServerSoftware::Iis => "iis".to_string(),
        WebServerSoftware::OpenResty => "openresty".to_string(),
        WebServerSoftware::Caddy => "caddy".to_string(),
        WebServerSoftware::Traefik => "traefik".to_string(),
        WebServerSoftware::Tomcat => "tomcat".to_string(),
        WebServerSoftware::Other => "other".to_string(),
    }
}

// ── Model ↔ DTO ──

fn to_response(s: &Server) -> ServerResponse {
    ServerResponse {
        id: s.id,
        asset_code: s.asset_code.clone(),
        name: s.name.clone(),
        primary_ip: s.primary_ip.clone(),
        location: s.location.clone(),
        isp_provider_id: s.isp_provider_id,
        data_center_id: s.data_center_id,
        hosting_type: s.hosting_type.as_ref().map(to_hosting_type_string),
        is_dual_line: s.is_dual_line,
        lease_start_date: s.lease_start_date,
        lease_end_date: s.lease_end_date,
        price: s.price.map(|p| p.to_string()),
        price_currency: s.price_currency.clone(),
        server_type: s.server_type.as_ref().map(to_server_type_string),
        role_tags: s.role_tags.iter().map(to_server_role_string).collect(),
        is_database_server: s.is_database_server,
        cpu: s.cpu.clone(),
        memory_gb: s.memory_gb,
        is_raid: s.is_raid,
        raid_level: s.raid_level.clone(),
        disk_layout: s.disk_layout.clone(),
        hardware_config: s.hardware_config.clone(),
        architecture: s.architecture.clone(),
        maintainer_id: s.maintainer_id,
        brand: s.brand.clone(),
        warranty_info: s.warranty_info.clone(),
        operating_system: s.operating_system.clone(),
        web_server_type: s.web_server_type.iter().map(to_web_server_string).collect(),
        server_provider_id: s.server_provider_id,
        software_provider_id: s.software_provider_id,
        status: to_status_string(&s.status),
        offline_time: s.offline_time,
        offline_reason: s.offline_reason.clone(),
        remarks: s.remarks.clone(),
        created_at: s.created_at,
        updated_at: s.updated_at,
    }
}

fn build_server_from_create(body: CreateServerRequest) -> Server {
    let now = Utc::now();
    Server {
        id: Uuid::new_v4(),
        asset_code: body.asset_code,
        name: body.name,
        primary_ip: body.primary_ip,
        location: body.location,
        isp_provider_id: body.isp_provider_id,
        data_center_id: body.data_center_id,
        hosting_type: body.hosting_type.as_deref().map(parse_hosting_type),
        is_dual_line: body.is_dual_line.unwrap_or(false),
        lease_start_date: body.lease_start_date,
        lease_end_date: body.lease_end_date,
        price: body.price.and_then(|p| p.parse::<Decimal>().ok()),
        price_currency: body.price_currency.unwrap_or_else(|| "CNY".to_string()),
        server_type: body.server_type.as_deref().map(parse_server_type),
        role_tags: body
            .role_tags
            .unwrap_or_default()
            .iter()
            .map(|s| parse_server_role(s))
            .collect(),
        is_database_server: body.is_database_server.unwrap_or(false),
        cpu: body.cpu,
        memory_gb: body.memory_gb,
        is_raid: body.is_raid.unwrap_or(false),
        raid_level: body.raid_level,
        disk_layout: body.disk_layout,
        hardware_config: body.hardware_config,
        architecture: body.architecture,
        maintainer_id: body.maintainer_id,
        brand: body.brand,
        warranty_info: body.warranty_info,
        operating_system: body.operating_system,
        web_server_type: body
            .web_server_type
            .unwrap_or_default()
            .iter()
            .map(|s| parse_web_server_software(s))
            .collect(),
        server_provider_id: body.server_provider_id,
        software_provider_id: body.software_provider_id,
        status: body
            .status
            .as_deref()
            .map(parse_status)
            .unwrap_or(ServerStatus::Active),
        offline_time: body.offline_time,
        offline_reason: body.offline_reason,
        remarks: body.remarks,
        created_at: now,
        updated_at: now,
    }
}

// ── Handlers ──

/// GET /servers/:id
#[utoipa::path(get, path = "/api/v1/servers/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = ServerResponse), (status = 404, body = ErrorResponse)), tag = "Server", security(("bearer_auth" = [])))]
pub async fn get_server(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(s)) => (StatusCode::OK, Json(to_response(&s))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "server not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("database error: {}", e),
            }),
        )
            .into_response(),
    }
}

/// GET /servers
#[utoipa::path(get, path = "/api/v1/servers", params(ListServersQuery), responses((status = 200, body = ServerListResponse)), tag = "Server", security(("bearer_auth" = [])))]
pub async fn list_servers(
    _auth: AuthUser,
    State(repo): State<Arc<dyn ServerRepository>>,
    Query(query): Query<ListServersQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let filter = ServerFilter {
        status: query.status,
        data_center_id: query.data_center_id,
        server_type: query.server_type,
        q: query.q,
        limit: Some(per_page),
        offset: Some(offset),
    };

    match repo.find_all(filter.clone()).await {
        Ok(servers) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let data = servers.iter().map(to_response).collect();
            (StatusCode::OK, Json(ServerListResponse { data, count })).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("database error: {}", e),
            }),
        )
            .into_response(),
    }
}

/// POST /servers
#[utoipa::path(post, path = "/api/v1/servers", request_body = CreateServerRequest, responses((status = 201, body = ServerResponse), (status = 400, body = ErrorResponse)), tag = "Server", security(("bearer_auth" = [])))]
pub async fn create_server(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Json(body): Json<CreateServerRequest>,
) -> impl IntoResponse {
    let server = build_server_from_create(body);

    match repo.create(&server).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "server", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to create server: {}", e),
            }),
        )
            .into_response(),
    }
}

/// PUT /servers/:id
#[utoipa::path(put, path = "/api/v1/servers/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateServerRequest, responses((status = 200, body = ServerResponse), (status = 404, body = ErrorResponse)), tag = "Server", security(("bearer_auth" = [])))]
pub async fn update_server(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateServerRequest>,
) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "server not found".to_string(),
                }),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("database error: {}", e),
                }),
            )
                .into_response()
        }
    };

    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let server = Server {
        id: existing.id,
        asset_code: body.asset_code.or(existing.asset_code),
        name: body.name.unwrap_or(existing.name),
        primary_ip: body.primary_ip.or(existing.primary_ip),
        location: body.location.or(existing.location),
        isp_provider_id: body.isp_provider_id.or(existing.isp_provider_id),
        data_center_id: body.data_center_id.or(existing.data_center_id),
        hosting_type: body
            .hosting_type
            .as_deref()
            .map(parse_hosting_type)
            .or(existing.hosting_type),
        is_dual_line: body.is_dual_line.unwrap_or(existing.is_dual_line),
        lease_start_date: body.lease_start_date.or(existing.lease_start_date),
        lease_end_date: body.lease_end_date.or(existing.lease_end_date),
        price: body
            .price
            .and_then(|p| p.parse::<Decimal>().ok())
            .or(existing.price),
        price_currency: body.price_currency.unwrap_or(existing.price_currency),
        server_type: body
            .server_type
            .as_deref()
            .map(parse_server_type)
            .or(existing.server_type),
        role_tags: body
            .role_tags
            .map(|tags| tags.iter().map(|s| parse_server_role(s)).collect())
            .unwrap_or(existing.role_tags),
        is_database_server: body.is_database_server.unwrap_or(existing.is_database_server),
        cpu: body.cpu.or(existing.cpu),
        memory_gb: body.memory_gb.or(existing.memory_gb),
        is_raid: body.is_raid.unwrap_or(existing.is_raid),
        raid_level: body.raid_level.or(existing.raid_level),
        disk_layout: body.disk_layout.or(existing.disk_layout),
        hardware_config: body.hardware_config.or(existing.hardware_config),
        architecture: body.architecture.or(existing.architecture),
        maintainer_id: body.maintainer_id.or(existing.maintainer_id),
        brand: body.brand.or(existing.brand),
        warranty_info: body.warranty_info.or(existing.warranty_info),
        operating_system: body.operating_system.or(existing.operating_system),
        web_server_type: body
            .web_server_type
            .map(|types| types.iter().map(|s| parse_web_server_software(s)).collect())
            .unwrap_or(existing.web_server_type),
        server_provider_id: body.server_provider_id.or(existing.server_provider_id),
        software_provider_id: body.software_provider_id.or(existing.software_provider_id),
        status: body
            .status
            .as_deref()
            .map(parse_status)
            .unwrap_or(existing.status),
        offline_time: body.offline_time.or(existing.offline_time),
        offline_reason: body.offline_reason.or(existing.offline_reason),
        remarks: body.remarks.or(existing.remarks),
        created_at: existing.created_at,
        updated_at: Utc::now(),
    };

    match repo.update(id, &server).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "server", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "server not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to update server: {}", e),
            }),
        )
            .into_response(),
    }
}

/// DELETE /servers/:id
#[utoipa::path(delete, path = "/api/v1/servers/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "Server", security(("bearer_auth" = [])))]
pub async fn delete_server(
    auth: AuthUser,
    State(repo): State<Arc<dyn ServerRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "server", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "server not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to delete server: {}", e),
            }),
        )
            .into_response(),
    }
}
