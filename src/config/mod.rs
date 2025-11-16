use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub pool_max_size: u32,
    pub pool_min_idle: u32,
    pub pool_timeout_seconds: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/polimarket".to_string()),
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            pool_max_size: std::env::var("POOL_MAX_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            pool_min_idle: std::env::var("POOL_MIN_IDLE")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
            pool_timeout_seconds: std::env::var("POOL_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }
}
