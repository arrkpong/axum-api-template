//! User repository - Data access for users

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::User;

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user
    pub async fn create(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, name, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(&user.is_active)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, is_active, created_at, updated_at
            FROM users
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, is_active, created_at, updated_at
            FROM users
            WHERE email = $1 AND is_active = true
            "#,
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await
    }

    /// Update user
    pub async fn update(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET email = $2, name = $3, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.name)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Soft delete user
    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET is_active = false, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}
