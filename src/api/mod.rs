//! API layer - HTTP handlers, routes, and middleware
//!
//! This module contains all HTTP-related code including:
//! - Route definitions
//! - Request handlers
//! - Middleware (authentication, logging, etc.)
//! - Error handling

pub mod error;
pub mod handlers;
pub mod middleware;
pub mod routes;

pub use routes::create_router;
