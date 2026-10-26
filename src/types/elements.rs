//! Request and response types for Tapsilat Elements sessions.
//!
//! Tapsilat Elements is the native Android and iOS card sheet. Your backend creates a session
//! with its secret API key and hands only the session's `client_secret` to the app, which opens
//! the sheet with it. Card data goes from the sheet to Tapsilat encrypted and never reaches your
//! backend.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Creates an Elements session with `POST /elements/sessions`.
///
/// `purpose` is always sent; unset optional fields are left out of the request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElementsSessionCreateRequest {
    /// [`ElementsSession::PURPOSE_PAY`] or [`ElementsSession::PURPOSE_TOKENIZE`].
    pub purpose: String,
    /// The order to pay. Required for a `pay` session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference_id: Option<String>,
    /// The card owner of a `tokenize` session, scoped to your organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    /// The tokenize mode: [`Self::MODE_SECURE`], [`Self::MODE_HALF_SECURE`] or
    /// [`Self::MODE_NON_SECURE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Lets the buyer save the card on a `pay` session.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub save_card: bool,
}

impl ElementsSessionCreateRequest {
    /// Tokenize with 3D Secure.
    pub const MODE_SECURE: i32 = 1;
    /// Tokenize half secure.
    pub const MODE_HALF_SECURE: i32 = 2;
    /// Tokenize without 3D Secure.
    pub const MODE_NON_SECURE: i32 = 3;
}

/// The card an Elements session holds: brand, last four digits and expiry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ElementsCardSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The card brand; see the `BRAND_*` constants. Other values may be added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
}

impl ElementsCardSummary {
    pub const BRAND_VISA: &'static str = "visa";
    pub const BRAND_MASTERCARD: &'static str = "mastercard";
    pub const BRAND_AMEX: &'static str = "amex";
    pub const BRAND_TROY: &'static str = "troy";
    pub const BRAND_DISCOVER: &'static str = "discover";
    pub const BRAND_UNIONPAY: &'static str = "unionpay";
    pub const BRAND_UNKNOWN: &'static str = "unknown";
}

/// Why the last attempt of an Elements session failed, such as `card_declined`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ElementsError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
}

/// The authoritative state of an Elements session, from `GET /elements/sessions/{id}`.
///
/// Every field may be absent. Timestamps are RFC 3339 strings. `purpose` and `status` are kept
/// as strings so that values added later still decode; compare them with the `PURPOSE_*` and
/// `STATUS_*` constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ElementsSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// The card, once one was entered. `None` when absent or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ElementsCardSummary>,
    /// The error of the last failed attempt. `None` when absent or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<ElementsError>,
}

impl ElementsSession {
    /// Saves a card for a user.
    pub const PURPOSE_TOKENIZE: &'static str = "tokenize";
    /// Pays an order.
    pub const PURPOSE_PAY: &'static str = "pay";

    pub const STATUS_REQUIRES_PAYMENT_METHOD: &'static str = "requires_payment_method";
    pub const STATUS_PROCESSING: &'static str = "processing";
    pub const STATUS_REQUIRES_ACTION: &'static str = "requires_action";
    pub const STATUS_SUCCEEDED: &'static str = "succeeded";
    pub const STATUS_FAILED: &'static str = "failed";
    pub const STATUS_EXPIRED: &'static str = "expired";
    pub const STATUS_CANCELED: &'static str = "canceled";
}

/// A newly created Elements session, from `POST /elements/sessions`: the session plus its
/// `client_secret` and `elements_base_url`.
///
/// Hand `client_secret` to the app that opens the card sheet; it is returned only once. Do not
/// log or store it. The `Debug` output prints it as `[redacted]`; JSON serialization keeps it,
/// so the session can still be sent to the app.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ElementsSessionWithSecret {
    #[serde(flatten)]
    pub session: ElementsSession,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements_base_url: Option<String>,
}

impl fmt::Debug for ElementsSessionWithSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElementsSessionWithSecret")
            .field("session", &self.session)
            .field("client_secret", &"[redacted]")
            .field("elements_base_url", &self.elements_base_url)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "es_test_1_secret_x9Qw";

    fn created() -> ElementsSessionWithSecret {
        ElementsSessionWithSecret {
            session: ElementsSession {
                id: Some("es_test_1".to_string()),
                status: Some(ElementsSession::STATUS_REQUIRES_PAYMENT_METHOD.to_string()),
                ..Default::default()
            },
            client_secret: SECRET.to_string(),
            elements_base_url: Some("https://checkout.tapsilat.dev".to_string()),
        }
    }

    #[test]
    fn debug_redacts_the_client_secret() {
        let session = created();
        for printed in [format!("{:?}", session), format!("{:#?}", session)] {
            assert!(!printed.contains(SECRET), "{printed}");
            assert!(printed.contains("es_test_1"), "{printed}");
            assert!(
                printed.contains("https://checkout.tapsilat.dev"),
                "{printed}"
            );
        }
        assert!(format!("{:?}", session).contains(r#"client_secret: "[redacted]""#));
    }

    #[test]
    fn json_keeps_the_client_secret() {
        let encoded = serde_json::to_value(created()).unwrap();
        assert_eq!(
            encoded,
            serde_json::json!({
                "id": "es_test_1",
                "status": "requires_payment_method",
                "client_secret": SECRET,
                "elements_base_url": "https://checkout.tapsilat.dev"
            })
        );
        let decoded: ElementsSessionWithSecret = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded, created());
    }

    #[test]
    fn session_decodes_null_absent_and_unknown_values() {
        let session: ElementsSession = serde_json::from_str(
            r#"{"id":"es_mock_1","status":"some_new_status","card":null,"last_error":null,"created_at":"not a timestamp","new_field":{"x":1}}"#,
        )
        .unwrap();
        assert_eq!(session.status.as_deref(), Some("some_new_status"));
        assert_eq!(session.created_at.as_deref(), Some("not a timestamp"));
        assert_eq!(session.card, None);
        assert_eq!(session.last_error, None);
        assert_eq!(session.purpose, None);
        assert_eq!(session.attempts, None);

        let empty: ElementsSession = serde_json::from_str("{}").unwrap();
        assert_eq!(empty, ElementsSession::default());
    }

    #[test]
    fn create_request_omits_unset_fields() {
        let request = ElementsSessionCreateRequest {
            purpose: ElementsSession::PURPOSE_TOKENIZE.to_string(),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({"purpose": "tokenize"})
        );
    }
}
