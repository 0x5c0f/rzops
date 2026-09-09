use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

/// 资源名称解析服务。
///
/// 用于把 `resource_type + resource_id` 展示为可读名称（审计/变更日志、
/// 备份/监控目标、附件上传者等）。所有 SQL 位于 `infra` 实现，
/// 领域层仅定义契约。
#[async_trait]
pub trait ResourceNameService: Send + Sync {
    /// 批量解析资源名称，返回 `(type, id) -> name` 映射；无法解析的键不出现。
    async fn resolve_resource_names(
        &self,
        items: &[(String, Uuid)],
    ) -> HashMap<(String, Uuid), String>;

    /// 批量解析服务器名称 + 状态，返回 `id -> (name, status)`。
    async fn resolve_server_briefs(&self, ids: &[Option<Uuid>]) -> HashMap<Uuid, (String, String)>;

    /// 解析单个关联目标名称（backup_plan / monitor_target / attachment 等）。
    async fn resolve_target_name(&self, target_type: &str, target_id: Uuid) -> Option<String>;

    /// 解析用户显示名（full_name）。
    async fn resolve_user_name(&self, user_id: Uuid) -> Option<String>;

    /// 批量解析用户邮箱，返回 `id -> email`。
    async fn resolve_user_emails(&self, ids: &[Option<Uuid>]) -> HashMap<Uuid, String>;
}
