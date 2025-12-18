use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionBilling {
    pub address: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "contact_name")]
    pub contact_name: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "vat_number")]
    pub vat_number: Option<String>,
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUser {
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "identity_number")]
    pub identity_number: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionOrder {
    pub amount: Option<String>,
    pub currency: Option<String>,
    #[serde(rename = "payment_date")]
    pub payment_date: Option<String>,
    #[serde(rename = "payment_url")]
    pub payment_url: Option<String>,
    #[serde(rename = "reference_id")]
    pub reference_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDetail {
    pub amount: Option<String>,
    pub currency: Option<String>,
    #[serde(rename = "due_date")]
    pub due_date: Option<String>,
    #[serde(rename = "external_reference_id")]
    pub external_reference_id: Option<String>,
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    pub orders: Option<Vec<SubscriptionOrder>>,
    #[serde(rename = "payment_date")]
    pub payment_date: Option<i32>,
    #[serde(rename = "payment_status")]
    pub payment_status: Option<String>,
    pub period: Option<i32>,
    pub title: Option<String>,
    pub user: Option<SubscriptionUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionListItem {
    pub amount: Option<String>,
    pub currency: Option<String>,
    #[serde(rename = "external_reference_id")]
    pub external_reference_id: Option<String>,
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    #[serde(rename = "payment_date")]
    pub payment_date: Option<i32>,
    #[serde(rename = "payment_status")]
    pub payment_status: Option<String>,
    pub period: Option<i32>,
    #[serde(rename = "reference_id")]
    pub reference_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCreateRequest {
    pub amount: Option<f64>,
    pub billing: Option<SubscriptionBilling>,
    #[serde(rename = "card_id")]
    pub card_id: Option<String>,
    pub currency: Option<String>,
    pub cycle: Option<i32>,
    #[serde(rename = "external_reference_id")]
    pub external_reference_id: Option<String>,
    #[serde(rename = "failure_url")]
    pub failure_url: Option<String>,
    #[serde(rename = "payment_date")]
    pub payment_date: Option<i32>,
    pub period: Option<i32>,
    #[serde(rename = "success_url")]
    pub success_url: Option<String>,
    pub title: Option<String>,
    pub user: Option<SubscriptionUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCreateResponse {
    pub code: Option<i32>,
    pub message: Option<String>,
    #[serde(rename = "order_reference_id")]
    pub order_reference_id: Option<String>,
    #[serde(rename = "reference_id")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionGetRequest {
    #[serde(rename = "external_reference_id")]
    pub external_reference_id: Option<String>,
    #[serde(rename = "reference_id")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCancelRequest {
    #[serde(rename = "external_reference_id")]
    pub external_reference_id: Option<String>,
    #[serde(rename = "reference_id")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRedirectRequest {
    #[serde(rename = "subscription_id")]
    pub subscription_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRedirectResponse {
    pub url: Option<String>,
}
