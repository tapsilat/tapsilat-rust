use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Order {
    pub id: Option<String>,
    #[serde(default)] // Handle missing id if needed, or Option
    pub reference_id: Option<String>,
    
    // Amount fields are strings in JSON logs
    pub amount: Option<String>, 
    pub total: Option<String>,
    pub paid_amount: Option<String>,
    pub refunded_amount: Option<String>,
    
    pub currency: Option<String>, // Relaxed from enum to avoid validation errors
    
    pub status: Option<i32>, // Status is int in logs
    pub status_enum: Option<String>,
    
    pub description: Option<String>,
    pub buyer: Option<Buyer>,
    pub items: Option<Vec<OrderItem>>, // Items missing in top level? JSON has basket_items?
    
    // JSON has basket_items, not items!
    #[serde(rename = "basket_items")]
    pub basket_items: Option<Vec<BasketItemDTO>>,
    
    pub callback_url: Option<String>,
    pub checkout_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub metadata: Option<Vec<MetadataDTO>>, // JSON metadata is array of key/value
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
    pub currency: String,
    pub locale: String,
    pub buyer: CreateBuyerRequest,
    #[serde(rename = "basket_items")]
    pub basket_items: Option<Vec<BasketItemDTO>>,
    #[serde(rename = "billing_address")]
    pub billing_address: Option<BillingAddressDTO>,
    #[serde(rename = "checkout_design")]
    pub checkout_design: Option<CheckoutDesignDTO>,
    #[serde(rename = "conversation_id")]
    pub conversation_id: Option<String>,
    #[serde(rename = "enabled_installments", skip_serializing_if = "Option::is_none")]
    pub enabled_installments: Option<Vec<i32>>,
    #[serde(rename = "external_reference_id")]
    pub external_reference_id: Option<String>,
    pub metadata: Option<Vec<MetadataDTO>>,
    #[serde(rename = "order_cards")]
    pub order_cards: Option<OrderCardDTO>,
    #[serde(rename = "paid_amount")]
    pub paid_amount: Option<f64>,
    #[serde(rename = "partial_payment")]
    pub partial_payment: Option<bool>,
    #[serde(rename = "payment_failure_url")]
    pub payment_failure_url: Option<String>,
    #[serde(rename = "payment_methods")]
    pub payment_methods: Option<bool>,
    #[serde(rename = "payment_mode")]
    pub payment_mode: Option<String>,
    #[serde(rename = "payment_options")]
    pub payment_options: Option<Vec<String>>,
    #[serde(rename = "payment_success_url")]
    pub payment_success_url: Option<String>,
    #[serde(rename = "payment_terms")]
    pub payment_terms: Option<Vec<PaymentTermDTO>>,
    #[serde(rename = "pf_sub_merchant")]
    pub pf_sub_merchant: Option<OrderPFSubMerchantDTO>,
    #[serde(rename = "redirect_failure_url")]
    pub redirect_failure_url: Option<String>,
    #[serde(rename = "redirect_success_url")]
    pub redirect_success_url: Option<String>,
    #[serde(rename = "shipping_address")]
    pub shipping_address: Option<ShippingAddressDTO>,
    #[serde(rename = "sub_organization")]
    pub sub_organization: Option<SubOrganizationDTO>,
    pub submerchants: Option<Vec<SubmerchantDTO>>,
    #[serde(rename = "tax_amount")]
    pub tax_amount: Option<f64>,
    #[serde(rename = "three_d_force")]
    pub three_d_force: Option<bool>,
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
    pub order_id: Option<String>,
    pub reference_id: Option<String>,
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: Option<String>,
    pub reference_id: Option<String>,
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundOrderRequest {
    pub amount: f64,
    #[serde(rename = "reference_id")]
    pub reference_id: String,
    #[serde(rename = "order_item_id")]
    pub order_item_id: Option<String>,
    #[serde(rename = "order_item_payment_id")]
    pub order_item_payment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundOrderResponse {
    pub order: Order,
    pub refund_amount: f64,
    pub refund_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataDTO {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketItemDTO {
    pub category1: Option<String>,
    pub category2: Option<String>,
    pub commission_amount: Option<f64>,
    pub coupon: Option<String>,
    pub coupon_discount: Option<f64>,
    pub data: Option<String>,
    pub id: Option<String>,
    pub item_type: Option<String>,
    pub name: Option<String>,
    pub paid_amount: Option<f64>,
    pub payer: Option<BasketItemPayerDTO>,
    pub price: Option<f64>,
    pub quantity: Option<i32>,
    pub quantity_float: Option<f64>,
    pub quantity_unit: Option<String>,
    pub sub_merchant_key: Option<String>,
    pub sub_merchant_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketItemPayerDTO {
    pub address: Option<String>,
    pub reference_id: Option<String>,
    pub tax_office: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub vat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddressDTO {
    pub address: Option<String>,
    pub billing_type: Option<String>,
    pub citizenship: Option<String>,
    pub city: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub country: Option<String>,
    pub district: Option<String>,
    pub tax_office: Option<String>,
    pub title: Option<String>,
    pub vat_number: Option<String>,
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutDesignDTO {
    pub input_background_color: Option<String>,
    pub input_text_color: Option<String>,
    pub label_text_color: Option<String>,
    pub left_background_color: Option<String>,
    pub logo: Option<String>,
    pub order_detail_html: Option<String>,
    pub pay_button_color: Option<String>,
    pub redirect_url: Option<String>,
    pub right_background_color: Option<String>,
    pub text_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCardDTO {
    pub card_id: String,
    pub card_sequence: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTermDTO {
    pub amount: Option<f64>,
    pub data: Option<String>,
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub required: Option<bool>,
    pub status: Option<String>,
    pub term_reference_id: Option<String>,
    pub term_sequence: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPFSubMerchantDTO {
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_iso_code: Option<String>,
    pub id: Option<String>,
    pub mcc: Option<String>,
    pub name: Option<String>,
    pub org_id: Option<String>,
    pub postal_code: Option<String>,
    pub submerchant_nin: Option<String>,
    pub submerchant_url: Option<String>,
    pub terminal_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddressDTO {
    pub address: Option<String>,
    pub city: Option<String>,
    pub contact_name: Option<String>,
    pub country: Option<String>,
    pub shipping_date: Option<String>,
    pub tracking_code: Option<String>,
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubOrganizationDTO {
    pub acquirer: Option<String>,
    pub address: Option<String>,
    pub contact_first_name: Option<String>,
    pub contact_last_name: Option<String>,
    pub currency: Option<String>,
    pub email: Option<String>,
    pub gsm_number: Option<String>,
    pub iban: Option<String>,
    pub identity_number: Option<String>,
    pub legal_company_title: Option<String>,
    pub organization_name: Option<String>,
    pub sub_merchant_external_id: Option<String>,
    pub sub_merchant_key: Option<String>,
    pub sub_merchant_type: Option<String>,
    pub tax_number: Option<String>,
    pub tax_office: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmerchantDTO {
    pub amount: Option<f64>,
    pub merchant_reference_id: Option<String>,
    pub order_basket_item_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAccountingRequest {
    #[serde(rename = "order_reference_id")]
    pub order_reference_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPostAuthRequest {
    pub amount: f64,
    #[serde(rename = "reference_id")]
    pub reference_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPaymentTermCreateDTO {
    pub order_id: String,
    pub term_reference_id: String,
    pub amount: f64,
    pub due_date: String,
    pub term_sequence: i32,
    pub required: bool,
    pub status: String,
    pub data: Option<String>,
    pub paid_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPaymentTermUpdateDTO {
    pub term_reference_id: String,
    pub amount: Option<f64>,
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub required: Option<bool>,
    pub status: Option<String>,
    pub term_sequence: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTermRefundRequest {
    pub term_id: String,
    pub amount: f64,
    pub reference_id: Option<String>,
    pub term_payment_id: Option<String>,
}


// Re-export Buyer from buyer.rs
use crate::types::buyer::Buyer;
pub use crate::types::buyer::CreateBuyerRequest;
