//! Axum API - Professional Axum Backend
//!
//! A production-ready REST API built with Axum framework.

mod api;
mod common;
mod config;
mod domain;
mod infrastructure;

use config::{AppConfig, AppState, DatabaseConfig};
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing (logging)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let app_config = AppConfig::from_env().expect("Failed to load app configuration");
    let db_config = DatabaseConfig::from_env().expect("Failed to load database configuration");

    tracing::info!("Starting server in {:?} mode", app_config.environment);

    // Create database connection pool
    let db_pool = db_config
        .create_pool()
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection established");

    // Run migrations
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations ran successfully");

    // Create application state
    let state = AppState::new(db_pool, app_config.clone());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // In production, specify allowed origins
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with CORS
    let app = api::create_router(state).layer(cors);

    // Start server
    let addr = app_config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("🚀 Server listening on http://{}", addr);

    axum::serve(listener, app).await.expect("Server error");
}
