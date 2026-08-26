use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::enums::{CommonStatus, LineType};
use rzops_domain::models::data_center::DataCenter;
use rzops_domain::ports::datacenter_repository::{DataCenterFilter, DataCenterRepository};

/// PostgreSQL implementation of DataCenterRepository.
pub struct PgDataCenterRepository {
    pool: Pool<Postgres>,
}

impl PgDataCenterRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn parse_line_type(s: &str) -> LineType {
    match s {
        "single_line" => LineType::SingleLine,
        "dual_line" => LineType::DualLine,
        "multi_line" => LineType::MultiLine,
        _ => LineType::Other,
    }
}

fn line_type_to_string(lt: &LineType) -> String {
    match lt {
        LineType::SingleLine => "single_line".to_string(),
        LineType::DualLine => "dual_line".to_string(),
        LineType::MultiLine => "multi_line".to_string(),
        LineType::Other => "other".to_string(),
    }
}

fn parse_common_status(s: &str) -> CommonStatus {
    match s {
        "active" => CommonStatus::Active,
        "inactive" => CommonStatus::Inactive,
        "archived" => CommonStatus::Archived,
        _ => CommonStatus::Active,
    }
}

fn common_status_to_string(s: &CommonStatus) -> String {
    match s {
        CommonStatus::Active => "active".to_string(),
        CommonStatus::Inactive => "inactive".to_string(),
        CommonStatus::Archived => "archived".to_string(),
    }
}

fn row_to_datacenter(row: &sqlx::postgres::PgRow) -> DataCenter {
    let status_str: String = row.get("status");
    let line_type_str: Option<String> = row.get("line_type");

    DataCenter {
        id: row.get("id"),
        name: row.get("name"),
        provider_id: row.get("provider_id"),
        phone: row.get("phone"),
        address: row.get("address"),
        country: row.get("country"),
        province: row.get("province"),
        city: row.get("city"),
        line_type: line_type_str.map(|s| parse_line_type(&s)),
        description: row.get("description"),
        status: parse_common_status(&status_str),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

#[async_trait]
impl DataCenterRepository for PgDataCenterRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DataCenter>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT id, name, provider_id, phone, address, country, province, city,
                      line_type::text, description, status::text, created_at, updated_at
               FROM cmdb_data_center WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| row_to_datacenter(&r)))
    }

    async fn find_all(&self, filter: DataCenterFilter) -> Result<Vec<DataCenter>, sqlx::Error> {
        let mut sql = String::from(
            r#"SELECT id, name, provider_id, phone, address, country, province, city,
                      line_type::text, description, status::text, created_at, updated_at
               FROM cmdb_data_center WHERE 1=1"#,
        );
        let mut binds: Vec<String> = Vec::new();
        let mut idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status::text = ${}", idx));
            binds.push(status.clone());
            idx += 1;
        }
        if let Some(ref country) = filter.country {
            sql.push_str(&format!(" AND country = ${}", idx));
            binds.push(country.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", idx));
            binds.push(format!("%{}%", q));
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query = sqlx::query(&sql);
        for bind in &binds {
            query = query.bind(bind);
        }

        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_datacenter(r)).collect())
    }

    async fn count(&self, filter: DataCenterFilter) -> Result<i64, sqlx::Error> {
        let mut sql = String::from("SELECT COUNT(*) as count FROM cmdb_data_center WHERE 1=1");
        let mut binds: Vec<String> = Vec::new();
        let mut idx = 1;

        if let Some(ref status) = filter.status {
            sql.push_str(&format!(" AND status::text = ${}", idx));
            binds.push(status.clone());
            idx += 1;
        }
        if let Some(ref country) = filter.country {
            sql.push_str(&format!(" AND country = ${}", idx));
            binds.push(country.clone());
            idx += 1;
        }
        if let Some(ref q) = filter.q {
            sql.push_str(&format!(" AND name ILIKE ${}", idx));
            binds.push(format!("%{}%", q));
            // idx += 1; // unused after last use
        }

        let mut query = sqlx::query(&sql);
        for bind in &binds {
            query = query.bind(bind);
        }

        let row = query.fetch_one(&self.pool).await?;
        Ok(row.get::<i64, _>("count"))
    }

    async fn create(&self, dc: &DataCenter) -> Result<DataCenter, sqlx::Error> {
        let row = sqlx::query(
            r#"INSERT INTO cmdb_data_center
               (id, name, provider_id, phone, address, country, province, city,
                line_type, description, status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
               RETURNING id, name, provider_id, phone, address, country, province, city,
                         line_type::text, description, status::text, created_at, updated_at"#,
        )
        .bind(dc.id)
        .bind(&dc.name)
        .bind(dc.provider_id)
        .bind(&dc.phone)
        .bind(&dc.address)
        .bind(&dc.country)
        .bind(&dc.province)
        .bind(&dc.city)
        .bind(dc.line_type.as_ref().map(line_type_to_string))
        .bind(&dc.description)
        .bind(common_status_to_string(&dc.status))
        .bind(dc.created_at)
        .bind(dc.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row_to_datacenter(&row))
    }

    async fn update(&self, id: Uuid, dc: &DataCenter) -> Result<Option<DataCenter>, sqlx::Error> {
        let row = sqlx::query(
            r#"UPDATE cmdb_data_center SET
                name = $2, provider_id = $3, phone = $4, address = $5,
                country = $6, province = $7, city = $8, line_type = $9,
                description = $10, status = $11, updated_at = $12
               WHERE id = $1
               RETURNING id, name, provider_id, phone, address, country, province, city,
                         line_type::text, description, status::text, created_at, updated_at"#,
        )
        .bind(id)
        .bind(&dc.name)
        .bind(dc.provider_id)
        .bind(&dc.phone)
        .bind(&dc.address)
        .bind(&dc.country)
        .bind(&dc.province)
        .bind(&dc.city)
        .bind(dc.line_type.as_ref().map(line_type_to_string))
        .bind(&dc.description)
        .bind(common_status_to_string(&dc.status))
        .bind(dc.updated_at)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| row_to_datacenter(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cmdb_data_center WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
