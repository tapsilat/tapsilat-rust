//! Error types and handling for the Tapsilat SDK.
//!
//! This module defines the comprehensive error types that can occur when using the SDK,
//! including HTTP errors, validation errors, and API-specific errors.

use std::fmt;

/// Main error type for all Tapsilat SDK operations.
///
/// This enum covers all possible error conditions that can occur when
/// using the SDK, from network issues to validation failures.
#[derive(Debug)]
pub enum TapsilatError {
    /// HTTP transport error occurred during API communication.
    Http(ureq::Error),
    /// Error occurred while serializing or deserializing data.
    Serialization(std::io::Error),
    /// API returned an invalid or unexpected response format.
    InvalidResponse(String),
    /// API returned an error status code with an error message.
    ApiError {
        /// HTTP status code returned by the API
        status_code: u16,
        /// Error message from the API
        message: String,
    },
    /// Configuration error, such as missing API key or invalid base URL.
    ConfigError(String),
    /// Input validation error occurred before making API request.
    ValidationError(String),
}

impl fmt::Display for TapsilatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TapsilatError::Http(err) => write!(f, "HTTP error: {}", err),
            TapsilatError::Serialization(err) => write!(f, "Serialization error: {}", err),
            TapsilatError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            TapsilatError::ApiError {
                status_code,
                message,
            } => {
                write!(f, "API error ({}): {}", status_code, message)
            }
            TapsilatError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            TapsilatError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for TapsilatError {}

impl From<ureq::Error> for TapsilatError {
    fn from(err: ureq::Error) -> Self {
        TapsilatError::Http(err)
    }
}

impl From<std::io::Error> for TapsilatError {
    fn from(err: std::io::Error) -> Self {
        TapsilatError::Serialization(err)
    }
}

impl From<serde_json::Error> for TapsilatError {
    fn from(err: serde_json::Error) -> Self {
        TapsilatError::InvalidResponse(format!("JSON parsing error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, TapsilatError>;
