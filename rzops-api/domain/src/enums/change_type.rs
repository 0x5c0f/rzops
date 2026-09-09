use serde::{Deserialize, Serialize};

/// 变更动作类型 — 系统内部行为标记（变更记录/审计用），
/// 不是用户可配置的表单选项，因此不作为数据字典维护。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Create,
    Update,
    StatusChange,
    Delete,
    Bind,
    Unbind,
    /// 永久删除（回收站彻底清除）
    Purge,
}
