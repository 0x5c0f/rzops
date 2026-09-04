use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 角色响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RoleResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_builtin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建角色请求
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateRoleRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

/// 更新角色请求（名称/描述/启用/权限点全量替换）
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub permissions: Option<Vec<String>>,
}

/// 角色列表响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RoleListResponse {
    pub data: Vec<RoleResponse>,
    pub count: i64,
}

/// 角色权限视图：单个角色 + 其权限点
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RoleDetailResponse {
    pub role: RoleResponse,
    pub permissions: Vec<String>,
}
