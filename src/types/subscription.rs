use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionBilling {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "contact_name", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "vat_number", skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "identity_number", skip_serializing_if = "Option::is_none")]
    pub identity_number: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubscriptionOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "payment_date", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "payment_url", skip_serializing_if = "Option::is_none")]
    pub payment_url: Option<String>,
    #[serde(rename = "reference_id", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubscriptionDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycles_completed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(
        rename = "external_reference_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_id: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<SubscriptionOrder>>,
    #[serde(rename = "payment_date", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<i32>,
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SubscriptionUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubscriptionListItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycles_completed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(
        rename = "external_reference_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_id: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "payment_date", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<i32>,
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "reference_id", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<SubscriptionBilling>,
    #[serde(rename = "card_id", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<i32>,
    #[serde(
        rename = "external_reference_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_id: Option<String>,
    #[serde(rename = "failure_url", skip_serializing_if = "Option::is_none")]
    pub failure_url: Option<String>,
    /// The billing interval, for example "month".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// The number of intervals between payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Deprecated: ignored by the server; the subscription start date sets the billing anchor.
    #[serde(rename = "payment_date", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<i32>,
    /// Deprecated: the legacy day-count cadence. Use `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "success_url", skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SubscriptionUser>,
    /// Deprecated: not part of the API; the server ignores it.
    #[serde(rename = "price_option", skip_serializing_if = "Option::is_none")]
    pub price_option: Option<SubscriptionPriceOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionPriceOption {
    pub count: i32,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubscriptionCreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "order_reference_id", skip_serializing_if = "Option::is_none")]
    pub order_reference_id: Option<String>,
    #[serde(rename = "reference_id", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionGetRequest {
    #[serde(
        rename = "external_reference_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_id: Option<String>,
    #[serde(rename = "reference_id", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionCancelRequest {
    #[serde(
        rename = "external_reference_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_id: Option<String>,
    #[serde(rename = "reference_id", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionRedirectRequest {
    #[serde(rename = "subscription_id", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubscriptionRedirectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
