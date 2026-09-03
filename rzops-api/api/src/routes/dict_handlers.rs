use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::dict::DictItem;
use rzops_domain::ports::dict_repository::DictRepository;

use crate::auth_extractor::AuthUser;
use crate::dto::dict_dto::*;
use crate::dto::provider_dto::ErrorResponse;

fn to_response(d: &DictItem) -> DictResponse {
    DictResponse {
        id: d.id,
        dict_type: d.dict_type.clone(),
        dict_code: d.dict_code.clone(),
        dict_label: d.dict_label.clone(),
        sort_order: d.sort_order,
        enabled: d.enabled,
        remark: d.remark.clone(),
        extra_data: d.extra_data.clone(),
        created_at: d.created_at,
        updated_at: d.updated_at,
    }
}

/// GET /dicts?dict_type=xxx&enabled_only=true
pub async fn list_dicts(
    _auth: AuthUser,
    State(repo): State<Arc<dyn DictRepository>>,
    Query(query): Query<ListDictsQuery>,
) -> impl IntoResponse {
    let result = match &query.dict_type {
        Some(ty) if !ty.is_empty() => repo.list_by_type(ty, query.enabled_only.unwrap_or(false)).await,
        _ if query.enabled_only.unwrap_or(false) => repo.list_all_enabled().await,
        _ => repo.list_all().await,
    };
    match result {
        Ok(items) => {
            let mut items: Vec<DictItem> = items;
            if let Some(q) = query.q.as_deref().map(|s| s.to_lowercase()).filter(|s| !s.is_empty()) {
                items.retain(|i| {
                    i.dict_type.to_lowercase().contains(&q)
                        || i.dict_code.to_lowercase().contains(&q)
                        || i.dict_label.to_lowercase().contains(&q)
                });
            }
            let count = items.len() as i64;
            let page = query.page.unwrap_or(1).max(1);
            let per_page = query.per_page.unwrap_or(20).max(1).min(200);
            let start = ((page - 1) * per_page) as usize;
            let slice = items.into_iter().skip(start).take(per_page as usize);
            let data: Vec<DictResponse> = slice.map(|i| to_response(&i)).collect();
            (StatusCode::OK, Json(DictListResponse { data, count })).into_response()
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

/// GET /dicts/{id}
pub async fn get_dict(
    _auth: AuthUser,
    State(repo): State<Arc<dyn DictRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(d)) => (StatusCode::OK, Json(to_response(&d))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "dict item not found".to_string(),
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

/// POST /dicts
pub async fn create_dict(
    _auth: AuthUser,
    State(repo): State<Arc<dyn DictRepository>>,
    Json(body): Json<CreateDictRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let item = DictItem {
        id: Uuid::new_v4(),
        dict_type: body.dict_type.trim().to_string(),
        dict_code: body.dict_code.trim().to_string(),
        dict_label: body.dict_label.trim().to_string(),
        sort_order: body.sort_order.unwrap_or(0),
        enabled: body.enabled.unwrap_or(true),
        remark: body.remark.map(|r| r.trim().to_string()),
        extra_data: body.extra_data,
        created_at: now,
        updated_at: now,
    };
    if item.dict_type.is_empty() || item.dict_code.is_empty() || item.dict_label.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "dict_type, dict_code and dict_label are required".to_string(),
            }),
        )
            .into_response();
    }
    match repo.create(&item).await {
        Ok(()) => (StatusCode::CREATED, Json(to_response(&item))).into_response(),
        Err(e) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: format!("failed to create dict item: {}", e),
            }),
        )
            .into_response(),
    }
}

/// PUT /dicts/{id}
pub async fn update_dict(
    _auth: AuthUser,
    State(repo): State<Arc<dyn DictRepository>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateDictRequest>,
) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "dict item not found".to_string(),
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
    let label = body.dict_label.unwrap_or(existing.dict_label);
    let sort_order = body.sort_order.unwrap_or(existing.sort_order);
    let enabled = body.enabled.unwrap_or(existing.enabled);
    let remark = body.remark.or(existing.remark);
    let extra_data = body.extra_data.or(existing.extra_data);
    match repo
        .update(id, &label, sort_order, enabled, remark.as_deref(), extra_data.as_ref())
        .await
    {
        Ok(()) => match repo.find_by_id(id).await {
            Ok(Some(d)) => (StatusCode::OK, Json(to_response(&d))).into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "failed to reload dict item".to_string(),
                }),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to update dict item: {}", e),
            }),
        )
            .into_response(),
    }
}

/// DELETE /dicts/{id} — soft delete (enabled = false)
pub async fn delete_dict(
    _auth: AuthUser,
    State(repo): State<Arc<dyn DictRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("failed to delete dict item: {}", e),
            }),
        )
            .into_response(),
    }
}
