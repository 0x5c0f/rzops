use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

/// 数据字典项 — maps to cmdb_dict.
/// 用于维护各资源表单的可选项（类型 / 状态 / 架构等），
/// 替代原先硬编码的 Rust 枚举，支持运行时增删改。
#[derive(Debug, Clone)]
pub struct DictItem {
    pub id: Uuid,
    pub dict_type: String,
    pub dict_code: String,
    pub dict_label: String,
    pub sort_order: i32,
    pub enabled: bool,
    pub remark: Option<String>,
    pub extra_data: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
