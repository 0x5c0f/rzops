use crate::db::IntoRepoResult;
use rzops_domain::errors::RepositoryError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::user::User;
use rzops_domain::ports::user_repository::UserRepository;

pub struct PgUserRepository {
    pool: Pool<Postgres>,
}

impl PgUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn row_to_user(row: &sqlx::postgres::PgRow) -> User {
    User {
        id: row.get("id"),
        email: row.get("email"),
        hashed_password: row.get("hashed_password"),
        is_active: row.get("is_active"),
        is_superuser: row.get("is_superuser"),
        full_name: row.get("full_name"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        deleted_at: row.get("deleted_at"),
    }
}

const SELECT_COLS: &str =
    "id, email, hashed_password, is_active, is_superuser, full_name, created_at, deleted_at";

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM \"user\" WHERE id = $1 AND deleted_at IS NULL",
            SELECT_COLS
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await.repo()?;
        Ok(row.map(|r| row_to_user(&r)))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM \"user\" WHERE email = $1 AND deleted_at IS NULL",
            SELECT_COLS
        ))
        .bind(email)
        .fetch_optional(&self.pool)
        .await.repo()?;
        Ok(row.map(|r| row_to_user(&r)))
    }

    async fn create(&self, user: &User) -> Result<User, RepositoryError> {
        let row = sqlx::query(
            r#"INSERT INTO "user" (id, email, hashed_password, is_active, is_superuser, full_name, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id, email, hashed_password, is_active, is_superuser, full_name, created_at, deleted_at"#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.is_active)
        .bind(user.is_superuser)
        .bind(&user.full_name)
        .bind(user.created_at)
        .fetch_one(&self.pool)
        .await.repo()?;
        Ok(row_to_user(&row))
    }

    async fn list(
        &self,
        q: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<User>, i64), RepositoryError> {
        let offset = (page.max(1) - 1) * per_page;
        let mut where_clause = String::from("WHERE deleted_at IS NULL");
        let mut binds: Vec<String> = Vec::new();
        if let Some(keyword) = q.map(|s| s.trim()).filter(|s| !s.is_empty()) {
            where_clause.push_str(&format!(
                " AND (email ILIKE ${} OR full_name ILIKE ${})",
                binds.len() + 1,
                binds.len() + 2
            ));
            let pattern = format!("%{}%", keyword);
            binds.push(pattern.clone());
            binds.push(pattern);
        }
        let sql = format!(
            "SELECT {} FROM \"user\" {} ORDER BY created_at DESC LIMIT {} OFFSET {}",
            SELECT_COLS, where_clause, per_page, offset
        );
        let mut qry = sqlx::query(&sql);
        for v in &binds {
            qry = qry.bind(v);
        }
        let rows = qry.fetch_all(&self.pool).await.repo()?;
        let data: Vec<User> = rows.iter().map(row_to_user).collect();

        let count_sql = format!("SELECT COUNT(*) as count FROM \"user\" {}", where_clause);
        let mut count_qry = sqlx::query(&count_sql);
        for v in &binds {
            count_qry = count_qry.bind(v);
        }
        let count_row = count_qry.fetch_one(&self.pool).await.repo()?;
        let total: i64 = count_row.get("count");
        Ok((data, total))
    }

    async fn update_profile(
        &self,
        id: Uuid,
        email: &str,
        full_name: Option<&str>,
        is_active: bool,
        is_superuser: bool,
    ) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query(
            r#"UPDATE "user" SET email = $2, full_name = $3, is_active = $4, is_superuser = $5
               WHERE id = $1 AND deleted_at IS NULL
               RETURNING id, email, hashed_password, is_active, is_superuser, full_name, created_at, deleted_at"#,
        )
        .bind(id)
        .bind(email)
        .bind(full_name)
        .bind(is_active)
        .bind(is_superuser)
        .fetch_optional(&self.pool)
        .await.repo()?;
        Ok(row.map(|r| row_to_user(&r)))
    }

    async fn update_password(&self, id: Uuid, hashed_password: &str) -> Result<bool, RepositoryError> {
        let result = sqlx::query(
            r#"UPDATE "user" SET hashed_password = $2 WHERE id = $1 AND deleted_at IS NULL"#,
        )
        .bind(id)
        .bind(hashed_password)
        .execute(&self.pool)
        .await.repo()?;
        Ok(result.rows_affected() > 0)
    }

    async fn soft_delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        let result = sqlx::query(
            r#"UPDATE "user" SET deleted_at = now(), is_active = false WHERE id = $1 AND deleted_at IS NULL"#,
        )
        .bind(id)
        .execute(&self.pool)
        .await.repo()?;
        Ok(result.rows_affected() > 0)
    }
}
