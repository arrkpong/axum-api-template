#![allow(dead_code)]

use validator::Validate;

use crate::api::error::ApiError;

/// Validate a request body and return an ApiError if invalid
pub fn validate_request<T: Validate>(data: &T) -> Result<(), ApiError> {
    data.validate().map_err(ApiError::from)
}

/// Common validation patterns
pub mod patterns {
    /// Phone number regex pattern (Thai format)
    pub const PHONE_THAI: &str = r"^(0[689]{1})\d{8}$";

    /// UUID pattern
    pub const UUID: &str =
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$";
}
