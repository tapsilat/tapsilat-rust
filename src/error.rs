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
        /// Error message from the API: the body's `message`, else its `error`, or the
        /// `message` of a nested `{"error": {...}}` object
        message: String,
        /// The body's `status` field, else the HTTP reason phrase
        status: String,
        /// The body's `code` field rendered as text, such as "400000", or the `code` of a
        /// nested `{"error": {...}}` object, such as "session_not_found"
        code: Option<String>,
        /// Per-field validation messages, when the API returns them
        errors: Option<Box<serde_json::Map<String, serde_json::Value>>>,
        /// Whether the request can be retried, from the `retryable` field of a nested
        /// `{"error": {...}}` object, as Elements errors carry; `false` when absent
        retryable: bool,
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
        let mut retryable = false;

        if let Ok(serde_json::Value::Object(payload)) = serde_json::from_str(body) {
            if let Some(value) = payload.get("status") {
                status = json_text(value);
            }
            code = payload.get("code").map(json_text);
            match (payload.get("message"), payload.get("error")) {
                (Some(value), _) => message = json_text(value),
                // Elements routes nest the error: {"error": {"code", "message", "retryable"}}
                (None, Some(serde_json::Value::Object(nested))) => {
                    if code.is_none() {
                        code = nested.get("code").map(json_text);
                    }
                    if let Some(value) = nested.get("message") {
                        message = json_text(value);
                    }
                    retryable = nested
                        .get("retryable")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false);
                }
                (None, Some(value)) => message = json_text(value),
                (None, None) => {}
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
            retryable,
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
            retryable: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parts(error: TapsilatError) -> (Option<String>, String, bool) {
        match error {
            TapsilatError::ApiError {
                code,
                message,
                retryable,
                ..
            } => (code, message, retryable),
            other => panic!("unexpected error {other:?}"),
        }
    }

    #[test]
    fn nested_error_objects_give_code_message_and_retryable() {
        let error = TapsilatError::from_api_response(
            503,
            r#"{"error":{"code":"unavailable","message":"try again","retryable":true}}"#,
        );
        assert_eq!(
            error.to_string(),
            "API request failed with status 503 (Service Unavailable): [unavailable] try again"
        );
        assert_eq!(
            parts(error),
            (
                Some("unavailable".to_string()),
                "try again".to_string(),
                true
            )
        );

        let error = TapsilatError::from_api_response(
            404,
            r#"{"error":{"code":"session_not_found","message":"session not found"}}"#,
        );
        assert_eq!(
            parts(error),
            (
                Some("session_not_found".to_string()),
                "session not found".to_string(),
                false
            )
        );
    }

    #[test]
    fn flat_error_shapes_still_work() {
        let error =
            TapsilatError::from_api_response(424, r#"{"error":"payout is not eligible","code":9}"#);
        assert_eq!(
            parts(error),
            (
                Some("9".to_string()),
                "payout is not eligible".to_string(),
                false
            )
        );

        let error = TapsilatError::from_api_response(
            400,
            r#"{"message":"INVALID_REQUEST_BODY","code":400000,"error":{"code":"x"}}"#,
        );
        assert_eq!(
            parts(error),
            (
                Some("400000".to_string()),
                "INVALID_REQUEST_BODY".to_string(),
                false
            )
        );
    }
}
