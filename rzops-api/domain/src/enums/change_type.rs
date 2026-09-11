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

#[cfg(test)]
mod tests {
    use super::ChangeType;

    #[test]
    fn serde_roundtrip_snake_case() {
        for (ty, expected) in [
            (ChangeType::Create, "create"),
            (ChangeType::Update, "update"),
            (ChangeType::StatusChange, "status_change"),
            (ChangeType::Delete, "delete"),
            (ChangeType::Bind, "bind"),
            (ChangeType::Unbind, "unbind"),
            (ChangeType::Purge, "purge"),
        ] {
            let s = serde_json::to_string(&ty).unwrap();
            assert_eq!(s, format!("\"{}\"", expected));
            let back: ChangeType = serde_json::from_str(&s).unwrap();
            assert_eq!(back, ty);
        }
    }

    #[test]
    fn purge_is_distinct_from_delete() {
        assert_ne!(ChangeType::Purge, ChangeType::Delete);
    }
}
