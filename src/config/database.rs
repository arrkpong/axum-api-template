//! Database configuration

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use std::time::Duration;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
}

impl DatabaseConfig {
    /// Load database configuration from environment
    pub fn from_env() -> Result<Self, DatabaseConfigError> {
        Ok(Self {
            url: env::var("DATABASE_URL").map_err(|_| DatabaseConfigError::MissingDatabaseUrl)?,
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            acquire_timeout_secs: env::var("DATABASE_ACQUIRE_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        })
    }

    /// Create a database connection pool
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(Duration::from_secs(self.acquire_timeout_secs))
            .connect(&self.url)
            .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseConfigError {
    #[error("DATABASE_URL environment variable is not set")]
    MissingDatabaseUrl,
}
