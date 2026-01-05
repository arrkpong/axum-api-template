//! API error handling
//!
//! Provides a unified error type for all API responses.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

/// Unified API error type
#[derive(Debug, ToSchema)]
pub struct ApiError {
    #[schema(value_type = u16, example = 500)]
    pub status: StatusCode,
    #[schema(example = "Internal server error")]
    pub message: String,
    #[schema(example = "INTERNAL_ERROR")]
    pub error_code: Option<String>,
}

/// JSON response body for errors
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorBody,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorBody {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[allow(dead_code)]
impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            error_code: None,
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.error_code = Some(code.into());
        self
    }

    // Common error constructors
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn unprocessable(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorResponse {
            success: false,
            error: ErrorBody {
                message: self.message,
                code: self.error_code,
            },
        };

        (self.status, Json(body)).into_response()
    }
}

// Convert common error types to ApiError
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        ApiError::internal("Database error occurred")
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        ApiError::bad_request(format!("Validation failed: {}", err)).with_code("VALIDATION_ERROR")
    }
}
