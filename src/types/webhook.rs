use crate::error::{Result, TapsilatError};
use crate::types::OrderStatus;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// The reference ID of an order payment detail that belongs to no order.
const ZERO_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// A callback Tapsilat posts to an organization's callback URLs or to an order's
/// `payment_success_url` and `payment_failure_url`:
/// `{"type": "...", "data": {...}, "token": "<delivery id>"}`.
///
/// The shape of `data` depends on the type, which each organization chooses in the panel:
/// "order_lite" (the default), "order", "order_payment_detail", "extended_order" or
/// "payment_facilitator". The getters read the order's reference ID, conversation ID and
/// status from whichever shape arrived.
///
/// Verify the `Tapsilat-Signature` header with
/// [`WebhookModule::verify_webhook`](crate::WebhookModule::verify_webhook) before trusting a
/// callback.
#[derive(Debug, Clone, PartialEq)]
pub struct WebhookEvent {
    /// The callback's `type`; see the `TYPE_*` constants.
    pub event_type: Option<String>,
    /// The callback's `data` object, or the whole payload when it has none.
    pub data: Value,
    /// The callback's `token`, unique per delivery.
    pub token: Option<String>,
    /// The decoded payload.
    pub payload: Value,
}

impl WebhookEvent {
    pub const TYPE_ORDER: &'static str = "order";
    pub const TYPE_ORDER_LITE: &'static str = "order_lite";
    pub const TYPE_ORDER_PAYMENT_DETAIL: &'static str = "order_payment_detail";
    pub const TYPE_EXTENDED_ORDER: &'static str = "extended_order";
    pub const TYPE_PAYMENT_FACILITATOR: &'static str = "payment_facilitator";

    /// Decodes a raw callback body. Fails when the body is not a JSON object.
    pub fn from_payload(payload: impl AsRef<[u8]>) -> Result<Self> {
        let value: Value = serde_json::from_slice(payload.as_ref()).map_err(|e| {
            TapsilatError::InvalidResponse(format!("Failed to parse webhook payload: {}", e))
        })?;
        Self::from_value(value)
    }

    /// Builds an event from a decoded callback body. Fails when it is not a JSON object.
    pub fn from_value(payload: Value) -> Result<Self> {
        let Value::Object(fields) = &payload else {
            return Err(TapsilatError::InvalidResponse(
                "The webhook payload is not a JSON object".to_string(),
            ));
        };
        let text = |key: &str| fields.get(key).and_then(Value::as_str).map(str::to_string);
        let event_type = text("type");
        let token = text("token");
        let data = match fields.get("data") {
            Some(data @ Value::Object(_)) => data.clone(),
            _ => payload.clone(),
        };
        Ok(Self {
            event_type,
            data,
            token,
            payload,
        })
    }

    /// The order's reference ID, whatever the callback type.
    pub fn reference_id(&self) -> Option<String> {
        first_id([
            self.data.get("orderReferenceId"),
            self.data.get("order_reference_id"),
            self.data.get("reference_id"),
            self.order().and_then(|order| order.get("reference_id")),
        ])
    }

    /// The conversation ID the order was created with.
    pub fn conversation_id(&self) -> Option<String> {
        first_id([
            self.data.get("conversationId"),
            self.data.get("conversation_id"),
        ])
    }

    /// The order status, or `None` when this callback type carries none.
    pub fn status(&self) -> Option<OrderStatus> {
        self.data
            .get("status")
            .and_then(OrderStatus::from_json)
            .or_else(|| {
                self.order()
                    .and_then(|order| order.get("status"))
                    .and_then(OrderStatus::from_json)
            })
    }

    /// The order status code, such as 3 for Paid.
    pub fn status_code(&self) -> Option<u8> {
        self.status().map(OrderStatus::code)
    }

    /// Whether the order's payment was captured. See [`OrderStatus::is_paid`].
    pub fn is_paid(&self) -> bool {
        self.status().is_some_and(OrderStatus::is_paid)
    }

    /// Whether the order will not be paid. See [`OrderStatus::is_failed`].
    pub fn is_failed(&self) -> bool {
        self.status().is_some_and(OrderStatus::is_failed)
    }

    /// Whether the order's payment was refunded. See [`OrderStatus::is_refunded`].
    pub fn is_refunded(&self) -> bool {
        self.status().is_some_and(OrderStatus::is_refunded)
    }

    fn order(&self) -> Option<&Value> {
        self.data.get("order").filter(|order| order.is_object())
    }
}

/// Returns the first non-empty string, or integer as decimal text, skipping the zero UUID.
fn first_id<'a>(values: impl IntoIterator<Item = Option<&'a Value>>) -> Option<String> {
    values.into_iter().flatten().find_map(|value| match value {
        Value::String(text) if !text.is_empty() && text != ZERO_UUID => Some(text.clone()),
        Value::Number(number) if number.is_i64() || number.is_u64() => Some(number.to_string()),
        _ => None,
    })
}

impl Serialize for WebhookEvent {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.payload.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::from_value(Value::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct WebhookVerificationResult {
    pub is_valid: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WebhookVerificationConfig {
    pub secret: String,
    /// Seconds the signature timestamp may differ from the current time; `None` or 0
    /// skips the timestamp check.
    pub tolerance_seconds: Option<u64>,
}
