use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_type: WebhookEventType,
    pub data: WebhookData,
    pub timestamp: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventType {
    #[serde(rename = "order.completed")]
    OrderCompleted,
    #[serde(rename = "order.failed")]
    OrderFailed,
    #[serde(rename = "order.cancelled")]
    OrderCancelled,
    #[serde(rename = "order.refunded")]
    OrderRefunded,
    #[serde(rename = "payment.completed")]
    PaymentCompleted,
    #[serde(rename = "payment.failed")]
    PaymentFailed,
    #[serde(rename = "installment.completed")]
    InstallmentCompleted,
    #[serde(rename = "installment.failed")]
    InstallmentFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookData {
    pub order_id: Option<String>,
    pub payment_id: Option<String>,
    pub installment_id: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone)]
pub struct WebhookVerificationResult {
    pub is_valid: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WebhookVerificationConfig {
    pub secret: String,
    pub tolerance_seconds: Option<u64>, // For timestamp validation
}
