use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::enums::{CredentialType, ReservedStatus};
use rzops_domain::models::credential::Credential;
use rzops_domain::ports::credential_repository::{CredentialFilter, CredentialRepository};

pub struct PgCredentialRepository { pool: Pool<Postgres> }
impl PgCredentialRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }

fn parse_cred_type(s: &str) -> CredentialType { match s { "password" => CredentialType::Password, "ssh_key" => CredentialType::SshKey, "api_token" => CredentialType::ApiToken, "certificate" => CredentialType::Certificate, _ => CredentialType::Other } }
fn cred_type_to_string(t: &CredentialType) -> String { match t { CredentialType::Password => "password", CredentialType::SshKey => "ssh_key", CredentialType::ApiToken => "api_token", CredentialType::Certificate => "certificate", CredentialType::Other => "other" }.to_string() }
fn parse_reserved_status(s: &str) -> ReservedStatus { match s { "draft" => ReservedStatus::Draft, "active" => ReservedStatus::Active, "inactive" => ReservedStatus::Inactive, "archived" => ReservedStatus::Archived, _ => ReservedStatus::Draft } }
fn reserved_status_to_string(s: &ReservedStatus) -> String { match s { ReservedStatus::Draft => "draft", ReservedStatus::Active => "active", ReservedStatus::Inactive => "inactive", ReservedStatus::Archived => "archived" }.to_string() }

fn row_to_entity(row: &sqlx::postgres::PgRow) -> Credential {
    Credential { id: row.get("id"), name: row.get("name"), credential_type: parse_cred_type(&row.get::<String, _>("credential_type")), username: row.get("username"), secret_ref: row.get("secret_ref"), owner_id: row.get("owner_id"), status: parse_reserved_status(&row.get::<String, _>("status")), remarks: row.get("remarks"), created_at: row.get::<DateTime<Utc>, _>("created_at"), updated_at: row.get::<DateTime<Utc>, _>("updated_at") }
}

const COLS: &str = "id, name, credential_type::text, username, secret_ref, owner_id, status::text, remarks, created_at, updated_at";

#[async_trait]
impl CredentialRepository for PgCredentialRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Credential>, sqlx::Error> {
        Ok(sqlx::query(&format!("SELECT {} FROM cmdb_credential WHERE id=$1", COLS)).bind(id).fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn find_all(&self, f: CredentialFilter) -> Result<Vec<Credential>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_credential WHERE 1=1", COLS);
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_type = f.credential_type.as_ref();
        let s_q = f.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_type.is_some() { sql.push_str(&format!(" AND credential_type::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(l) = f.limit { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = f.offset { sql.push_str(&format!(" OFFSET {}", o)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(t) = s_type { query = query.bind(t); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_all(&self.pool).await?.iter().map(|r| row_to_entity(r)).collect())
    }
    async fn count(&self, f: CredentialFilter) -> Result<i64, sqlx::Error> {
        let mut sql = "SELECT COUNT(*) as count FROM cmdb_credential WHERE 1=1".to_string();
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_type = f.credential_type.as_ref();
        let s_q = f.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_type.is_some() { sql.push_str(&format!(" AND credential_type::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(t) = s_type { query = query.bind(t); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_one(&self.pool).await?.get::<i64, _>("count"))
    }
    async fn create(&self, e: &Credential) -> Result<Credential, sqlx::Error> {
        Ok(row_to_entity(&sqlx::query(&format!("INSERT INTO cmdb_credential (id,name,credential_type,username,secret_ref,owner_id,status,remarks,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING {}", COLS))
            .bind(e.id).bind(&e.name).bind(cred_type_to_string(&e.credential_type)).bind(&e.username).bind(&e.secret_ref).bind(e.owner_id).bind(reserved_status_to_string(&e.status)).bind(&e.remarks).bind(e.created_at).bind(e.updated_at)
            .fetch_one(&self.pool).await?))
    }
    async fn update(&self, id: Uuid, e: &Credential) -> Result<Option<Credential>, sqlx::Error> {
        Ok(sqlx::query(&format!("UPDATE cmdb_credential SET name=$2,credential_type=$3,username=$4,secret_ref=$5,owner_id=$6,status=$7,remarks=$8,updated_at=$9 WHERE id=$1 RETURNING {}", COLS))
            .bind(id).bind(&e.name).bind(cred_type_to_string(&e.credential_type)).bind(&e.username).bind(&e.secret_ref).bind(e.owner_id).bind(reserved_status_to_string(&e.status)).bind(&e.remarks).bind(e.updated_at)
            .fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_credential WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}
