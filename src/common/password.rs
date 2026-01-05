//! Password hashing utilities using Argon2

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

/// Hash a password using Argon2
pub fn hash(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| PasswordError::HashingFailed)
}

/// Verify a password against a hash
pub fn verify(password: &str, hash: &str) -> Result<bool, PasswordError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| PasswordError::InvalidHash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("Failed to hash password")]
    HashingFailed,
    #[error("Invalid password hash format")]
    InvalidHash,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "secure_password123";
        let hashed = hash(password).expect("Failed to hash password");

        assert!(verify(password, &hashed).expect("Failed to verify"));
        assert!(!verify("wrong_password", &hashed).expect("Failed to verify"));
    }
}
