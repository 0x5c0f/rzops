use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use rzops_domain::models::role::Role;
use rzops_domain::ports::role_repository::RoleRepository;

/// PostgreSQL implementation of RoleRepository.
pub struct PgRoleRepository {
    pool: Pool<Postgres>,
}

impl PgRoleRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn row_to_role(row: &sqlx::postgres::PgRow) -> Role {
    Role {
        id: row.get("id"),
        code: row.get("code"),
        name: row.get("name"),
        description: row.get("description"),
        is_builtin: row.get("is_builtin"),
        is_active: row.get("is_active"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}

const SELECT_COLS: &str =
    "id, code, name, description, is_builtin, is_active, created_at, updated_at, deleted_at";

#[async_trait]
impl RoleRepository for PgRoleRepository {
    async fn create(&self, role: &Role) -> Result<Role, sqlx::Error> {
        let row = sqlx::query(
            r#"INSERT INTO role (id, code, name, description, is_builtin, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               RETURNING id, code, name, description, is_builtin, is_active, created_at, updated_at, deleted_at"#,
        )
        .bind(role.id)
        .bind(&role.code)
        .bind(&role.name)
        .bind(&role.description)
        .bind(role.is_builtin)
        .bind(role.is_active)
        .bind(role.created_at)
        .bind(role.updated_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(row_to_role(&row))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, sqlx::Error> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM role WHERE id = $1 AND deleted_at IS NULL",
            SELECT_COLS
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| row_to_role(&r)))
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Role>, sqlx::Error> {
        let row = sqlx::query(&format!(
            "SELECT {} FROM role WHERE code = $1 AND deleted_at IS NULL",
            SELECT_COLS
        ))
        .bind(code)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| row_to_role(&r)))
    }

    async fn find_all(&self) -> Result<Vec<Role>, sqlx::Error> {
        let rows = sqlx::query(&format!(
            "SELECT {} FROM role WHERE deleted_at IS NULL ORDER BY is_builtin DESC, created_at ASC",
            SELECT_COLS
        ))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.iter().map(row_to_role).collect())
    }

    async fn update(
        &self,
        id: Uuid,
        name: &str,
        description: Option<&str>,
        is_active: bool,
    ) -> Result<Option<Role>, sqlx::Error> {
        let row = sqlx::query(
            r#"UPDATE role SET name = $2, description = $3, is_active = $4, updated_at = now()
               WHERE id = $1 AND deleted_at IS NULL
               RETURNING id, code, name, description, is_builtin, is_active, created_at, updated_at, deleted_at"#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(is_active)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| row_to_role(&r)))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        // 内置角色禁止删除；自定义角色软删除（保留权限关联以便审计追溯）
        let result = sqlx::query(
            "UPDATE role SET deleted_at = now(), is_active = false, updated_at = now() WHERE id = $1 AND is_builtin = false AND deleted_at IS NULL",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn get_role_permissions(&self, role_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT permission_code FROM role_permission WHERE role_id = $1 ORDER BY permission_code",
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.iter().map(|r| r.get::<String, _>("permission_code")).collect())
    }

    async fn set_role_permissions(&self, role_id: Uuid, perms: &[String]) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM role_permission WHERE role_id = $1")
            .bind(role_id)
            .execute(&mut *tx)
            .await?;
        for p in perms {
            sqlx::query("INSERT INTO role_permission (role_id, permission_code) VALUES ($1, $2)")
                .bind(role_id)
                .bind(p)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT r.id, r.code, r.name, r.description, r.is_builtin, r.is_active, r.created_at, r.updated_at, r.deleted_at
               FROM role r
               JOIN user_role ur ON ur.role_id = r.id
               WHERE ur.user_id = $1 AND r.deleted_at IS NULL AND r.is_active = true
               ORDER BY r.is_builtin DESC, r.created_at ASC"#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.iter().map(row_to_role).collect())
    }

    async fn set_user_roles(&self, user_id: Uuid, role_ids: &[Uuid]) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM user_role WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        for rid in role_ids {
            sqlx::query("INSERT INTO user_role (user_id, role_id) VALUES ($1, $2)")
                .bind(user_id)
                .bind(rid)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await
    }

    async fn get_user_role_codes(&self, user_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT r.code FROM role r
               JOIN user_role ur ON ur.role_id = r.id
               WHERE ur.user_id = $1 AND r.deleted_at IS NULL AND r.is_active = true
               ORDER BY r.code"#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.iter().map(|r| r.get::<String, _>("code")).collect())
    }

    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT DISTINCT rp.permission_code FROM role_permission rp
               JOIN user_role ur ON ur.role_id = rp.role_id
               JOIN role r ON r.id = rp.role_id
               WHERE ur.user_id = $1 AND r.deleted_at IS NULL AND r.is_active = true
               ORDER BY rp.permission_code"#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.iter().map(|r| r.get::<String, _>("permission_code")).collect())
    }
}
