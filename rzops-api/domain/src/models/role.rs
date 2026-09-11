use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 角色 — maps to role 表。
/// 预置角色（admin/ops/audit/viewer）is_builtin=true，不可删除；
/// 自定义角色由管理员创建并勾选权限点。
#[derive(Debug, Clone)]
pub struct Role {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_builtin: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
