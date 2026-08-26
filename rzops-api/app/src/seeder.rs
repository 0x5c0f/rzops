use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;

/// Seed the database with an initial admin user.
/// Reads credentials from environment variables:
///   - RZOPS_SEED_ADMIN_EMAIL (default: admin@rzops.local)
///   - RZOPS_SEED_ADMIN_PASSWORD (default: admin123)
pub async fn seed_admin_user(pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let email = std::env::var("RZOPS_SEED_ADMIN_EMAIL")
        .unwrap_or_else(|_| "admin@rzops.local".to_string());
    let password = std::env::var("RZOPS_SEED_ADMIN_PASSWORD")
        .unwrap_or_else(|_| "admin123".to_string());

    // Check if user already exists
    let existing: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM \"user\" WHERE email = $1"
    )
    .bind(&email)
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        tracing::info!("admin user '{}' already exists, skipping seed", email);
        return Ok(());
    }

    let hashed = bcrypt::hash(&password, bcrypt::DEFAULT_COST)?;
    let now = Utc::now();

    sqlx::query(
        r#"INSERT INTO "user" (id, email, hashed_password, is_active, is_superuser, full_name, created_at)
           VALUES ($1, $2, $3, true, true, 'Administrator', $4)"#
    )
    .bind(Uuid::new_v4())
    .bind(&email)
    .bind(&hashed)
    .bind(now)
    .execute(pool)
    .await?;

    tracing::info!("seeded admin user: {}", email);
    Ok(())
}
