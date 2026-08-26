use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::enums::{SiteDatabaseUsage, SiteServerRole};
use rzops_domain::models::ops_site_relation::{OpsSiteServer, OpsSiteDatabase, OpsSiteDomain};
use rzops_domain::ports::site_relation_repository::{SiteServerRelationRepository, SiteDatabaseRelationRepository, SiteDomainRelationRepository};

pub struct PgSiteRelationRepository { pool: Pool<Postgres> }
impl PgSiteRelationRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }

fn parse_site_server_role(s: &str) -> SiteServerRole { match s { "web" => SiteServerRole::Web, "api" => SiteServerRole::Api, "worker" => SiteServerRole::Worker, "static" => SiteServerRole::Static, _ => SiteServerRole::Other } }
fn site_server_role_to_string(r: &SiteServerRole) -> String { match r { SiteServerRole::Web => "web", SiteServerRole::Api => "api", SiteServerRole::Worker => "worker", SiteServerRole::Static => "static", SiteServerRole::Other => "other" }.to_string() }
fn parse_site_db_usage(s: &str) -> SiteDatabaseUsage { match s { "primary" => SiteDatabaseUsage::Primary, "replica" => SiteDatabaseUsage::Replica, "analytics" => SiteDatabaseUsage::Analytics, "archive" => SiteDatabaseUsage::Archive, _ => SiteDatabaseUsage::Other } }

fn row_to_site_server(row: &sqlx::postgres::PgRow) -> OpsSiteServer {
    let role_str: Option<String> = row.get("deploy_role");
    OpsSiteServer { id: row.get("id"), site_id: row.get("site_id"), server_id: row.get("server_id"), deploy_role: role_str.map(|s| parse_site_server_role(&s)), is_primary: row.get("is_primary"), created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

fn row_to_site_database(row: &sqlx::postgres::PgRow) -> OpsSiteDatabase {
    let usage_str: Option<String> = row.get("usage_type");
    OpsSiteDatabase { id: row.get("id"), site_id: row.get("site_id"), database_instance_id: row.get("database_instance_id"), usage_type: usage_str.map(|s| parse_site_db_usage(&s)), is_primary: row.get("is_primary"), created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

fn row_to_site_domain(row: &sqlx::postgres::PgRow) -> OpsSiteDomain {
    OpsSiteDomain { id: row.get("id"), site_id: row.get("site_id"), domain_id: row.get("domain_id"), is_primary: row.get("is_primary"), created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

#[async_trait]
impl SiteServerRelationRepository for PgSiteRelationRepository {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteServer>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, site_id, server_id, deploy_role::text, is_primary, created_at FROM cmdb_ops_site_server WHERE site_id=$1 ORDER BY created_at")
            .bind(site_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_site_server(r)).collect())
    }
    async fn create(&self, e: &OpsSiteServer) -> Result<OpsSiteServer, sqlx::Error> {
        Ok(row_to_site_server(&sqlx::query("INSERT INTO cmdb_ops_site_server (id,site_id,server_id,deploy_role,is_primary,created_at) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id, site_id, server_id, deploy_role::text, is_primary, created_at")
            .bind(e.id).bind(e.site_id).bind(e.server_id).bind(e.deploy_role.as_ref().map(site_server_role_to_string)).bind(e.is_primary).bind(e.created_at)
            .fetch_one(&self.pool).await?))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_ops_site_server WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}

#[async_trait]
impl SiteDatabaseRelationRepository for PgSiteRelationRepository {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDatabase>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, site_id, database_instance_id, usage_type::text, is_primary, created_at FROM cmdb_ops_site_database WHERE site_id=$1 ORDER BY created_at")
            .bind(site_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_site_database(r)).collect())
    }
    async fn create(&self, e: &OpsSiteDatabase) -> Result<OpsSiteDatabase, sqlx::Error> {
        Ok(row_to_site_database(&sqlx::query("INSERT INTO cmdb_ops_site_database (id,site_id,database_instance_id,usage_type,is_primary,created_at) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id, site_id, database_instance_id, usage_type::text, is_primary, created_at")
            .bind(e.id).bind(e.site_id).bind(e.database_instance_id).bind(e.usage_type.as_ref().map(|u| match u { SiteDatabaseUsage::Primary => "primary", SiteDatabaseUsage::Replica => "replica", SiteDatabaseUsage::Analytics => "analytics", SiteDatabaseUsage::Archive => "archive", SiteDatabaseUsage::Other => "other" }.to_string())).bind(e.is_primary).bind(e.created_at)
            .fetch_one(&self.pool).await?))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_ops_site_database WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}

#[async_trait]
impl SiteDomainRelationRepository for PgSiteRelationRepository {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDomain>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, site_id, domain_id, is_primary, created_at FROM cmdb_ops_site_domain WHERE site_id=$1 ORDER BY created_at")
            .bind(site_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_site_domain(r)).collect())
    }
    async fn create(&self, e: &OpsSiteDomain) -> Result<OpsSiteDomain, sqlx::Error> {
        Ok(row_to_site_domain(&sqlx::query("INSERT INTO cmdb_ops_site_domain (id,site_id,domain_id,is_primary,created_at) VALUES ($1,$2,$3,$4,$5) RETURNING id, site_id, domain_id, is_primary, created_at")
            .bind(e.id).bind(e.site_id).bind(e.domain_id).bind(e.is_primary).bind(e.created_at)
            .fetch_one(&self.pool).await?))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_ops_site_domain WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}
