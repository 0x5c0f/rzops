use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    /// Build a PostgreSQL connection URL for sqlx.
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    /// Secret key for HS256 signing. Must be set via RZOPS_JWT__SECRET.
    pub secret: String,
    /// Token expiration in seconds. Default: 86400 (24 hours).
    pub expiration_seconds: u64,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, String> {
        let secret = std::env::var("RZOPS_JWT__SECRET")
            .map_err(|_| "RZOPS_JWT__SECRET is required. Set it in .env or environment.".to_string())?;
        let expiration = std::env::var("RZOPS_JWT__EXPIRATION_SECONDS")
            .unwrap_or_else(|_| "86400".to_string())
            .parse()
            .map_err(|e| format!("invalid RZOPS_JWT__EXPIRATION_SECONDS: {e}"))?;
        Ok(Self { secret, expiration_seconds: expiration })
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

impl Settings {
    /// Load settings from environment variables.
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            server: ServerConfig {
                host: std::env::var("RZOPS_SERVER__HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("RZOPS_SERVER__PORT")
                    .unwrap_or_else(|_| "8000".to_string())
                    .parse()
                    .map_err(|e| format!("invalid RZOPS_SERVER__PORT: {e}"))?,
            },
            database: DatabaseConfig {
                host: std::env::var("RZOPS_DATABASE__HOST")
                    .unwrap_or_else(|_| "localhost".to_string()),
                port: std::env::var("RZOPS_DATABASE__PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .map_err(|e| format!("invalid RZOPS_DATABASE__PORT: {e}"))?,
                user: std::env::var("RZOPS_DATABASE__USER")
                    .map_err(|_| "RZOPS_DATABASE__USER is required".to_string())?,
                password: std::env::var("RZOPS_DATABASE__PASSWORD")
                    .map_err(|_| "RZOPS_DATABASE__PASSWORD is required".to_string())?,
                name: std::env::var("RZOPS_DATABASE__NAME")
                    .map_err(|_| "RZOPS_DATABASE__NAME is required".to_string())?,
            },
            jwt: JwtConfig::from_env()?,
        })
    }
}
