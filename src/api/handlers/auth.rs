//! Authentication handlers

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{api::error::ApiError, config::AppState, domain::services::AuthService};

// ============================================================================
// Request/Response DTOs
// ============================================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "user@example.com")]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(example = "password123")]
    pub password: String,
    #[validate(length(min = 1, message = "Name is required"))]
    #[schema(example = "John Doe")]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub success: bool,
    pub data: AuthData,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthData {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
}

// ============================================================================
// Handlers
// ============================================================================

/// Register a new user
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Registration successful", body = AuthResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 409, description = "Email already exists", body = ApiError)
    )
)]
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
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Validation error", body = ApiError),
        (status = 401, description = "Invalid credentials", body = ApiError)
    )
)]
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
