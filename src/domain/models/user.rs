//! User domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Create a new user instance (for insertion)
    pub fn new(email: String, password_hash: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            name,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
