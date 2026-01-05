//! Domain-specific errors

use thiserror::Error;

use crate::api::error::ApiError;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User not found")]
    UserNotFound,

    #[error("User already exists with this email")]
    UserAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Password hashing failed")]
    PasswordHashingFailed,

    #[error("Token generation failed")]
    TokenGenerationFailed,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

// Convert DomainError to ApiError for HTTP responses
impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::UserNotFound => {
                ApiError::not_found("User not found").with_code("USER_NOT_FOUND")
            }
            DomainError::UserAlreadyExists => {
                ApiError::conflict("User already exists with this email")
                    .with_code("USER_ALREADY_EXISTS")
            }
            DomainError::InvalidCredentials => {
                ApiError::unauthorized("Invalid email or password").with_code("INVALID_CREDENTIALS")
            }
            DomainError::PasswordHashingFailed => {
                ApiError::internal("An error occurred during registration")
            }
            DomainError::TokenGenerationFailed => {
                ApiError::internal("An error occurred during authentication")
            }
            DomainError::DatabaseError(_) => ApiError::internal("A database error occurred"),
        }
    }
}
