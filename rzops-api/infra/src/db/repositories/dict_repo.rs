use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::dict::DictItem;
use rzops_domain::ports::dict_repository::DictRepository;

/// PostgreSQL implementation of DictRepository.
pub struct PgDictRepository {
    pool: Pool<Postgres>,
}

impl PgDictRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

const SELECT_COLS: &str = r#"id, dict_type, dict_code, dict_label, sort_order, enabled, remark, created_at, updated_at"#;

fn row_to_dict(row: &sqlx::postgres::PgRow) -> DictItem {
    DictItem {
        id: row.get("id"),
        dict_type: row.get("dict_type"),
        dict_code: row.get("dict_code"),
        dict_label: row.get("dict_label"),
        sort_order: row.get("sort_order"),
        enabled: row.get("enabled"),
        remark: row.get("remark"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[async_trait]
impl DictRepository for PgDictRepository {
    async fn list_by_type(&self, dict_type: &str, enabled_only: bool) -> Result<Vec<DictItem>, String> {
        let sql = if enabled_only {
            format!(
                "SELECT {} FROM cmdb_dict WHERE dict_type = $1 AND enabled = TRUE ORDER BY sort_order ASC, dict_label ASC",
                SELECT_COLS
            )
        } else {
            format!(
                "SELECT {} FROM cmdb_dict WHERE dict_type = $1 ORDER BY sort_order ASC, dict_label ASC",
                SELECT_COLS
            )
        };
        sqlx::query(&sql)
            .bind(dict_type)
            .fetch_all(&self.pool)
            .await
            .map(|rows| rows.iter().map(row_to_dict).collect())
            .map_err(|e| e.to_string())
    }

    async fn list_all(&self) -> Result<Vec<DictItem>, String> {
        let sql = format!(
            "SELECT {} FROM cmdb_dict ORDER BY dict_type ASC, sort_order ASC, dict_label ASC",
            SELECT_COLS
        );
        sqlx::query(&sql)
            .fetch_all(&self.pool)
            .await
            .map(|rows| rows.iter().map(row_to_dict).collect())
            .map_err(|e| e.to_string())
    }

    async fn list_all_enabled(&self) -> Result<Vec<DictItem>, String> {
        let sql = format!(
            "SELECT {} FROM cmdb_dict WHERE enabled = TRUE ORDER BY dict_type ASC, sort_order ASC, dict_label ASC",
            SELECT_COLS
        );
        sqlx::query(&sql)
            .fetch_all(&self.pool)
            .await
            .map(|rows| rows.iter().map(row_to_dict).collect())
            .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<DictItem>, String> {
        let sql = format!("SELECT {} FROM cmdb_dict WHERE id = $1", SELECT_COLS);
        let row = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(row.map(|r| row_to_dict(&r)))
    }

    async fn create(&self, item: &DictItem) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO cmdb_dict (id, dict_type, dict_code, dict_label, sort_order, enabled, remark, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        )
        .bind(item.id)
        .bind(&item.dict_type)
        .bind(&item.dict_code)
        .bind(&item.dict_label)
        .bind(item.sort_order)
        .bind(item.enabled)
        .bind(&item.remark)
        .bind(item.created_at)
        .bind(item.updated_at)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
    }

    async fn update(&self, id: Uuid, label: &str, sort_order: i32, enabled: bool, remark: Option<&str>) -> Result<(), String> {
        sqlx::query(
            r#"UPDATE cmdb_dict SET dict_label = $2, sort_order = $3, enabled = $4, remark = $5, updated_at = NOW() WHERE id = $1"#,
        )
        .bind(id)
        .bind(label)
        .bind(sort_order)
        .bind(enabled)
        .bind(remark)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        // 软删：停用，保留历史数据可查 label
        sqlx::query("UPDATE cmdb_dict SET enabled = FALSE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn exists(&self, dict_type: &str, code: &str) -> Result<bool, String> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM cmdb_dict WHERE dict_type = $1 AND dict_code = $2 AND enabled = TRUE) AS ok",
        )
        .bind(dict_type)
        .bind(code)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row.get::<bool, _>("ok"))
    }
}
