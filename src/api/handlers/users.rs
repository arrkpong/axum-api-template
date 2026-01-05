//! User handlers

use axum::{
    Extension, Json,
    extract::{Path, State},
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    api::error::ApiError,
    config::AppState,
    domain::{models::User, services::UserService},
};

// ============================================================================
// Response DTOs
// ============================================================================

use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub success: bool,
    pub data: UserData,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserData {
    pub id: Uuid,
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "John Doe")]
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
        }
    }
}

// ============================================================================
// Handlers
// ============================================================================

/// Get current authenticated user
#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_current_user(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user_service = UserService::new(&state);
    let user = user_service.get_by_id(user_id).await?;

    Ok(Json(UserResponse {
        success: true,
        data: user.into(),
    }))
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user_service = UserService::new(&state);
    let user = user_service.get_by_id(id).await?;

    Ok(Json(UserResponse {
        success: true,
        data: user.into(),
    }))
}
