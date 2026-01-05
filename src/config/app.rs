//! Application configuration

use std::env;

/// Main application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Server host address
    pub host: String,
    /// Server port
    pub port: u16,
    /// Environment (development, staging, production)
    pub environment: Environment,
    /// JWT secret key
    pub jwt_secret: String,
    /// JWT expiration time in hours
    pub jwt_expiration_hours: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidPort)?,
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string())
                .parse()?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| ConfigError::MissingEnvVar("JWT_SECRET"))?,
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidJwtExpiration)?,
        })
    }

    /// Get the full server address
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Check if running in production
    #[allow(dead_code)]
    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}

impl std::str::FromStr for Environment {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "staging" | "stg" => Ok(Self::Staging),
            "production" | "prod" => Ok(Self::Production),
            _ => Err(ConfigError::InvalidEnvironment),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(&'static str),
    #[error("Invalid port number")]
    InvalidPort,
    #[error("Invalid environment (use: development, staging, production)")]
    InvalidEnvironment,
    #[error("Invalid JWT expiration hours")]
    InvalidJwtExpiration,
}
