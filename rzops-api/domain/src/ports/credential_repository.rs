use async_trait::async_trait;
use uuid::Uuid;
use crate::models::credential::Credential;

#[derive(Debug, Clone, Default)]
pub struct CredentialFilter {
    pub status: Option<String>,
    pub credential_type: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[async_trait]
pub trait CredentialRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Credential>, sqlx::Error>;
    async fn find_all(&self, filter: CredentialFilter) -> Result<Vec<Credential>, sqlx::Error>;
    async fn count(&self, filter: CredentialFilter) -> Result<i64, sqlx::Error>;
    async fn create(&self, item: &Credential) -> Result<Credential, sqlx::Error>;
    async fn update(&self, id: Uuid, item: &Credential) -> Result<Option<Credential>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
