//! Authentication service

use crate::{
    common::{jwt::create_token, password},
    config::AppState,
    domain::{errors::DomainError, models::User},
    infrastructure::repositories::UserRepository,
};

pub struct AuthService<'a> {
    state: &'a AppState,
    user_repo: UserRepository<'a>,
}

impl<'a> AuthService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            user_repo: UserRepository::new(&state.db_pool),
        }
    }

    /// Register a new user
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        name: &str,
    ) -> Result<String, DomainError> {
        // Check if user already exists
        if self.user_repo.find_by_email(email).await?.is_some() {
            return Err(DomainError::UserAlreadyExists);
        }

        // Hash password
        let password_hash =
            password::hash(password).map_err(|_| DomainError::PasswordHashingFailed)?;

        // Create user
        let user = User::new(email.to_string(), password_hash, name.to_string());

        // Save to database
        self.user_repo.create(&user).await?;

        // Generate JWT token
        let token = create_token(
            user.id,
            &self.state.config.jwt_secret,
            self.state.config.jwt_expiration_hours,
        )
        .map_err(|_| DomainError::TokenGenerationFailed)?;

        Ok(token)
    }

    /// Login with email and password
    pub async fn login(&self, email: &str, password: &str) -> Result<String, DomainError> {
        // Find user by email
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // Verify password
        if !password::verify(password, &user.password_hash)
            .map_err(|_| DomainError::InvalidCredentials)?
        {
            return Err(DomainError::InvalidCredentials);
        }

        // Generate JWT token
        let token = create_token(
            user.id,
            &self.state.config.jwt_secret,
            self.state.config.jwt_expiration_hours,
        )
        .map_err(|_| DomainError::TokenGenerationFailed)?;

        Ok(token)
    }
}
