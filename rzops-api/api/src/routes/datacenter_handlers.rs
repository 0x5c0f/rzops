use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::enums::{CommonStatus, LineType};
use rzops_domain::models::data_center::DataCenter;
use rzops_domain::ports::datacenter_repository::{DataCenterFilter, DataCenterRepository};

use crate::auth_extractor::AuthUser;
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
        province: dc.province.clone(),
        city: dc.city.clone(),
        line_type: dc.line_type.as_ref().map(|lt| match lt {
            LineType::SingleLine => "single_line".to_string(),
            LineType::DualLine => "dual_line".to_string(),
            LineType::MultiLine => "multi_line".to_string(),
            LineType::Other => "other".to_string(),
        }),
        description: dc.description.clone(),
        status: match dc.status {
            CommonStatus::Active => "active".to_string(),
            CommonStatus::Inactive => "inactive".to_string(),
            CommonStatus::Archived => "archived".to_string(),
        },
        created_at: dc.created_at,
        updated_at: dc.updated_at,
    }
}

fn parse_status(s: &str) -> CommonStatus {
    match s {
        "active" => CommonStatus::Active,
        "inactive" => CommonStatus::Inactive,
        "archived" => CommonStatus::Archived,
        _ => CommonStatus::Active,
    }
}

fn parse_line_type(s: &str) -> LineType {
    match s {
        "single_line" => LineType::SingleLine,
        "dual_line" => LineType::DualLine,
        "multi_line" => LineType::MultiLine,
        _ => LineType::Other,
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
    _auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
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
        province: body.province,
        city: body.city,
        line_type: body.line_type.as_deref().map(parse_line_type),
        description: body.description,
        status: body
            .status
            .as_deref()
            .map(parse_status)
            .unwrap_or(CommonStatus::Active),
        created_at: now,
        updated_at: now,
    };

    match repo.create(&dc).await {
        Ok(created) => (
            StatusCode::CREATED,
            Json(to_response(&created)),
        )
            .into_response(),
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
    _auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
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

    let dc = DataCenter {
        id: existing.id,
        name: body.name.unwrap_or(existing.name),
        provider_id: body.provider_id.or(existing.provider_id),
        phone: body.phone.or(existing.phone),
        address: body.address.or(existing.address),
        country: body.country.or(existing.country),
        province: body.province.or(existing.province),
        city: body.city.or(existing.city),
        line_type: body
            .line_type
            .as_deref()
            .map(parse_line_type)
            .or(existing.line_type),
        description: body.description.or(existing.description),
        status: body
            .status
            .as_deref()
            .map(parse_status)
            .unwrap_or(existing.status),
        created_at: existing.created_at,
        updated_at: Utc::now(),
    };

    match repo.update(id, &dc).await {
        Ok(Some(updated)) => (StatusCode::OK, Json(to_response(&updated))).into_response(),
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
    _auth: AuthUser,
    State(repo): State<Arc<dyn DataCenterRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
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
