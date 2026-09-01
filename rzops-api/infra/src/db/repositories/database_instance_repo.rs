use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::database_instance::DatabaseInstance;
use rzops_domain::ports::database_instance_repository::{DatabaseInstanceFilter, DatabaseInstanceRepository};

pub struct PgDatabaseInstanceRepository {
    pool: Pool<Postgres>,
}

impl PgDatabaseInstanceRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}






fn row_to_database_instance(row: &sqlx::postgres::PgRow) -> DatabaseInstance {
    let status_str: String = row.get("status");
    let type_str: String = row.get("db_type");
    let importance_str: Option<String> = row.get("importance");
    DatabaseInstance {
        id: row.get("id"),
        server_id: row.get("server_id"),
        name: row.get("name"),
        db_type: type_str,
        description: row.get("description"),
        status: status_str,
        offline_time: row.get("offline_time"),
        is_self_installed: row.get("is_self_installed"),
        importance: importance_str,
        is_ops_managed: row.get("is_ops_managed"),
        port: row.get("port"),
        instance_name: row.get("instance_name"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

const SELECT_COLS: &str = r#"id, server_id, name, db_type::text, description, status::text,
    offline_time, is_self_installed, importance::text, is_ops_managed,
    port, instance_name,
    created_at, updated_at, deleted_at"#;

#[async_trait]
impl DatabaseInstanceRepository for PgDatabaseInstanceRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DatabaseInstance>, sqlx::Error> {
        let row = sqlx::query(&format!("SELECT {} FROM cmdb_database_instance WHERE id = $1 AND deleted_at IS NULL", SELECT_COLS))
            .bind(id).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_database_instance(&r)))
    }

    async fn find_all(&self, filter: DatabaseInstanceFilter) -> Result<Vec<DatabaseInstance>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_database_instance WHERE deleted_at IS NULL", SELECT_COLS);
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(ref status) = filter.status { sql.push_str(&format!(" AND status::text = ${}", idx)); string_binds.push(status.clone()); idx += 1; }
        if let Some(ref db_type) = filter.db_type { sql.push_str(&format!(" AND db_type::text = ${}", idx)); string_binds.push(db_type.clone()); idx += 1; }
        if let Some(sid) = filter.server_id { sql.push_str(&format!(" AND server_id = ${}", idx)); uuid_binds.push(sid); idx += 1; }
        if let Some(ref q) = filter.q { sql.push_str(&format!(" AND name ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }
        sql.push_str(" ORDER BY created_at DESC");
        if let Some(limit) = filter.limit { sql.push_str(&format!(" LIMIT {}", limit)); }
        if let Some(offset) = filter.offset { sql.push_str(&format!(" OFFSET {}", offset)); }
        let mut query = sqlx::query(&sql);
        for s in &string_binds { query = query.bind(s); }
        for u in &uuid_binds { query = query.bind(u); }
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_database_instance(r)).collect())
    }

    async fn count(&self, filter: DatabaseInstanceFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_database_instance WHERE deleted_at IS NULL");
        let mut string_binds: Vec<String> = Vec::new();
        let mut uuid_binds: Vec<Uuid> = Vec::new();
        let mut idx = 1;
        if let Some(ref status) = filter.status { sql.push_str(&format!(" AND status::text = ${}", idx)); string_binds.push(status.clone()); idx += 1; }
        if let Some(ref db_type) = filter.db_type { sql.push_str(&format!(" AND db_type::text = ${}", idx)); string_binds.push(db_type.clone()); idx += 1; }
        if let Some(sid) = filter.server_id { sql.push_str(&format!(" AND server_id = ${}", idx)); uuid_binds.push(sid); idx += 1; }
        if let Some(ref q) = filter.q { sql.push_str(&format!(" AND name ILIKE ${}", idx)); string_binds.push(format!("%{}%", q)); }
        let mut query = sqlx::query(&sql);
        for s in &string_binds { query = query.bind(s); }
        for u in &uuid_binds { query = query.bind(u); }
        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, db: &DatabaseInstance) -> Result<DatabaseInstance, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"INSERT INTO cmdb_database_instance
               (id, server_id, name, db_type, description, status, offline_time,
                is_self_installed, importance, is_ops_managed,
                port, instance_name, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
               RETURNING {}"#, SELECT_COLS
        ))
        .bind(db.id).bind(db.server_id).bind(&db.name).bind(db.db_type.clone())
        .bind(&db.description).bind(db.status.clone()).bind(db.offline_time)
        .bind(db.is_self_installed).bind(db.importance.clone())
        .bind(db.is_ops_managed).bind(db.port).bind(&db.instance_name)
        .bind(db.created_at).bind(db.updated_at)
        .fetch_one(&self.pool).await?;
        Ok(row_to_database_instance(&row))
    }

    async fn update(&self, id: Uuid, db: &DatabaseInstance) -> Result<Option<DatabaseInstance>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"UPDATE cmdb_database_instance SET
                server_id=$2, name=$3, db_type=$4, description=$5, status=$6,
                offline_time=$7, is_self_installed=$8, importance=$9, is_ops_managed=$10,
                port=$11, instance_name=$12, updated_at=$13
               WHERE id=$1 AND deleted_at IS NULL RETURNING {}"#, SELECT_COLS
        ))
        .bind(id).bind(db.server_id).bind(&db.name).bind(db.db_type.clone())
        .bind(&db.description).bind(db.status.clone()).bind(db.offline_time)
        .bind(db.is_self_installed).bind(db.importance.clone())
        .bind(db.is_ops_managed).bind(db.port).bind(&db.instance_name).bind(db.updated_at)
        .fetch_optional(&self.pool).await?;
        Ok(row.map(|r| row_to_database_instance(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE cmdb_database_instance SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL").bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }
}
