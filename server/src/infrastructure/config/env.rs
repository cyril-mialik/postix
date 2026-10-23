use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub cache_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            cache_url: std::env::var("CACHE_URL")?,
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            cors_origin: std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".into()),
        })
    }
}
