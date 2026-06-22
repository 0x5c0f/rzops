use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::enums::CommonStatus;
use rzops_domain::models::provider::Provider;
use rzops_domain::ports::provider_repository::{ProviderFilter, ProviderRepository};

use crate::auth_extractor::AuthUser;
use crate::dto::provider_dto::*;

fn to_response(p: &Provider) -> ProviderResponse {
    ProviderResponse {
        id: p.id, name: p.name.clone(), provider_types: p.provider_types.clone(),
        contact_name: p.contact_name.clone(), contact_phone: p.contact_phone.clone(),
        contact_qq: p.contact_qq.clone(), fax: p.fax.clone(), address: p.address.clone(),
        website: p.website.clone(), country: p.country.clone(), description: p.description.clone(),
        status: match p.status { CommonStatus::Active => "active", CommonStatus::Inactive => "inactive", CommonStatus::Archived => "archived" }.to_string(),
        created_at: p.created_at, updated_at: p.updated_at,
    }
}

fn parse_status(s: &str) -> CommonStatus {
    match s { "active" => CommonStatus::Active, "inactive" => CommonStatus::Inactive, "archived" => CommonStatus::Archived, _ => CommonStatus::Active }
}

/// GET /providers/:id — Get a provider by ID.
#[utoipa::path(get, path = "/api/v1/providers/{id}", params(("id" = Uuid, Path, description = "Provider ID")), responses((status = 200, body = ProviderResponse), (status = 404, body = ErrorResponse)), tag = "Provider")]
pub async fn get_provider(_auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(p)) => (StatusCode::OK, Json(to_response(&p))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// GET /providers — List providers with pagination and filtering.
#[utoipa::path(get, path = "/api/v1/providers", params(("status" = Option<String>, Query), ("country" = Option<String>, Query), ("q" = Option<String>, Query), ("page" = Option<i64>, Query), ("per_page" = Option<i64>, Query)), responses((status = 200, body = ProviderListResponse)), tag = "Provider")]
pub async fn list_providers(_auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Query(query): Query<ListProvidersQuery>) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;
    let filter = ProviderFilter { status: query.status, country: query.country, q: query.q, limit: Some(per_page), offset: Some(offset) };
    match repo.find_all(filter.clone()).await {
        Ok(providers) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let data = providers.iter().map(to_response).collect();
            (StatusCode::OK, Json(ProviderListResponse { data, count })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// POST /providers — Create a new provider.
#[utoipa::path(post, path = "/api/v1/providers", request_body = CreateProviderRequest, responses((status = 201, body = ProviderResponse), (status = 500, body = ErrorResponse)), tag = "Provider")]
pub async fn create_provider(_auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Json(body): Json<CreateProviderRequest>) -> impl IntoResponse {
    let now = Utc::now();
    let provider = Provider {
        id: Uuid::new_v4(), name: body.name, provider_types: body.provider_types,
        contact_name: body.contact_name, contact_phone: body.contact_phone, contact_qq: body.contact_qq,
        fax: body.fax, address: body.address, website: body.website, country: body.country,
        description: body.description,
        status: body.status.as_deref().map(parse_status).unwrap_or(CommonStatus::Active),
        created_at: now, updated_at: now,
    };
    match repo.create(&provider).await {
        Ok(created) => (StatusCode::CREATED, Json(to_response(&created))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create provider: {}", e) })).into_response(),
    }
}

/// PUT /providers/:id — Update a provider.
#[utoipa::path(put, path = "/api/v1/providers/{id}", params(("id" = Uuid, Path)), request_body = UpdateProviderRequest, responses((status = 200, body = ProviderResponse), (status = 404, body = ErrorResponse)), tag = "Provider")]
pub async fn update_provider(_auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Path(id): Path<Uuid>, Json(body): Json<UpdateProviderRequest>) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(p)) => p,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    };
    let provider = Provider {
        id: existing.id, name: body.name.unwrap_or(existing.name),
        provider_types: body.provider_types.unwrap_or(existing.provider_types),
        contact_name: body.contact_name.or(existing.contact_name), contact_phone: body.contact_phone.or(existing.contact_phone),
        contact_qq: body.contact_qq.or(existing.contact_qq), fax: body.fax.or(existing.fax),
        address: body.address.or(existing.address), website: body.website.or(existing.website),
        country: body.country.or(existing.country), description: body.description.or(existing.description),
        status: body.status.as_deref().map(parse_status).unwrap_or(existing.status),
        created_at: existing.created_at, updated_at: Utc::now(),
    };
    match repo.update(id, &provider).await {
        Ok(Some(updated)) => (StatusCode::OK, Json(to_response(&updated))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update provider: {}", e) })).into_response(),
    }
}

/// DELETE /providers/:id — Delete a provider.
#[utoipa::path(delete, path = "/api/v1/providers/{id}", params(("id" = Uuid, Path)), responses((status = 204), (status = 404, body = ErrorResponse)), tag = "Provider")]
pub async fn delete_provider(_auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete provider: {}", e) })).into_response(),
    }
}
