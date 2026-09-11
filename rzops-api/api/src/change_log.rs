//! 变更记录（ChangeRecord）写入辅助。
//! 供各资源写操作（create / update / delete）handler 调用，
//! 将操作记录写入 cmdb_change_record，供前端 change-records 页展示。

use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use rzops_domain::enums::ChangeType;
use rzops_domain::models::change_record::ChangeRecord;
use rzops_domain::ports::change_record_repository::ChangeRecordRepository;
use rzops_domain::ports::user_repository::UserRepository;

use crate::auth_extractor::AuthUser;

/// Handler 注入状态：写操作记录变更所需的依赖。
#[derive(Clone)]
pub struct ChangeLogState {
    pub change_repo: Arc<dyn ChangeRecordRepository>,
    #[allow(dead_code)]
    pub user_repo: Arc<dyn UserRepository>,
}

impl ChangeLogState {
    pub fn new(
        change_repo: Arc<dyn ChangeRecordRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self { change_repo, user_repo }
    }
}

/// 记录一条变更。失败只记日志，不阻塞主流程。
// 参数直接来自各 handler 的上下文（8 个），组合成 struct 会破坏 55+ 调用点的可读性；
// 该 lint 对内部日志辅助函数属于误报倾向场景，按 clippy 官方建议豁免。
#[allow(clippy::too_many_arguments)]
pub async fn record_change(
    state: &ChangeLogState,
    auth: &AuthUser,
    change_type: ChangeType,
    resource_type: &str,
    resource_id: Option<Uuid>,
    before: Value,
    after: Value,
    remarks: Option<String>,
) {
    let record = ChangeRecord {
        id: Uuid::new_v4(),
        actor_id: Some(auth.user_id),
        change_type,
        resource_type: resource_type.to_string(),
        resource_id,
        before_data: before,
        after_data: after,
        remarks,
        created_at: chrono::Utc::now(),
    };
    if let Err(e) = state.change_repo.create(&record).await {
        tracing::error!("failed to record change for {}: {}", resource_type, e);
    }
}
