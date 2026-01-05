//! Domain layer - Business logic and domain models
//!
//! This module contains:
//! - Domain entities (models)
//! - Business logic services
//! - Domain-specific errors

pub mod errors;
pub mod models;
pub mod services;

pub use errors::DomainError;
