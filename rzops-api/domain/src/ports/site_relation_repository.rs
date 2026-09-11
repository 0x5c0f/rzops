use async_trait::async_trait;
use uuid::Uuid;
use crate::models::ops_site_relation::{OpsSiteServer, OpsSiteDatabase, OpsSiteDomain, SiteRefByServer, SiteRefByDatabase};

/// Site-Server relation repository.
#[async_trait]
pub trait SiteServerRelationRepository: Send + Sync {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteServer>, crate::errors::RepositoryError>;
    async fn find_sites_by_server(&self, server_id: Uuid) -> Result<Vec<SiteRefByServer>, crate::errors::RepositoryError>;
    async fn create(&self, item: &OpsSiteServer) -> Result<OpsSiteServer, crate::errors::RepositoryError>;
    async fn update(&self, id: Uuid, deploy_role: Option<String>) -> Result<bool, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}

/// Site-Database relation repository.
#[async_trait]
pub trait SiteDatabaseRelationRepository: Send + Sync {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDatabase>, crate::errors::RepositoryError>;
    async fn find_sites_by_database(&self, database_instance_id: Uuid) -> Result<Vec<SiteRefByDatabase>, crate::errors::RepositoryError>;
    async fn create(&self, item: &OpsSiteDatabase) -> Result<OpsSiteDatabase, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}

/// Site-Domain relation repository.
#[async_trait]
pub trait SiteDomainRelationRepository: Send + Sync {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDomain>, crate::errors::RepositoryError>;
    async fn create(&self, item: &OpsSiteDomain) -> Result<OpsSiteDomain, crate::errors::RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<bool, crate::errors::RepositoryError>;
}
