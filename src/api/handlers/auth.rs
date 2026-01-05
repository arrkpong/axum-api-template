//! Authentication handlers

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{api::error::ApiError, config::AppState, domain::services::AuthService};

// ============================================================================
// Request/Response DTOs
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub data: AuthData,
}

#[derive(Debug, Serialize)]
pub struct AuthData {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
}

// ============================================================================
// Handlers
// ============================================================================

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    // Validate input
    payload.validate()?;

    // Call auth service
    let auth_service = AuthService::new(&state);
    let token = auth_service
        .register(&payload.email, &payload.password, &payload.name)
        .await?;

    Ok(Json(AuthResponse {
        success: true,
        data: AuthData {
            token,
            token_type: "Bearer".to_string(),
            expires_in: state.config.jwt_expiration_hours * 3600,
        },
    }))
}

/// Login with email and password
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    // Validate input
    payload.validate()?;

    // Call auth service
    let auth_service = AuthService::new(&state);
    let token = auth_service
        .login(&payload.email, &payload.password)
        .await?;

    Ok(Json(AuthResponse {
        success: true,
        data: AuthData {
            token,
            token_type: "Bearer".to_string(),
            expires_in: state.config.jwt_expiration_hours * 3600,
        },
    }))
}
