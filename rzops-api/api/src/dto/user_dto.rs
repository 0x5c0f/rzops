use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::role_dto::RoleResponse;

/// 用户响应（管理视图）
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub full_name: Option<String>,
    pub roles: Vec<RoleResponse>,
    pub created_at: DateTime<Utc>,
}

/// 用户列表响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserListResponse {
    pub data: Vec<UserResponse>,
    pub count: i64,
}

/// 用户列表查询参数
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListUsersQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// 创建用户请求
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
    pub is_superuser: Option<bool>,
    pub role_ids: Vec<Uuid>,
}

/// 更新用户请求（基础信息 + 角色关联全量替换；密码单独接口）
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
    pub is_superuser: Option<bool>,
    pub role_ids: Option<Vec<Uuid>>,
}

/// 重置密码请求
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ResetPasswordRequest {
    pub new_password: String,
}
