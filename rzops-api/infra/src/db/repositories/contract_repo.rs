use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use rzops_domain::models::contract::Contract;
use rzops_domain::ports::contract_repository::{ContractFilter, ContractRepository};

pub struct PgContractRepository { pool: Pool<Postgres> }
impl PgContractRepository { pub fn new(pool: Pool<Postgres>) -> Self { Self { pool } } }


fn row_to_entity(row: &sqlx::postgres::PgRow) -> Contract {
    Contract { id: row.get("id"), name: row.get("name"), provider_id: row.get("provider_id"), subject_type: row.get("subject_type"), subject_id: row.get("subject_id"), contract_no: row.get("contract_no"), start_date: row.get("start_date"), end_date: row.get("end_date"), amount: row.get::<Option<Decimal>, _>("amount"), currency: row.get("currency"), status: row.get::<String, _>("status"), remarks: row.get("remarks"), created_at: row.get::<DateTime<Utc>, _>("created_at"), updated_at: row.get::<DateTime<Utc>, _>("updated_at") }
}

const COLS: &str = "id, name, provider_id, subject_type, subject_id, contract_no, start_date, end_date, amount, currency, status::text, remarks, created_at, updated_at";

#[async_trait]
impl ContractRepository for PgContractRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Contract>, sqlx::Error> {
        Ok(sqlx::query(&format!("SELECT {} FROM cmdb_contract WHERE id=$1", COLS)).bind(id).fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn find_all(&self, f: ContractFilter) -> Result<Vec<Contract>, sqlx::Error> {
        let mut sql = format!("SELECT {} FROM cmdb_contract WHERE 1=1", COLS);
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_q = f.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        sql.push_str(" ORDER BY CASE WHEN status::text IN ('expired', 'terminated', 'inactive') THEN 1 ELSE 0 END, created_at DESC");
        if let Some(l) = f.limit { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = f.offset { sql.push_str(&format!(" OFFSET {}", o)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_all(&self.pool).await?.iter().map(|r| row_to_entity(r)).collect())
    }
    async fn count(&self, f: ContractFilter) -> Result<i64, sqlx::Error> {
        let mut sql = "SELECT COUNT(*) as count FROM cmdb_contract WHERE 1=1".to_string();
        let mut idx = 1;
        let s_status = f.status.as_ref();
        let s_q = f.q.as_ref();
        if s_status.is_some() { sql.push_str(&format!(" AND status::text = ${}", idx)); idx += 1; }
        if s_q.is_some() { sql.push_str(&format!(" AND name ILIKE ${}", idx)); }
        let mut query = sqlx::query(&sql);
        if let Some(s) = s_status { query = query.bind(s); }
        if let Some(q) = s_q { query = query.bind(format!("%{}%", q)); }
        Ok(query.fetch_one(&self.pool).await?.get::<i64, _>("count"))
    }
    async fn create(&self, e: &Contract) -> Result<Contract, sqlx::Error> {
        Ok(row_to_entity(&sqlx::query(&format!("INSERT INTO cmdb_contract (id,name,provider_id,subject_type,subject_id,contract_no,start_date,end_date,amount,currency,status,remarks,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14) RETURNING {}", COLS))
            .bind(e.id).bind(&e.name).bind(e.provider_id).bind(&e.subject_type).bind(e.subject_id).bind(&e.contract_no).bind(e.start_date).bind(e.end_date).bind(e.amount).bind(&e.currency).bind(e.status.clone()).bind(&e.remarks).bind(e.created_at).bind(e.updated_at)
            .fetch_one(&self.pool).await?))
    }
    async fn update(&self, id: Uuid, e: &Contract) -> Result<Option<Contract>, sqlx::Error> {
        Ok(sqlx::query(&format!("UPDATE cmdb_contract SET name=$2,provider_id=$3,subject_type=$4,subject_id=$5,contract_no=$6,start_date=$7,end_date=$8,amount=$9,currency=$10,status=$11,remarks=$12,updated_at=$13 WHERE id=$1 RETURNING {}", COLS))
            .bind(id).bind(&e.name).bind(e.provider_id).bind(&e.subject_type).bind(e.subject_id).bind(&e.contract_no).bind(e.start_date).bind(e.end_date).bind(e.amount).bind(&e.currency).bind(e.status.clone()).bind(&e.remarks).bind(e.updated_at)
            .fetch_optional(&self.pool).await?.map(|r| row_to_entity(&r)))
    }
    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query("DELETE FROM cmdb_contract WHERE id=$1").bind(id).execute(&self.pool).await?.rows_affected() > 0)
    }
}
