//! OpenAPI documentation configuration

use utoipa::{
    OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::{
    api::{
        error::{ApiError, ErrorBody, ErrorResponse},
        handlers::{auth, health, users},
    },
    domain::models::User,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        auth::register,
        auth::login,
        users::get_current_user,
        users::get_user_by_id,
    ),
    components(
        schemas(
            User,
            auth::RegisterRequest,
            auth::LoginRequest,
            auth::AuthResponse,
            auth::AuthData,
            users::UserResponse,
            users::UserData,
            health::HealthResponse,
            ApiError,
            ErrorResponse,
            ErrorBody,
        )
    ),
    tags(
        (name = "system", description = "System endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
    ),
    modifiers(&SecurityAddon),
    security(
        ("jwt" = [])
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
