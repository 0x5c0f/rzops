use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::data_center::DataCenter;
use rzops_domain::ports::datacenter_repository::{DataCenterFilter, DataCenterRepository};

use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::datacenter_dto::*;
use crate::dto::provider_dto::ErrorResponse;

/// Convert domain DataCenter to API response.
fn to_response(dc: &DataCenter) -> DataCenterResponse {
    DataCenterResponse {
        id: dc.id,
        name: dc.name.clone(),
        provider_id: dc.provider_id,
        phone: dc.phone.clone(),
        address: dc.address.clone(),
        country: dc.country.clone(),
        line_type: dc.line_type.clone(),
        description: dc.description.clone(),
        status: dc.status.clone(),
        created_at: dc.created_at,
        updated_at: dc.updated_at,
    }
}



/// GET /data-centers/:id
#[utoipa::path(get, path = "/api/v1/data-centers/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = DataCenterResponse), (status = 404, body = ErrorResponse)), tag = "DataCenter", security(("bearer_auth" = [])))]
pub async fn get_data_center(
    _auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(dc)) => (StatusCode::OK, Json(to_response(&dc))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "data center not found".to_string(),
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

/// GET /data-centers
#[utoipa::path(get, path = "/api/v1/data-centers", params(ListDataCentersQuery), responses((status = 200, body = DataCenterListResponse)), tag = "DataCenter", security(("bearer_auth" = [])))]
pub async fn list_data_centers(
    _auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
    Query(query): Query<ListDataCentersQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let filter = DataCenterFilter {
        status: query.status,
        country: query.country,
        q: query.q,
        limit: Some(per_page),
        offset: Some(offset),
    };

    match repo.find_all(filter.clone()).await {
        Ok(dcs) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let data = dcs.iter().map(to_response).collect();
            (
                StatusCode::OK,
                Json(DataCenterListResponse { data, count }),
            )
                .into_response()
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

/// POST /data-centers
#[utoipa::path(post, path = "/api/v1/data-centers", request_body = CreateDataCenterRequest, responses((status = 201, body = DataCenterResponse), (status = 400, body = ErrorResponse)), tag = "DataCenter", security(("bearer_auth" = [])))]
pub async fn create_data_center(
    auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Json(body): Json<CreateDataCenterRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let dc = DataCenter {
        id: Uuid::new_v4(),
        name: body.name,
        provider_id: body.provider_id,
        phone: body.phone,
        address: body.address,
        country: body.country,
        line_type: body.line_type,
        description: body.description,
        status: body
            .status
            
            .unwrap_or_else(|| "active".to_string()),
        created_at: now,
        updated_at: now,
    };

    match repo.create(&dc).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "data_center", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (
                StatusCode::CREATED,
                Json(to_response(&created)),
            )
                .into_response()
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to create data center: {}", e),
            }),
        )
            .into_response(),
    }
}

/// PUT /data-centers/:id
#[utoipa::path(put, path = "/api/v1/data-centers/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateDataCenterRequest, responses((status = 200, body = DataCenterResponse), (status = 404, body = ErrorResponse)), tag = "DataCenter", security(("bearer_auth" = [])))]
pub async fn update_data_center(
    auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateDataCenterRequest>,
) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(dc)) => dc,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "data center not found".to_string(),
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
    let dc = DataCenter {
        id: existing.id,
        name: body.name.unwrap_or(existing.name),
        provider_id: body.provider_id.or(existing.provider_id),
        phone: body.phone.or(existing.phone),
        address: body.address.or(existing.address),
        country: body.country.or(existing.country),
        line_type: body
            .line_type
            
            .or(existing.line_type),
        description: body.description.or(existing.description),
        status: body
            .status
            
            .unwrap_or(existing.status),
        created_at: existing.created_at,
        updated_at: Utc::now(),
    };

    match repo.update(id, &dc).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "data_center", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "data center not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to update data center: {}", e),
            }),
        )
            .into_response(),
    }
}

/// DELETE /data-centers/:id
#[utoipa::path(delete, path = "/api/v1/data-centers/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "DataCenter", security(("bearer_auth" = [])))]
pub async fn delete_data_center(
    auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "data_center", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "data center not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to delete data center: {}", e),
            }),
        )
            .into_response(),
    }
}
