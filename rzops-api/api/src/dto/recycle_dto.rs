use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 回收站条目
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RecycleItem {
    /// 资源类型：server / datacenter / provider / domain / certificate / server_ip /
    /// server_port / server_port_template / ops_site / database_instance / backup_plan /
    /// monitor_target / attachment / contract / dict / user / role
    pub resource_type: String,
    pub id: Uuid,
    /// 展示名称（不同资源取对应名称字段）
    pub name: String,
    pub deleted_at: DateTime<Utc>,
}

/// 回收站列表响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RecycleListResponse {
    pub data: Vec<RecycleItem>,
    pub count: i64,
}

/// 回收站查询参数
#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListRecycleQuery {
    pub resource_type: Option<String>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// 恢复请求
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RestoreRequest {
    pub resource_type: String,
    pub id: Uuid,
}
