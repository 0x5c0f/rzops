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
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, email, hashed_password, is_active, is_superuser, full_name, created_at FROM \"user\" WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| row_to_user(&r)))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, email, hashed_password, is_active, is_superuser, full_name, created_at FROM \"user\" WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| row_to_user(&r)))
    }

    async fn create(&self, user: &User) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            r#"INSERT INTO "user" (id, email, hashed_password, is_active, is_superuser, full_name, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id, email, hashed_password, is_active, is_superuser, full_name, created_at"#
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.is_active)
        .bind(user.is_superuser)
        .bind(&user.full_name)
        .bind(user.created_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(row_to_user(&row))
    }
}
