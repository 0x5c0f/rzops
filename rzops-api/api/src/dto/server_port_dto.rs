use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 关联服务器简要信息（用于响应展示）
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerBrief {
    pub id: Uuid,
    pub name: String,
    /// 服务器状态（用于详情页状态展示）
    pub status: String,
}

/// Request body for creating a server port.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateServerPortRequest {
    /// 关联的服务器 id 列表（多对多，至少一个）
    pub server_ids: Vec<Uuid>,
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
    pub server_ids: Option<Vec<Uuid>>,
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
    pub protocol: String,
    pub port: i32,
    pub service_name: String,
    pub access_scope: Option<String>,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// 关联的服务器 id 列表
    pub server_ids: Vec<Uuid>,
    /// 关联的服务器简要信息（名称）
    pub servers: Vec<ServerBrief>,
}

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ServerPortListResponse {
    pub data: Vec<ServerPortResponse>,
    pub count: i64,
}
