use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::provider::Provider;
use rzops_domain::ports::provider_repository::{ProviderFilter, ProviderRepository};

use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::*;

fn to_response(p: &Provider) -> ProviderResponse {
    ProviderResponse {
        id: p.id, name: p.name.clone(), provider_types: p.provider_types.clone(),
        contact_name: p.contact_name.clone(), contact_phone: p.contact_phone.clone(),
        contact_qq: p.contact_qq.clone(), fax: p.fax.clone(), address: p.address.clone(),
        website: p.website.clone(), description: p.description.clone(),
        status: p.status.clone(),
        created_at: p.created_at, updated_at: p.updated_at,
    }
}


/// GET /providers/:id 鈥?Get a provider by ID.
#[utoipa::path(get, path = "/api/v1/providers/{id}", params(("id" = Uuid, Path, description = "Provider ID")), responses((status = 200, body = ProviderResponse), (status = 404, body = ErrorResponse)), tag = "Provider")]
pub async fn get_provider(_auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(p)) => (StatusCode::OK, Json(to_response(&p))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// GET /providers 鈥?List providers with pagination and filtering.
#[utoipa::path(get, path = "/api/v1/providers", params(("status" = Option<String>, Query), ("q" = Option<String>, Query), ("page" = Option<i64>, Query), ("per_page" = Option<i64>, Query)), responses((status = 200, body = ProviderListResponse)), tag = "Provider")]
pub async fn list_providers(_auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Query(query): Query<ListProvidersQuery>) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;
    let filter = ProviderFilter { status: query.status, q: query.q, limit: Some(per_page), offset: Some(offset) };
    match repo.find_all(filter.clone()).await {
        Ok(providers) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let data = providers.iter().map(to_response).collect();
            (StatusCode::OK, Json(ProviderListResponse { data, count })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// POST /providers 鈥?Create a new provider.
#[utoipa::path(post, path = "/api/v1/providers", request_body = CreateProviderRequest, responses((status = 201, body = ProviderResponse), (status = 500, body = ErrorResponse)), tag = "Provider")]
pub async fn create_provider(auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(body): Json<CreateProviderRequest>) -> impl IntoResponse {
    let now = Utc::now();
    let provider = Provider {
        id: Uuid::new_v4(), name: body.name, provider_types: body.provider_types,
        contact_name: body.contact_name, contact_phone: body.contact_phone, contact_qq: body.contact_qq,
        fax: body.fax, address: body.address, website: body.website,
        description: body.description,
        status: body.status.unwrap_or_else(|| "active".to_string()),
        created_at: now, updated_at: now,
    };
    match repo.create(&provider).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "provider", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create provider: {}", e) })).into_response(),
    }
}

/// PUT /providers/:id 鈥?Update a provider.
#[utoipa::path(put, path = "/api/v1/providers/{id}", params(("id" = Uuid, Path)), request_body = UpdateProviderRequest, responses((status = 200, body = ProviderResponse), (status = 404, body = ErrorResponse)), tag = "Provider")]
pub async fn update_provider(auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>, Json(body): Json<UpdateProviderRequest>) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(p)) => p,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    };
    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let provider = Provider {
        id: existing.id, name: body.name.unwrap_or(existing.name),
        provider_types: body.provider_types.unwrap_or(existing.provider_types),
        contact_name: body.contact_name.or(existing.contact_name), contact_phone: body.contact_phone.or(existing.contact_phone),
        contact_qq: body.contact_qq.or(existing.contact_qq), fax: body.fax.or(existing.fax),
        address: body.address.or(existing.address), website: body.website.or(existing.website),
        description: body.description.or(existing.description),
        status: body.status.unwrap_or(existing.status),
        created_at: existing.created_at, updated_at: Utc::now(),
    };
    match repo.update(id, &provider).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "provider", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update provider: {}", e) })).into_response(),
    }
}

/// DELETE /providers/:id 鈥?Delete a provider.
#[utoipa::path(delete, path = "/api/v1/providers/{id}", params(("id" = Uuid, Path)), responses((status = 204), (status = 404, body = ErrorResponse)), tag = "Provider")]
pub async fn delete_provider(auth: AuthUser, State(repo): State<Arc<dyn ProviderRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "provider", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "provider not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete provider: {}", e) })).into_response(),
    }
}

