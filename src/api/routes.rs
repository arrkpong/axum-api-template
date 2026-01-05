//! Route definitions
//!
//! Composes all application routes into a single router.

use axum::{
    Router, middleware,
    routing::{get, post},
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::AppState;

use super::docs::ApiDoc;
use super::handlers::{auth, health, users};
use super::middleware::auth::auth_middleware;

/// Create the main application router
pub fn create_router(state: AppState) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/users/me", get(users::get_current_user))
        .route("/users/{id}", get(users::get_user_by_id))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Combine all routes under /api/v1 prefix
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest(
            "/api/v1",
            Router::new().merge(public_routes).merge(protected_routes),
        )
        .with_state(state)
}
