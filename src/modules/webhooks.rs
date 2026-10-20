use crate::error::{Result, TapsilatError};
use crate::types::{WebhookEvent, WebhookVerificationConfig, WebhookVerificationResult};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

/// The header that carries the signature of every callback Tapsilat sends.
pub const WEBHOOK_SIGNATURE_HEADER: &str = "Tapsilat-Signature";

/// How many seconds a callback's signature timestamp may differ from the current time.
pub const DEFAULT_WEBHOOK_TOLERANCE: u64 = 300;

/// Options for [`WebhookModule::verify_webhook_with`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WebhookVerifyOptions {
    /// Seconds the signature timestamp may differ from `now`; 0 disables the check.
    pub tolerance_seconds: u64,
    /// Unix time in seconds to compare the timestamp with; `None` uses the system clock.
    pub now: Option<i64>,
}

impl Default for WebhookVerifyOptions {
    fn default() -> Self {
        Self {
            tolerance_seconds: DEFAULT_WEBHOOK_TOLERANCE,
            now: None,
        }
    }
}

impl WebhookVerifyOptions {
    /// Sets the tolerance in seconds; 0 disables the timestamp check.
    pub fn with_tolerance(mut self, tolerance_seconds: u64) -> Self {
        self.tolerance_seconds = tolerance_seconds;
        self
    }

    /// Compares the timestamp with this Unix time instead of the system clock.
    pub fn with_now(mut self, now: i64) -> Self {
        self.now = Some(now);
        self
    }
}

/// Decodes upper- or lowercase hex; `None` for anything else.
fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16).ok())
        .collect()
}

/// Compares a hex signature with the MAC in constant time.
fn matches_hex(mac: Hmac<Sha256>, signature: &str) -> bool {
    decode_hex(signature).is_some_and(|tag| mac.verify_slice(&tag).is_ok())
}

pub struct WebhookModule;

impl WebhookModule {
    /// Verifies the `Tapsilat-Signature` header of a callback against the organization's
    /// webhook secret, allowing [`DEFAULT_WEBHOOK_TOLERANCE`] seconds of clock difference.
    ///
    /// See [`WebhookModule::verify_webhook_with`]. This never returns an error.
    pub fn verify_webhook(payload: &str, signature: &str, secret: &str) -> Result<bool> {
        Ok(Self::verify_webhook_with(
            payload,
            signature,
            secret,
            &WebhookVerifyOptions::default(),
        ))
    }

    /// Verifies the `Tapsilat-Signature` header of a callback against the organization's
    /// webhook secret.
    ///
    /// Tapsilat sends `t=<unix timestamp>,v1=<hex HMAC-SHA256 of "<timestamp>.<raw body>">`.
    /// The timestamp must be within `options.tolerance_seconds` of `options.now` (the system
    /// clock by default) so that a captured callback cannot be replayed. `payload` must be the
    /// raw request body, before any JSON decoding. The comparison takes constant time.
    ///
    /// A bare hex HMAC-SHA256 of the payload, with or without a `sha256=` prefix, is still
    /// accepted for signatures made the way earlier versions of this SDK documented.
    pub fn verify_webhook_with(
        payload: impl AsRef<[u8]>,
        signature: &str,
        secret: &str,
        options: &WebhookVerifyOptions,
    ) -> bool {
        Self::check_signature(payload.as_ref(), signature, secret, options).is_ok()
    }

    /// Verifies the `Tapsilat-Signature` header like [`WebhookModule::verify_webhook_with`]
    /// and reports why a signature was rejected. `config.tolerance_seconds` of `None` skips
    /// the timestamp check.
    pub fn verify_webhook_advanced(
        payload: &str,
        signature: &str,
        config: &WebhookVerificationConfig,
    ) -> Result<WebhookVerificationResult> {
        let options =
            WebhookVerifyOptions::default().with_tolerance(config.tolerance_seconds.unwrap_or(0));
        let error =
            Self::check_signature(payload.as_bytes(), signature, &config.secret, &options).err();
        Ok(WebhookVerificationResult {
            is_valid: error.is_none(),
            error: error.map(str::to_string),
        })
    }

    /// Parses a raw callback body into a [`WebhookEvent`].
    pub fn parse_webhook(payload: &str) -> Result<WebhookEvent> {
        WebhookEvent::from_payload(payload)
    }

    fn check_signature(
        payload: &[u8],
        signature: &str,
        secret: &str,
        options: &WebhookVerifyOptions,
    ) -> std::result::Result<(), &'static str> {
        let signature = signature.trim();
        if signature.is_empty() {
            return Err("Missing signature");
        }
        if secret.is_empty() {
            return Err("Missing webhook secret");
        }
        let Ok(mut mac) = Self::mac(secret) else {
            return Err("Invalid webhook secret");
        };

        if signature.starts_with("sha256=") || !signature.contains('=') {
            mac.update(payload);
            let digest = signature.strip_prefix("sha256=").unwrap_or(signature);
            return if matches_hex(mac, digest) {
                Ok(())
            } else {
                Err("Invalid signature")
            };
        }

        let mut timestamp = None;
        let mut candidates = Vec::new();
        for part in signature.split(',') {
            let Some((key, value)) = part.trim().split_once('=') else {
                continue;
            };
            match key {
                "t" => timestamp = Some(value),
                "v1" => candidates.push(value),
                _ => {}
            }
        }
        let Some(timestamp) =
            timestamp.filter(|t| !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()))
        else {
            return Err("Signature header has no valid timestamp");
        };
        let Ok(signed_at) = timestamp.parse::<i64>() else {
            return Err("Signature header has no valid timestamp");
        };
        if candidates.is_empty() {
            return Err("Signature header has no v1 signature");
        }
        if options.tolerance_seconds > 0 {
            let now = options
                .now
                .unwrap_or_else(|| chrono::Utc::now().timestamp());
            if now.abs_diff(signed_at) > options.tolerance_seconds {
                return Err("Signature timestamp is outside the tolerance");
            }
        }

        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(payload);
        if candidates
            .iter()
            .any(|candidate| matches_hex(mac.clone(), candidate))
        {
            Ok(())
        } else {
            Err("Invalid signature")
        }
    }

    /// Creates the bare hex HMAC-SHA256 signature of the payload, without the "sha256="
    /// prefix. Tapsilat itself signs `"<timestamp>.<payload>"`; see
    /// [`WebhookModule::verify_webhook_with`].
    pub fn create_signature(payload: &str, secret: &str) -> Result<String> {
        let mut mac = Self::mac(secret)?;
        mac.update(payload.as_bytes());
        Ok(hex(&mac.finalize().into_bytes()))
    }

    fn mac(secret: &str) -> Result<Hmac<Sha256>> {
        <Hmac<Sha256> as KeyInit>::new_from_slice(secret.as_bytes())
            .map_err(|e| TapsilatError::ValidationError(format!("Invalid webhook secret: {}", e)))
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

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OrderStatus;
    use serde_json::json;

    // An order_lite callback signed the way admin's monosign.SignWebhook signs it; the
    // expected signature was computed with openssl, independently of the SDK.
    const PAYLOAD: &str = concat!(
        r#"{"data":{"id":"5f0c6f1e-1111-4a5b-9c1d-2e3f4a5b6c7d","#,
        r#""orderReferenceId":"0b6a4f9e-2222-4c3d-8e9f-a1b2c3d4e5f6","conversationId":"wc-1001","status":"3"},"#,
        r#""type":"order_lite","token":"9d8c7b6a-3333-4e5f-a6b7-c8d9e0f1a2b3"}"#
    );
    const SECRET: &str = "whsec_test";
    const TIMESTAMP: i64 = 1726480000;
    const SIGNATURE: &str = "3e003945e8122172a03ae1aa56fd099b56e6ac11a8472c03e33809c7c480aeb6";
    const LEGACY_SIGNATURE: &str =
        "402b9d6616b32be533a4fc0300e8db19c304a56137d3fb274c614f30f9be6fbb";

    fn header(signature: &str, timestamp: impl std::fmt::Display) -> String {
        format!("t={},v1={}", timestamp, signature)
    }

    fn at(now: i64) -> WebhookVerifyOptions {
        WebhookVerifyOptions::default().with_now(now)
    }

    fn verify(payload: &str, signature: &str, secret: &str, options: WebhookVerifyOptions) -> bool {
        WebhookModule::verify_webhook_with(payload, signature, secret, &options)
    }

    fn sign(payload: &str, secret: &str, timestamp: i64) -> String {
        let mut mac = WebhookModule::mac(secret).unwrap();
        mac.update(format!("{}.{}", timestamp, payload).as_bytes());
        hex(&mac.finalize().into_bytes())
    }

    fn event(payload: serde_json::Value) -> WebhookEvent {
        WebhookEvent::from_value(payload).unwrap()
    }

    #[test]
    fn test_verifies_the_tapsilat_signature_header() {
        assert_eq!(sign(PAYLOAD, SECRET, TIMESTAMP), SIGNATURE);
        assert!(verify(
            PAYLOAD,
            &header(SIGNATURE, TIMESTAMP),
            SECRET,
            at(TIMESTAMP + 10)
        ));
        assert!(verify(
            PAYLOAD,
            &format!(" t={}, v1={} ", TIMESTAMP, SIGNATURE.to_uppercase()),
            SECRET,
            at(TIMESTAMP)
        ));
        assert!(WebhookModule::verify_webhook_with(
            PAYLOAD.as_bytes(),
            &header(SIGNATURE, TIMESTAMP),
            SECRET,
            &at(TIMESTAMP)
        ));
    }

    #[test]
    fn test_rejects_a_tampered_body_or_the_wrong_secret() {
        let tampered = PAYLOAD.replace(r#""status":"3""#, r#""status":"13""#);
        let signed = header(SIGNATURE, TIMESTAMP);

        assert!(!verify(&tampered, &signed, SECRET, at(TIMESTAMP)));
        assert!(!verify(PAYLOAD, &signed, "other", at(TIMESTAMP)));
        assert!(!verify(PAYLOAD, &signed, "", at(TIMESTAMP)));
        assert!(!verify(PAYLOAD, "", SECRET, at(TIMESTAMP)));
    }

    #[test]
    fn test_rejects_a_replayed_or_malformed_header() {
        let signed = header(SIGNATURE, TIMESTAMP);
        let no_tolerance = at(TIMESTAMP).with_tolerance(0);

        assert!(!verify(PAYLOAD, &signed, SECRET, at(TIMESTAMP + 301)));
        assert!(!verify(PAYLOAD, &signed, SECRET, at(TIMESTAMP - 301)));
        assert!(verify(
            PAYLOAD,
            &signed,
            SECRET,
            at(TIMESTAMP + 86400).with_tolerance(0)
        ));
        assert!(!verify(
            PAYLOAD,
            &format!("v1={}", SIGNATURE),
            SECRET,
            no_tolerance
        ));
        assert!(!verify(
            PAYLOAD,
            &format!("t={}", TIMESTAMP),
            SECRET,
            no_tolerance
        ));
        assert!(!verify(
            PAYLOAD,
            &header(SIGNATURE, "1726480000.5"),
            SECRET,
            no_tolerance
        ));
        assert!(!verify(
            PAYLOAD,
            &header(SIGNATURE, "+1726480000"),
            SECRET,
            no_tolerance
        ));
        assert!(!verify(
            PAYLOAD,
            &header(SIGNATURE, "99999999999999999999999999"),
            SECRET,
            no_tolerance
        ));
        assert!(!verify(
            PAYLOAD,
            &header(SIGNATURE, "99999999999999999999999999"),
            SECRET,
            at(TIMESTAMP)
        ));
    }

    #[test]
    fn test_accepts_any_of_several_v1_signatures() {
        let signed = format!("t={},v1={},v1={}", TIMESTAMP, "0".repeat(64), SIGNATURE);

        assert!(verify(PAYLOAD, &signed, SECRET, at(TIMESTAMP)));
    }

    #[test]
    fn test_uses_the_current_time_by_default() {
        let now = chrono::Utc::now().timestamp();
        let signed = header(&sign(PAYLOAD, SECRET, now), now);

        assert!(WebhookModule::verify_webhook(PAYLOAD, &signed, SECRET).unwrap());
        assert!(verify(
            PAYLOAD,
            &signed,
            SECRET,
            WebhookVerifyOptions::default()
        ));
        assert!(
            !WebhookModule::verify_webhook(PAYLOAD, &header(SIGNATURE, TIMESTAMP), SECRET).unwrap()
        );
    }

    #[test]
    fn test_still_accepts_a_bare_signature_of_the_body() {
        assert_eq!(
            WebhookModule::create_signature(PAYLOAD, SECRET).unwrap(),
            LEGACY_SIGNATURE
        );
        assert!(WebhookModule::verify_webhook(PAYLOAD, LEGACY_SIGNATURE, SECRET).unwrap());
        assert!(WebhookModule::verify_webhook(
            PAYLOAD,
            &format!("sha256={}", LEGACY_SIGNATURE),
            SECRET
        )
        .unwrap());
        assert!(!WebhookModule::verify_webhook(PAYLOAD, LEGACY_SIGNATURE, "other").unwrap());
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
    fn test_decodes_only_hex() {
        assert_eq!(decode_hex("0aFf"), Some(vec![0x0a, 0xff]));
        assert_eq!(decode_hex("+a"), None);
        assert_eq!(decode_hex("abc"), None);
        assert_eq!(decode_hex("zz"), None);
    }

    #[test]
    fn test_verify_webhook_advanced_reports_the_reason() {
        let signed = header(SIGNATURE, TIMESTAMP);

        let config = WebhookModule::create_verification_config(SECRET.to_string(), None);
        let result = WebhookModule::verify_webhook_advanced(PAYLOAD, &signed, &config).unwrap();
        assert!(result.is_valid);
        assert!(result.error.is_none());

        let config = WebhookModule::create_verification_config(SECRET.to_string(), Some(300));
        let result = WebhookModule::verify_webhook_advanced(PAYLOAD, &signed, &config).unwrap();
        assert!(!result.is_valid);
        assert_eq!(
            result.error.as_deref(),
            Some("Signature timestamp is outside the tolerance")
        );

        let config = WebhookModule::create_verification_config("other".to_string(), None);
        let result = WebhookModule::verify_webhook_advanced(PAYLOAD, &signed, &config).unwrap();
        assert_eq!(result.error.as_deref(), Some("Invalid signature"));
    }

    #[test]
    fn test_webhook_verification_config() {
        let config =
            WebhookModule::create_verification_config("test_secret".to_string(), Some(300));

        assert_eq!(config.secret, "test_secret");
        assert_eq!(config.tolerance_seconds, Some(300));
    }

    #[test]
    fn test_parses_an_order_lite_callback() {
        let event = WebhookModule::parse_webhook(PAYLOAD).unwrap();

        assert_eq!(
            event.event_type.as_deref(),
            Some(WebhookEvent::TYPE_ORDER_LITE)
        );
        assert_eq!(
            event.token.as_deref(),
            Some("9d8c7b6a-3333-4e5f-a6b7-c8d9e0f1a2b3")
        );
        assert_eq!(event.data["id"], "5f0c6f1e-1111-4a5b-9c1d-2e3f4a5b6c7d");
        assert_eq!(
            event.reference_id().as_deref(),
            Some("0b6a4f9e-2222-4c3d-8e9f-a1b2c3d4e5f6")
        );
        assert_eq!(event.conversation_id().as_deref(), Some("wc-1001"));
        assert_eq!(event.status(), Some(OrderStatus::Paid));
        assert_eq!(event.status_code(), Some(3));
        assert_eq!(event.status().map(OrderStatus::name), Some("Paid"));
        assert!(event.is_paid());
        assert!(!event.is_failed());

        let decoded: WebhookEvent = serde_json::from_str(PAYLOAD).unwrap();
        assert_eq!(decoded, event);
    }

    #[test]
    fn test_parses_an_order_callback() {
        let event = event(json!({
            "type": "order",
            "token": "t-1",
            "data": {"id": "o-1", "reference_id": "ref-1", "conversation_id": "c-1", "status": 13},
        }));

        assert_eq!(event.reference_id().as_deref(), Some("ref-1"));
        assert_eq!(event.conversation_id().as_deref(), Some("c-1"));
        assert_eq!(event.status().map(OrderStatus::name), Some("Failure"));
        assert!(event.is_failed());
    }

    #[test]
    fn test_parses_an_order_payment_detail_callback() {
        let event = event(json!({
            "type": "order_payment_detail",
            "token": "t-2",
            "data": {
                "id": "d-1",
                "order_reference_id": "ref-2",
                "conversation_id": "c-2",
                "order": {"reference_id": "ref-2", "id": "o-2", "status": "Paid"},
                "order_payment_status": {"code": 0},
            },
        }));

        assert_eq!(event.reference_id().as_deref(), Some("ref-2"));
        assert_eq!(event.conversation_id().as_deref(), Some("c-2"));
        assert_eq!(event.status(), Some(OrderStatus::Paid));
    }

    #[test]
    fn test_parses_callbacks_without_a_status() {
        let extended = event(json!({
            "type": "extended_order",
            "data": {
                "id": "o-3",
                "reference_id": "ref-3",
                "terms": [{"reference_id": "term-1", "status": 2}],
            },
        }));
        assert_eq!(extended.reference_id().as_deref(), Some("ref-3"));
        assert_eq!(extended.conversation_id(), None);
        assert_eq!(extended.status_code(), None);
        assert!(!extended.is_paid());

        let facilitator = event(json!({
            "type": "payment_facilitator",
            "data": {"id": "p-1", "payment_id": "pay-1", "reference_id": "ref-4"},
        }));
        assert_eq!(facilitator.reference_id().as_deref(), Some("ref-4"));
    }

    #[test]
    fn test_ignores_a_zero_uuid() {
        let event = event(json!({
            "type": "order_payment_detail",
            "data": {
                "order_reference_id": "00000000-0000-0000-0000-000000000000",
                "order": {"reference_id": "ref-5"},
            },
        }));

        assert_eq!(event.reference_id().as_deref(), Some("ref-5"));
    }

    #[test]
    fn test_reads_the_whole_payload_without_a_data_object() {
        let event = event(json!({"reference_id": 42, "status": 13.0}));

        assert_eq!(event.event_type, None);
        assert_eq!(event.data, event.payload);
        assert_eq!(event.reference_id().as_deref(), Some("42"));
        assert_eq!(event.status(), Some(OrderStatus::Failure));
    }

    #[test]
    fn test_rejects_a_payload_that_is_not_a_json_object() {
        assert!(WebhookEvent::from_payload(r#""order""#).is_err());
        assert!(WebhookModule::parse_webhook("not json").is_err());
        assert!(serde_json::from_str::<WebhookEvent>("[]").is_err());
    }

    #[test]
    fn test_order_status_names_and_groups() {
        let parse = |value: &str| value.parse::<OrderStatus>().ok();

        assert_eq!(
            parse("waiting for payment"),
            Some(OrderStatus::WaitingForPayment)
        );
        assert_eq!(parse("waiting for payment").map(OrderStatus::code), Some(7));
        assert_eq!(parse("21"), Some(OrderStatus::Expired));
        assert_eq!(
            OrderStatus::from_code(15).map(OrderStatus::name),
            Some("Partially refunded")
        );
        assert_eq!(parse("SUCCESS"), None);
        assert_eq!(OrderStatus::from_code(99), None);
        assert_eq!(OrderStatus::from_code(0), None);
        assert_eq!(OrderStatus::from_json(&serde_json::Value::Null), None);
        assert_eq!(
            OrderStatus::from_json(&json!(13)),
            Some(OrderStatus::Failure)
        );
        assert_eq!(
            OrderStatus::from_json(&json!(13.0)),
            Some(OrderStatus::Failure)
        );
        assert_eq!(OrderStatus::from_json(&json!(13.5)), None);
        assert_eq!(
            OrderStatus::from_json(&json!(" 3 ")),
            Some(OrderStatus::Paid)
        );

        assert!(parse("Sub merchant payment approved").is_some_and(OrderStatus::is_paid));
        assert!(OrderStatus::from_code(9).is_some_and(OrderStatus::is_paid));
        assert!(!OrderStatus::WaitingForPayment.is_paid());
        assert!(parse("Expired").is_some_and(OrderStatus::is_failed));
        assert!(!OrderStatus::Paid.is_failed());
        assert!(parse("10").is_some_and(OrderStatus::is_refunded));

        for (index, status) in OrderStatus::ALL.into_iter().enumerate() {
            assert_eq!(usize::from(status.code()), index + 1);
            assert_eq!(parse(status.name()), Some(status));
        }
        assert_eq!(serde_json::to_string(&OrderStatus::Paid).unwrap(), "3");
        assert_eq!(
            serde_json::from_str::<OrderStatus>(r#""Paid""#).unwrap(),
            OrderStatus::Paid
        );
    }
}
