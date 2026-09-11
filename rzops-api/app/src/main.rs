use rzops_config::Settings;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let settings = Settings::from_env().map_err(|e| anyhow::anyhow!("{}", e))?;
    tracing::info!("starting server on {}:{}", settings.server.host, settings.server.port);

    let state = rzops_server::AppState::build(&settings).await?;
    rzops_server::run(state, &settings.server.host, settings.server.port).await
}
