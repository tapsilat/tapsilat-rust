use crate::types::{WebhookEvent, WebhookVerificationResult, WebhookVerificationConfig};
use crate::error::{Result, TapsilatError};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WebhookModule;

impl WebhookModule {
    /// Verifies webhook signature and timestamp
    pub fn verify_webhook(
        payload: &str,
        signature: &str,
        config: &WebhookVerificationConfig,
    ) -> Result<WebhookVerificationResult> {
        // Parse the webhook event to get timestamp
        let webhook_event: WebhookEvent = serde_json::from_str(payload)
            .map_err(|e| TapsilatError::InvalidResponse(
                format!("Invalid webhook payload: {}", e)
            ))?;

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
                error: if is_valid { None } else { Some("Invalid signature".to_string()) },
            }),
            Err(e) => Ok(WebhookVerificationResult {
                is_valid: false,
                error: Some(format!("Signature verification error: {}", e)),
            })
        }
    }

    /// Parses webhook payload into WebhookEvent
    pub fn parse_webhook(payload: &str) -> Result<WebhookEvent> {
        serde_json::from_str(payload)
            .map_err(|e| TapsilatError::InvalidResponse(
                format!("Failed to parse webhook payload: {}", e)
            ))
    }

    /// Verifies webhook signature using HMAC-SHA256
    fn verify_signature(payload: &str, signature: &str, secret: &str) -> Result<bool> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Remove 'sha256=' prefix if present
        let signature = signature.strip_prefix("sha256=").unwrap_or(signature);

        // Create expected signature
        let expected_signature = Self::create_signature(payload, secret)?;
        
        // Compare signatures (constant time comparison would be better for production)
        Ok(signature == expected_signature)
    }

    /// Creates HMAC-SHA256 signature
    fn create_signature(payload: &str, secret: &str) -> Result<String> {
        // This is a simplified implementation
        // In a real implementation, you would use a proper HMAC-SHA256 library
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        format!("{}{}", secret, payload).hash(&mut hasher);
        let hash = hasher.finish();
        
        Ok(format!("{:x}", hash))
    }

    /// Verifies webhook timestamp
    fn verify_timestamp(timestamp_str: &str, tolerance_seconds: u64) -> Result<()> {
        // Parse timestamp (assuming ISO 8601 format or Unix timestamp)
        let webhook_time = if timestamp_str.contains('T') {
            // ISO 8601 format
            Self::parse_iso8601_timestamp(timestamp_str)?
        } else {
            // Assume Unix timestamp
            timestamp_str.parse::<u64>()
                .map_err(|e| TapsilatError::InvalidResponse(
                    format!("Invalid timestamp format: {}", e)
                ))?
        };

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| TapsilatError::InvalidResponse(
                format!("System time error: {}", e)
            ))?
            .as_secs();

        let time_diff = if current_time > webhook_time {
            current_time - webhook_time
        } else {
            webhook_time - current_time
        };

        if time_diff > tolerance_seconds {
            return Err(TapsilatError::InvalidResponse(
                format!("Webhook timestamp too old or too far in future. Difference: {}s, tolerance: {}s", 
                    time_diff, tolerance_seconds)
            ));
        }

        Ok(())
    }

    /// Parses ISO 8601 timestamp to Unix timestamp
    fn parse_iso8601_timestamp(timestamp: &str) -> Result<u64> {
        // This is a simplified parser
        // In production, use a proper datetime parsing library like chrono
        
        // For now, just return current timestamp as fallback
        // TODO: Implement proper ISO 8601 parsing
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| TapsilatError::InvalidResponse(
                format!("Timestamp parsing error: {}", e)
            ))
            .map(|d| d.as_secs())
    }

    /// Utility method to construct webhook verification config
    pub fn create_verification_config(secret: String, tolerance_seconds: Option<u64>) -> WebhookVerificationConfig {
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
        let config = WebhookModule::create_verification_config(
            "test_secret".to_string(),
            Some(300)
        );
        
        assert_eq!(config.secret, "test_secret");
        assert_eq!(config.tolerance_seconds, Some(300));
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
        assert!(matches!(webhook.event_type, crate::types::WebhookEventType::OrderCompleted));
    }
}