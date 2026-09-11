use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// 回收站条目（软删除数据的快照视图）。
#[derive(Debug, Clone, Serialize)]
pub struct RecycleEntry {
    /// 资源类型：server / datacenter / provider / domain / certificate / server_ip /
    /// server_port / server_port_template / ops_site / database_instance / backup_plan /
    /// monitor_target / attachment / contract / dict / user / role
    pub resource_type: String,
    pub id: Uuid,
    /// 展示名称（不同资源取对应名称字段）
    pub name: String,
    pub deleted_at: DateTime<Utc>,
    /// 删除前的完整数据快照（该表整行转 JSON）
    pub data: Option<serde_json::Value>,
}
