use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateAttachmentRequest {
    pub filename: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub storage_key: Option<String>,
    pub content_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by_id: Option<Uuid>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListAttachmentsQuery {
    pub status: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AttachmentResponse {
    pub id: Uuid,
    pub filename: String,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub storage_key: Option<String>,
    pub content_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by_id: Option<Uuid>,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AttachmentListResponse {
    pub data: Vec<AttachmentResponse>,
    pub count: i64,
}
