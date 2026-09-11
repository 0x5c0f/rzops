use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request body for creating a server port (每台服务器独立端口).
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateServerPortRequest {
    /// 所属服务器 id（一对多，必填）
    pub server_id: Uuid,
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: Option<bool>,
    pub description: Option<String>,
}

/// Request body for updating a server port.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateServerPortRequest {
    pub server_id: Option<Uuid>,
    pub protocol: Option<String>,
    pub port: Option<i32>,
    pub service_name: Option<String>,
    pub access_scope: Option<String>,
    pub is_enabled: Option<bool>,
    pub description: Option<String>,
}

/// Query parameters for listing server ports.
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListServerPortsQuery {
    pub server_id: Option<Uuid>,
    pub protocol: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Response body for a server port.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerPortResponse {
    pub id: Uuid,
    pub server_id: Uuid,
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// 所属服务器名称（聚合展示）
    pub server_name: Option<String>,
    /// 所属服务器状态（聚合展示）
    pub server_status: Option<String>,
}

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerPortListResponse {
    pub data: Vec<ServerPortResponse>,
    pub count: i64,
}

/// 从模板批量实例化端口到指定服务器的请求体.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ApplyTemplateRequest {
    pub template_id: Uuid,
    pub server_ids: Vec<Uuid>,
}
