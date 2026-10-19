use crate::error::{Result, TapsilatError};
use crate::types::{WebhookEvent, WebhookVerificationConfig, WebhookVerificationResult};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(value.get(i..i + 2)?, 16).ok())
        .collect()
}

pub struct WebhookModule;

impl WebhookModule {
    pub fn verify_webhook(payload: &str, signature: &str, secret: &str) -> Result<bool> {
        Self::verify_signature(payload, signature, secret)
    }

    /// Verifies webhook signature and timestamp (advanced version)
    pub fn verify_webhook_advanced(
        payload: &str,
        signature: &str,
        config: &WebhookVerificationConfig,
    ) -> Result<WebhookVerificationResult> {
        // Parse the webhook event to get timestamp
        let webhook_event: WebhookEvent = serde_json::from_str(payload).map_err(|e| {
            TapsilatError::InvalidResponse(format!("Invalid webhook payload: {}", e))
        })?;

        // Verify timestamp if tolerance is set
        if let Some(tolerance) = config.tolerance_seconds {
            if let Err(e) = Self::verify_timestamp(&webhook_event.timestamp, tolerance) {
                return Ok(WebhookVerificationResult {
                    is_valid: false,
                    error: Some(format!("Timestamp validation failed: {}", e)),
                });
            }
        }

        // Verify signature
        match Self::verify_signature(payload, signature, &config.secret) {
            Ok(is_valid) => Ok(WebhookVerificationResult {
                is_valid,
                error: if is_valid {
                    None
                } else {
                    Some("Invalid signature".to_string())
                },
            }),
            Err(e) => Ok(WebhookVerificationResult {
                is_valid: false,
                error: Some(format!("Signature verification error: {}", e)),
            }),
        }
    }

    /// Parses webhook payload into WebhookEvent
    pub fn parse_webhook(payload: &str) -> Result<WebhookEvent> {
        serde_json::from_str(payload).map_err(|e| {
            TapsilatError::InvalidResponse(format!("Failed to parse webhook payload: {}", e))
        })
    }

    /// Verifies a hex HMAC-SHA256 signature of the payload, with or without the
    /// "sha256=" prefix, in constant time.
    fn verify_signature(payload: &str, signature: &str, secret: &str) -> Result<bool> {
        let signature = signature.trim();
        let signature = signature.strip_prefix("sha256=").unwrap_or(signature);
        let Some(expected) = decode_hex(signature) else {
            return Ok(false);
        };

        let mut mac = Self::mac(secret)?;
        mac.update(payload.as_bytes());
        Ok(mac.verify_slice(&expected).is_ok())
    }

    /// Creates the hex HMAC-SHA256 signature of the payload, without the "sha256=" prefix.
    pub fn create_signature(payload: &str, secret: &str) -> Result<String> {
        let mut mac = Self::mac(secret)?;
        mac.update(payload.as_bytes());
        Ok(mac
            .finalize()
            .into_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect())
    }

    fn mac(secret: &str) -> Result<Hmac<Sha256>> {
        <Hmac<Sha256> as KeyInit>::new_from_slice(secret.as_bytes())
            .map_err(|e| TapsilatError::ValidationError(format!("Invalid webhook secret: {}", e)))
    }

    /// Verifies webhook timestamp
    fn verify_timestamp(timestamp_str: &str, tolerance_seconds: u64) -> Result<()> {
        // Parse timestamp (assuming ISO 8601 format or Unix timestamp)
        let webhook_time = if timestamp_str.contains('T') {
            // ISO 8601 format
            Self::parse_iso8601_timestamp(timestamp_str)?
        } else {
            // Assume Unix timestamp
            timestamp_str.parse::<u64>().map_err(|e| {
                TapsilatError::InvalidResponse(format!("Invalid timestamp format: {}", e))
            })?
        };

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| TapsilatError::InvalidResponse(format!("System time error: {}", e)))?
            .as_secs();

        let time_diff = current_time.abs_diff(webhook_time);

        if time_diff > tolerance_seconds {
            return Err(TapsilatError::InvalidResponse(format!(
                "Webhook timestamp too old or too far in future. Difference: {}s, tolerance: {}s",
                time_diff, tolerance_seconds
            )));
        }

        Ok(())
    }

    /// Parses an RFC3339 timestamp to a Unix timestamp
    fn parse_iso8601_timestamp(timestamp: &str) -> Result<u64> {
        let parsed = chrono::DateTime::parse_from_rfc3339(timestamp).map_err(|e| {
            TapsilatError::InvalidResponse(format!("Timestamp parsing error: {}", e))
        })?;
        u64::try_from(parsed.timestamp())
            .map_err(|_| TapsilatError::InvalidResponse("Timestamp is before 1970".to_string()))
    }

    /// Utility method to construct webhook verification config
    pub fn create_verification_config(
        secret: String,
        tolerance_seconds: Option<u64>,
    ) -> WebhookVerificationConfig {
        WebhookVerificationConfig {
            secret,
            tolerance_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_verification_config() {
        let config =
            WebhookModule::create_verification_config("test_secret".to_string(), Some(300));

        assert_eq!(config.secret, "test_secret");
        assert_eq!(config.tolerance_seconds, Some(300));
    }

    #[test]
    fn test_signature_is_hmac_sha256() {
        // RFC 4231 test case 2
        let signature =
            WebhookModule::create_signature("what do ya want for nothing?", "Jefe").unwrap();
        assert_eq!(
            signature,
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
        assert!(WebhookModule::verify_webhook(
            "what do ya want for nothing?",
            &format!("sha256={}", signature),
            "Jefe"
        )
        .unwrap());
        assert!(WebhookModule::verify_webhook(
            "what do ya want for nothing?",
            &signature.to_uppercase(),
            "Jefe"
        )
        .unwrap());
        assert!(!WebhookModule::verify_webhook("tampered", &signature, "Jefe").unwrap());
        assert!(!WebhookModule::verify_webhook("payload", "sha256=zz", "Jefe").unwrap());
    }

    #[test]
    fn test_timestamp_parsing() {
        assert_eq!(
            WebhookModule::parse_iso8601_timestamp("2023-01-01T00:00:00Z").unwrap(),
            1672531200
        );
        assert!(WebhookModule::parse_iso8601_timestamp("not a date").is_err());
    }

    #[test]
    fn test_webhook_parsing() {
        let payload = r#"{
            "event_type": "order.completed",
            "data": {
                "order_id": "order_123",
                "amount": 100.0,
                "currency": "TRY",
                "status": "completed"
            },
            "timestamp": "2023-01-01T00:00:00Z"
        }"#;

        let result = WebhookModule::parse_webhook(payload);
        assert!(result.is_ok());

        let webhook = result.unwrap();
        assert!(matches!(
            webhook.event_type,
            crate::types::WebhookEventType::OrderCompleted
        ));
    }
}
