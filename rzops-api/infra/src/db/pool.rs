use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};

/// Create a PostgreSQL connection pool from a database URL.
pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let options: PgConnectOptions = database_url.parse()?;
    PgPoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await
}
