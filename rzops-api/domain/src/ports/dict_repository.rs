use uuid::Uuid;
use async_trait::async_trait;

use crate::models::dict::DictItem;

/// 数据字典仓储 — 维护各资源表单可选项（字典）。
#[async_trait]
pub trait DictRepository: Send + Sync {
    /// 查询某类型的全部字典项（可按 enabled 过滤）
    async fn list_by_type(&self, dict_type: &str, enabled_only: bool) -> Result<Vec<DictItem>, String>;

    /// 查询全部字典项（管理页用，按类型 + 排序，含停用项）
    async fn list_all(&self) -> Result<Vec<DictItem>, String>;

    /// 查询全部启用字典项（业务表单下拉用）
    async fn list_all_enabled(&self) -> Result<Vec<DictItem>, String>;

    /// 按 id 查询
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DictItem>, String>;

    /// 新增
    async fn create(&self, item: &DictItem) -> Result<(), String>;

    /// 更新（label / sort_order / enabled / remark / extra_data）
    async fn update(&self, id: Uuid, label: &str, sort_order: i32, enabled: bool, remark: Option<&str>, extra_data: Option<&serde_json::Value>) -> Result<(), String>;

    /// 删除（软删：enabled = false；历史数据 label 不丢）
    async fn delete(&self, id: Uuid) -> Result<(), String>;

    /// 校验某个 code 在该类型下是否存在且启用（写业务数据时的可选一致性校验）
    async fn exists(&self, dict_type: &str, code: &str) -> Result<bool, String>;
}
