mod seeder;

use rzops_config::Settings;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let settings = Settings::from_env().map_err(|e| anyhow::anyhow!("{}", e))?;
    tracing::info!("starting server on {}:{}", settings.server.host, settings.server.port);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&settings.database.url())
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("database migrations completed");

    // Seed initial admin user
    seeder::seed_admin_user(&pool).await?;

    // Seed data dictionary (cmdb_dict)
    seeder::seed_dicts(&pool).await?;

    let state = rzops_server::AppState::new(
        pool,
        settings.jwt.secret.as_bytes(),
        settings.jwt.expiration_seconds,
    );
    rzops_server::run(state, &settings.server.host, settings.server.port).await
}
