//! User service

use uuid::Uuid;

use crate::{
    config::AppState,
    domain::{errors::DomainError, models::User},
    infrastructure::repositories::UserRepository,
};

pub struct UserService<'a> {
    user_repo: UserRepository<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            user_repo: UserRepository::new(&state.db_pool),
        }
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        self.user_repo
            .find_by_id(id)
            .await?
            .ok_or(DomainError::UserNotFound)
    }

    /// Get user by email
    #[allow(dead_code)]
    pub async fn get_by_email(&self, email: &str) -> Result<User, DomainError> {
        self.user_repo
            .find_by_email(email)
            .await?
            .ok_or(DomainError::UserNotFound)
    }
}
