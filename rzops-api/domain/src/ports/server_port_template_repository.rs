use async_trait::async_trait;
use uuid::Uuid;

use crate::models::server_port_template::ServerPortTemplate;

/// Filter criteria for listing server port templates.
#[derive(Debug, Clone, Default)]
pub struct ServerPortTemplateFilter {
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// ServerPortTemplate repository port.
#[async_trait]
pub trait ServerPortTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServerPortTemplate>, crate::errors::RepositoryError>;
    async fn find_all(&self, filter: ServerPortTemplateFilter) -> Result<Vec<ServerPortTemplate>, crate::errors::RepositoryError>;
    async fn count(&self, filter: ServerPortTemplateFilter) -> Result<i64, crate::errors::RepositoryError>;
    async fn create(&self, tpl: &ServerPortTemplate) -> Result<ServerPortTemplate, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, tpl: &ServerPortTemplate) -> Result<Option<ServerPortTemplate>, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
