use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::models::attachment::Attachment;
use rzops_domain::ports::attachment_repository::{AttachmentFilter, AttachmentRepository};

pub struct PgAttachmentRepository { pool: Pool<Postgres> }
impl PgAttachmentRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }


fn row_to_entity(row: &sqlx::postgres::PgRow) -> Attachment {
    Attachment { id: row.get("id"), filename: row.get("filename"), target_type: row.get("target_type"), target_id: row.get("target_id"), storage_key: row.get("storage_key"), content_type: row.get("content_type"), size_bytes: row.get("size_bytes"), uploaded_by_id: row.get("uploaded_by_id"), status: row.get::<String, _>("status"), remarks: row.get("remarks"), created_at: row.get::<DateTime<Utc>, _>("created_at"), updated_at: row.get::<DateTime<Utc>, _>("updated_at") }
}

const COLS: &str = "id, filename, target_type, target_id, storage_key, content_type, size_bytes, uploaded_by_id, status::text, remarks, created_at, updated_at";

#[async_trait]
impl AttachmentRepository for PgAttachmentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Attachment>, sqlx::Error> {
        Ok(sqlx::query(&format!("SELECT {} FROM cmdb_attachment WHERE id=$1", COLS)).bind(id).fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn find_all(&self, f: AttachmentFilter) -> Result<Vec<Attachment>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_attachment WHERE 1=1", COLS);
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(ref s) = f.status { sql.push_str(&format!(" AND status::text = ${}", idx)); string_binds.push(s.clone()); idx += 1; }
        if let Some(ref tt) = f.target_type { sql.push_str(&format!(" AND target_type = ${}", idx)); string_binds.push(tt.clone()); idx += 1; }
        if let Some(tid) = f.target_id { sql.push_str(&format!(" AND target_id = ${}", idx)); uuid_binds.push(tid); idx += 1; }
        if let Some(ref q) = f.q { sql.push_str(&format!(" AND filename ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(l) = f.limit { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = f.offset { sql.push_str(&format!(" OFFSET {}", o)); }
        let mut query = sqlx::query(&sql);
        for s in &string_binds { query = query.bind(s); }
        for u in &uuid_binds { query = query.bind(u); }
        Ok(query.fetch_all(&self.pool).await?.iter().map(|r| row_to_entity(r)).collect())
    }
    async fn count(&self, f: AttachmentFilter) -> Result<i64, sqlx::Error> {
        let mut sql = "SELECT COUNT(*) as count FROM cmdb_attachment WHERE 1=1".to_string();
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(ref s) = f.status { sql.push_str(&format!(" AND status::text = ${}", idx)); string_binds.push(s.clone()); idx += 1; }
        if let Some(ref tt) = f.target_type { sql.push_str(&format!(" AND target_type = ${}", idx)); string_binds.push(tt.clone()); idx += 1; }
        if let Some(tid) = f.target_id { sql.push_str(&format!(" AND target_id = ${}", idx)); uuid_binds.push(tid); idx += 1; }
        if let Some(ref q) = f.q { sql.push_str(&format!(" AND filename ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }
        let mut query = sqlx::query(&sql);
        for s in &string_binds { query = query.bind(s); }
        for u in &uuid_binds { query = query.bind(u); }
        Ok(query.fetch_one(&self.pool).await?.get::<i64, _>("count"))
    }
    async fn create(&self, e: &Attachment) -> Result<Attachment, sqlx::Error> {
        Ok(row_to_entity(&sqlx::query(&format!("INSERT INTO cmdb_attachment (id,filename,target_type,target_id,storage_key,content_type,size_bytes,uploaded_by_id,status,remarks,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) RETURNING {}", COLS))
            .bind(e.id).bind(&e.filename).bind(&e.target_type).bind(e.target_id).bind(&e.storage_key).bind(&e.content_type).bind(e.size_bytes).bind(e.uploaded_by_id).bind(e.status.clone()).bind(&e.remarks).bind(e.created_at).bind(e.updated_at)
            .fetch_one(&self.pool).await?))
    }
    async fn update(&self, id: Uuid, e: &Attachment) -> Result<Option<Attachment>, sqlx::Error> {
        Ok(sqlx::query(&format!("UPDATE cmdb_attachment SET filename=$2,target_type=$3,target_id=$4,storage_key=$5,content_type=$6,size_bytes=$7,uploaded_by_id=$8,status=$9,remarks=$10,updated_at=$11 WHERE id=$1 RETURNING {}", COLS))
            .bind(id).bind(&e.filename).bind(&e.target_type).bind(e.target_id).bind(&e.storage_key).bind(&e.content_type).bind(e.size_bytes).bind(e.uploaded_by_id).bind(e.status.clone()).bind(&e.remarks).bind(e.updated_at)
            .fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_attachment WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}
