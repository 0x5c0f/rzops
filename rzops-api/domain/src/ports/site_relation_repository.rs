use async_trait::async_trait;
use uuid::Uuid;
use crate::models::ops_site_relation::{OpsSiteServer, OpsSiteDatabase, OpsSiteDomain, SiteRefByServer, SiteRefByDatabase};

/// Site-Server relation repository.
#[async_trait]
pub trait SiteServerRelationRepository: Send + Sync {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteServer>, sqlx::Error>;
    async fn find_sites_by_server(&self, server_id: Uuid) -> Result<Vec<SiteRefByServer>, sqlx::Error>;
    async fn create(&self, item: &OpsSiteServer) -> Result<OpsSiteServer, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}

/// Site-Database relation repository.
#[async_trait]
pub trait SiteDatabaseRelationRepository: Send + Sync {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDatabase>, sqlx::Error>;
    async fn find_sites_by_database(&self, database_instance_id: Uuid) -> Result<Vec<SiteRefByDatabase>, sqlx::Error>;
    async fn create(&self, item: &OpsSiteDatabase) -> Result<OpsSiteDatabase, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}

/// Site-Domain relation repository.
#[async_trait]
pub trait SiteDomainRelationRepository: Send + Sync {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDomain>, sqlx::Error>;
    async fn create(&self, item: &OpsSiteDomain) -> Result<OpsSiteDomain, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}
