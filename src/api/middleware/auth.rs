//! Authentication middleware

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::{api::error::ApiError, common::jwt::verify_token, config::AppState};

/// Authentication middleware
/// Validates JWT token and injects user_id into request extensions
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| ApiError::unauthorized("Missing authorization header"))?;

    // Parse Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::unauthorized("Invalid authorization format"))?;

    // Verify token and extract claims
    let claims = verify_token(token, &state.config.jwt_secret)
        .map_err(|_| ApiError::unauthorized("Invalid or expired token"))?;

    // Insert user_id into request extensions
    request.extensions_mut().insert(claims.sub);

    Ok(next.run(request).await)
}
