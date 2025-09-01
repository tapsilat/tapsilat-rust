use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub amount: f64,
    pub currency: Currency,
    pub status: OrderStatus,
    pub description: Option<String>,
    pub buyer: Option<Buyer>,
    pub items: Vec<OrderItem>,
    pub callback_url: Option<String>,
    pub checkout_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "refunded")]
    Refunded,
    #[serde(rename = "partially_refunded")]
    PartiallyRefunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "TRY")]
    TRY,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "GBP")]
    GBP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub amount: f64,
    pub currency: Currency,
    pub locale: Option<String>,
    #[serde(rename = "conversationId")]
    pub conversation_id: Option<String>,
    pub description: Option<String>,
    pub buyer: Option<CreateBuyerRequest>,
    pub items: Vec<CreateOrderItemRequest>,
    #[serde(rename = "callbackUrl")]
    pub callback_url: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderItemRequest {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: Order,
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub reference_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundOrderRequest {
    pub amount: Option<f64>, // None for full refund
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundOrderResponse {
    pub order: Order,
    pub refund_amount: f64,
    pub refund_id: String,
}

// Re-export Buyer from buyer.rs
use crate::types::buyer::Buyer;
pub use crate::types::buyer::CreateBuyerRequest;
