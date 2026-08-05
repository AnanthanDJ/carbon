use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),

            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),

            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://genesis.db".into()),

            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "change-me".into()),
        }
    }
}
