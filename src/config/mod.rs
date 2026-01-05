//! Configuration module for application settings.
//!
//! This module handles loading and validating configuration from environment variables.

mod app;
mod database;

pub use app::AppConfig;
pub use database::DatabaseConfig;

use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(db_pool: sqlx::PgPool, config: AppConfig) -> Self {
        Self {
            db_pool,
            config: Arc::new(config),
        }
    }
}
