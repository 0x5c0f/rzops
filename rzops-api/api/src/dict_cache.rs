//! 字典进程内 TTL 缓存
//!
//! 针对「查询全部启用字典项」（前端 loadAllDicts 的 `enabled_only=true&per_page=1000`）
//! 做内存缓存，避免每次刷新页面都全表扫 DB。
//! - TTL 兜底：缓存过期后自动重查
//! - 主动失效：字典 create/update/delete 后立即 invalidate，保证跨端实时一致
//! - 内存占用极小（当前 187 条 ≈ 0.1 MB），随字典规模线性增长

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

use rzops_domain::models::dict::DictItem;
use rzops_domain::ports::dict_repository::DictRepository;

pub struct DictCache {
    inner: RwLock<Option<(Instant, Vec<DictItem>)>>,
    ttl: Duration,
}

impl DictCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: RwLock::new(None),
            ttl,
        }
    }

    /// 使缓存失效（字典增删改后调用，下次查询重新从 DB 加载）
    pub async fn invalidate(&self) {
        *self.inner.write().await = None;
    }

    /// 获取全部启用字典项；优先走缓存，未命中/过期时从 DB 加载并填充
    pub async fn get_enabled(&self, repo: &Arc<dyn DictRepository>) -> Result<Vec<DictItem>, String> {
        {
            let guard = self.inner.read().await;
            if let Some((ts, items)) = guard.as_ref() {
                if ts.elapsed() < self.ttl {
                    return Ok(items.clone());
                }
            }
        }
        let items = repo.list_all_enabled().await?;
        let mut guard = self.inner.write().await;
        // double-check：并发场景下可能已被其他请求填充
        if guard
            .as_ref()
            .map(|(ts, _)| ts.elapsed() >= self.ttl)
            .unwrap_or(true)
        {
            *guard = Some((Instant::now(), items.clone()));
        }
        Ok(items)
    }
}
