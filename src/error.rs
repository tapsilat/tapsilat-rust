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
    Http(Box<ureq::Error>),
    /// Error occurred while serializing a request or deserializing a response.
    Serialization(std::io::Error),
    /// API returned an invalid or unexpected response format.
    InvalidResponse(String),
    /// API returned an error status code, or a body that is not JSON.
    ApiError {
        /// HTTP status code returned by the API
        status_code: u16,
        /// Error message from the API: the body's `message`, else its `error`
        message: String,
        /// The body's `status` field, else the HTTP reason phrase
        status: String,
        /// The body's `code` field rendered as text, such as "400000"
        code: Option<String>,
        /// Per-field validation messages, when the API returns them
        errors: Option<Box<serde_json::Map<String, serde_json::Value>>>,
        /// The raw response body
        body: String,
    },
    /// Configuration error, such as missing API key or invalid base URL.
    ConfigError(String),
    /// Input validation error occurred before making API request.
    ValidationError(String),
}

impl TapsilatError {
    /// Builds an [`TapsilatError::ApiError`] from an error response body.
    pub(crate) fn from_api_response(status_code: u16, body: &str) -> Self {
        let mut status = ureq::http::StatusCode::from_u16(status_code)
            .ok()
            .and_then(|s| s.canonical_reason())
            .unwrap_or_default()
            .to_string();
        let mut code = None;
        let mut message = String::new();
        let mut errors = None;

        if let Ok(serde_json::Value::Object(payload)) = serde_json::from_str(body) {
            if let Some(value) = payload.get("status") {
                status = json_text(value);
            }
            code = payload.get("code").map(json_text);
            if let Some(value) = payload.get("message").or_else(|| payload.get("error")) {
                message = json_text(value);
            }
            if let Some(serde_json::Value::Object(map)) = payload.get("errors") {
                errors = Some(Box::new(map.clone()));
            }
        }

        TapsilatError::ApiError {
            status_code,
            message,
            status,
            code,
            errors,
            body: body.to_string(),
        }
    }

    /// Builds the error returned when a response is not JSON, such as the panel's HTML page.
    pub(crate) fn unexpected_response(status_code: u16, content_type: &str, body: &str) -> Self {
        TapsilatError::ApiError {
            status_code,
            message: format!(
                "unexpected non-JSON response (Content-Type {:?}); check that the endpoint URL and route are correct",
                content_type
            ),
            status: ureq::http::StatusCode::from_u16(status_code)
                .ok()
                .and_then(|s| s.canonical_reason())
                .unwrap_or_default()
                .to_string(),
            code: None,
            errors: None,
            body: body.to_string(),
        }
    }
}

fn json_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.clone(),
        other => other.to_string(),
    }
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
                status,
                code,
                body,
                ..
            } => match code {
                Some(code) if !message.is_empty() => write!(
                    f,
                    "API request failed with status {} ({}): [{}] {}",
                    status_code, status, code, message
                ),
                _ if !message.is_empty() => write!(
                    f,
                    "API request failed with status {} ({}): {}",
                    status_code, status, message
                ),
                _ if !body.is_empty() => write!(
                    f,
                    "API request failed with status {} ({}): {}",
                    status_code, status, body
                ),
                _ => write!(
                    f,
                    "API request failed with status {} ({})",
                    status_code, status
                ),
            },
            TapsilatError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            TapsilatError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for TapsilatError {}

impl From<ureq::Error> for TapsilatError {
    fn from(err: ureq::Error) -> Self {
        TapsilatError::Http(Box::new(err))
    }
}

impl From<std::io::Error> for TapsilatError {
    fn from(err: std::io::Error) -> Self {
        TapsilatError::Serialization(err)
    }
}

impl From<serde_json::Error> for TapsilatError {
    fn from(err: serde_json::Error) -> Self {
        TapsilatError::Serialization(err.into())
    }
}

pub type Result<T> = std::result::Result<T, TapsilatError>;
