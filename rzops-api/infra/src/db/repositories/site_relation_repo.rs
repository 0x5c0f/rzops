use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::models::ops_site_relation::{OpsSiteServer, OpsSiteDatabase, OpsSiteDomain, SiteRefByServer, SiteRefByDatabase};
use rzops_domain::ports::site_relation_repository::{SiteServerRelationRepository, SiteDatabaseRelationRepository, SiteDomainRelationRepository};

pub struct PgSiteRelationRepository { pool: Pool<Postgres> }
impl PgSiteRelationRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }


fn row_to_site_server(row: &sqlx::postgres::PgRow) -> OpsSiteServer {
    let role_str: Option<String> = row.get("deploy_role");
    OpsSiteServer { id: row.get("id"), site_id: row.get("site_id"), server_id: row.get("server_id"), deploy_role: role_str, created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

fn row_to_site_database(row: &sqlx::postgres::PgRow) -> OpsSiteDatabase {
    let usage_str: Option<String> = row.get("usage_type");
    OpsSiteDatabase { id: row.get("id"), site_id: row.get("site_id"), database_instance_id: row.get("database_instance_id"), usage_type: usage_str, created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

fn row_to_site_domain(row: &sqlx::postgres::PgRow) -> OpsSiteDomain {
    OpsSiteDomain { id: row.get("id"), site_id: row.get("site_id"), domain_id: row.get("domain_id"), domain_role: row.get("domain_role"), created_at: row.get::<DateTime<Utc>, _>("created_at") }
}

#[async_trait]
impl SiteServerRelationRepository for PgSiteRelationRepository {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteServer>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, site_id, server_id, deploy_role::text, created_at FROM cmdb_ops_site_server WHERE site_id=$1 ORDER BY created_at")
            .bind(site_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_site_server(r)).collect())
    }
    async fn find_sites_by_server(&self, server_id: Uuid) -> Result<Vec<SiteRefByServer>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT r.id AS relation_id, r.site_id, s.name AS site_name, r.deploy_role::text \
             FROM cmdb_ops_site_server r JOIN cmdb_ops_site s ON s.id = r.site_id AND s.deleted_at IS NULL \
             WHERE r.server_id=$1 ORDER BY s.name")
            .bind(server_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| SiteRefByServer {
            relation_id: r.get("relation_id"),
            site_id: r.get("site_id"),
            site_name: r.get("site_name"),
            deploy_role: r.get("deploy_role"),
        }).collect())
    }
    async fn create(&self, e: &OpsSiteServer) -> Result<OpsSiteServer, sqlx::Error> {
        Ok(row_to_site_server(&sqlx::query("INSERT INTO cmdb_ops_site_server (id,site_id,server_id,deploy_role,created_at) VALUES ($1,$2,$3,$4,$5) RETURNING id, site_id, server_id, deploy_role::text, created_at")
            .bind(e.id).bind(e.site_id).bind(e.server_id).bind(e.deploy_role.clone()).bind(e.created_at)
            .fetch_one(&self.pool).await?))
    }
    async fn update(&self, id: Uuid, deploy_role: Option<String>) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("UPDATE cmdb_ops_site_server SET deploy_role=$1 WHERE id=$2")
            .bind(deploy_role).bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_ops_site_server WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}

#[async_trait]
impl SiteDatabaseRelationRepository for PgSiteRelationRepository {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDatabase>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, site_id, database_instance_id, usage_type::text, created_at FROM cmdb_ops_site_database WHERE site_id=$1 ORDER BY created_at")
            .bind(site_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_site_database(r)).collect())
    }
    async fn find_sites_by_database(&self, database_instance_id: Uuid) -> Result<Vec<SiteRefByDatabase>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT r.site_id, s.name AS site_name, r.usage_type::text \
             FROM cmdb_ops_site_database r JOIN cmdb_ops_site s ON s.id = r.site_id AND s.deleted_at IS NULL \
             WHERE r.database_instance_id=$1 ORDER BY s.name")
            .bind(database_instance_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| SiteRefByDatabase {
            site_id: r.get("site_id"),
            site_name: r.get("site_name"),
            usage_type: r.get("usage_type"),
        }).collect())
    }
    async fn create(&self, e: &OpsSiteDatabase) -> Result<OpsSiteDatabase, sqlx::Error> {
        Ok(row_to_site_database(&sqlx::query("INSERT INTO cmdb_ops_site_database (id,site_id,database_instance_id,usage_type,created_at) VALUES ($1,$2,$3,$4,$5) RETURNING id, site_id, database_instance_id, usage_type::text, created_at")
            .bind(e.id).bind(e.site_id).bind(e.database_instance_id).bind(e.usage_type.clone()).bind(e.created_at)
            .fetch_one(&self.pool).await?))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_ops_site_database WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}

#[async_trait]
impl SiteDomainRelationRepository for PgSiteRelationRepository {
    async fn find_by_site(&self, site_id: Uuid) -> Result<Vec<OpsSiteDomain>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, site_id, domain_id, domain_role, created_at FROM cmdb_ops_site_domain WHERE site_id=$1 ORDER BY created_at")
            .bind(site_id).fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_site_domain(r)).collect())
    }
    async fn create(&self, e: &OpsSiteDomain) -> Result<OpsSiteDomain, sqlx::Error> {
        Ok(row_to_site_domain(&sqlx::query("INSERT INTO cmdb_ops_site_domain (id,site_id,domain_id,domain_role,created_at) VALUES ($1,$2,$3,$4,$5) RETURNING id, site_id, domain_id, domain_role, created_at")
            .bind(e.id).bind(e.site_id).bind(e.domain_id).bind(e.domain_role.clone()).bind(e.created_at)
            .fetch_one(&self.pool).await?))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_ops_site_domain WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}
