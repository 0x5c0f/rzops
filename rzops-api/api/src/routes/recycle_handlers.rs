use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use rzops_domain::enums::ChangeType;
use rzops_domain::models::change_record::ChangeRecord;
use rzops_domain::ports::change_record_repository::ChangeRecordRepository;
use rzops_domain::ports::recycle_repository::RecycleRepository;

use crate::auth_extractor::AuthUser;
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::recycle_dto::*;

/// Shared state for recycle bin routes.
#[derive(Clone)]
pub struct RecycleState {
    pub recycle_repo: Arc<dyn RecycleRepository>,
    pub change_repo: Arc<dyn ChangeRecordRepository>,
}

/// GET /recycle — list soft-deleted items across resources.
#[utoipa::path(
    get,
    path = "/api/v1/recycle",
    params(
        ("resource_type" = Option<String>, Query),
        ("q" = Option<String>, Query),
        ("page" = Option<i64>, Query),
        ("per_page" = Option<i64>, Query),
    ),
    responses((status = 200, body = RecycleListResponse)),
    tag = "Recycle"
)]
pub async fn list_recycle(
    _auth: AuthUser,
    State(state): State<RecycleState>,
    Query(query): Query<ListRecycleQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    match state
        .recycle_repo
        .list(query.resource_type.as_deref(), query.q.as_deref(), per_page, offset)
        .await
    {
        Ok((entries, count)) => {
            let data: Vec<RecycleItem> = entries
                .into_iter()
                .map(|e| RecycleItem {
                    resource_type: e.resource_type,
                    id: e.id,
                    name: e.name,
                    deleted_at: e.deleted_at,
                    data: e.data,
                })
                .collect();
            (StatusCode::OK, Json(RecycleListResponse { data, count })).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}

/// POST /recycle/restore — restore a soft-deleted item.
#[utoipa::path(
    post,
    path = "/api/v1/recycle/restore",
    request_body = RestoreRequest,
    responses((status = 200), (status = 400, body = ErrorResponse), (status = 404, body = ErrorResponse)),
    tag = "Recycle"
)]
pub async fn restore_item(
    _auth: AuthUser,
    State(state): State<RecycleState>,
    Json(body): Json<RestoreRequest>,
) -> impl IntoResponse {
    match state.recycle_repo.restore(&body.resource_type, body.id).await {
        Ok(true) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response(),
        Ok(false) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: format!("unsupported resource type: {}", body.resource_type) })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("restore failed: {}", e) })).into_response(),
    }
}

/// DELETE /recycle/:resource_type/:id — permanently delete an item.
#[utoipa::path(
    delete,
    path = "/api/v1/recycle/{resource_type}/{id}",
    params(
        ("resource_type" = String, Path, description = "Resource type"),
        ("id" = Uuid, Path, description = "Item ID"),
    ),
    responses((status = 200), (status = 400, body = ErrorResponse), (status = 404, body = ErrorResponse)),
    tag = "Recycle"
)]
pub async fn purge_item(
    auth: AuthUser,
    State(state): State<RecycleState>,
    axum::extract::Path((resource_type, id)): axum::extract::Path<(String, Uuid)>,
) -> impl IntoResponse {
    // 尝试取资源名称（用于日志；失败不影响主流程）
    let resource_name = match state.recycle_repo.purge_name(&resource_type, id).await {
        Ok(name) => name,
        Err(e) => {
            tracing::warn!("purge name lookup failed for {}: {}", resource_type, e);
            None
        }
    };

    match state.recycle_repo.purge(&resource_type, id).await {
        Ok(true) => {
            // 永久删除：记录变更日志（purge）与审计日志（purge），与软删除（delete）区分
            let change = ChangeRecord {
                id: Uuid::new_v4(),
                actor_id: Some(auth.user_id),
                change_type: ChangeType::Purge,
                resource_type: resource_type.clone(),
                resource_id: Some(id),
                before_data: serde_json::json!({ "name": resource_name }),
                after_data: serde_json::json!(null),
                remarks: None,
                created_at: chrono::Utc::now(),
            };
            if let Err(e) = state.change_repo.create(&change).await {
                tracing::error!("failed to record purge change for {}: {}", resource_type, e);
            }
            // 审计日志由审计中间件统一记录（action=purge），此处不再重复写入
            (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "item not found in recycle bin".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("delete failed: {}", e) })).into_response(),
    }
}
