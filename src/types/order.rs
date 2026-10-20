use crate::error::TapsilatError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// An order as returned by the order detail endpoints.
///
/// Every field is optional because the server omits empty values.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Order {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    // Amount fields are strings in JSON logs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_paid_amount: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>, // Relaxed from enum to avoid validation errors

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>, // Status is int in logs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_enum: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer: Option<Buyer>,
    /// Deprecated: the API returns basket items in `basket_items`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub basket_items: Option<Vec<BasketItemDTO>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddressDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddressDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_design: Option<CheckoutDesignDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<Vec<OrderPaymentTermDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_requests: Option<Vec<OrderPaymentRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_cancel_requests: Option<Vec<OrderRefundCancelRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_failure_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_failure_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<MetadataDTO>>, // JSON metadata is array of key/value
}

/// The order detail returned by [`crate::TapsilatClient::get_order`] and
/// [`crate::TapsilatClient::get_order_by_conversation_id`].
pub type OrderDetail = Order;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// An order status. Callbacks and order details carry its numeric code, and
/// GET /order/{reference_id}/status returns its name.
///
/// A status can be read from a code ([`OrderStatus::from_code`]), from a numeric string
/// such as order_lite callbacks send or a name such as "Paid", case-insensitively
/// ([`str::parse`]), or from a JSON value holding either ([`OrderStatus::from_json`]).
/// It serializes as its code and deserializes from any of these forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    Received = 1,
    Unpaid = 2,
    Paid = 3,
    Processing = 4,
    Shipped = 5,
    OnHold = 6,
    WaitingForPayment = 7,
    Cancelled = 8,
    Completed = 9,
    Refunded = 10,
    Fraud = 11,
    Rejected = 12,
    Failure = 13,
    Retrying = 14,
    PartiallyRefunded = 15,
    SubMerchantPaymentApproved = 16,
    SubMerchantPaymentDisapproved = 17,
    SubMerchantPaymentErrored = 18,
    StillHasUnpaidInstallments = 19,
    StillHasUnpaidTerms = 20,
    Expired = 21,
    StillHasUnpaidSubMerchantPayments = 22,
    PartiallyPaid = 23,
    Terminated = 24,
    CardTokenization = 25,
    PreAuthorized = 26,
    Disputed = 27,
    PartiallyDisputed = 28,
    Suspect = 29,
}

impl OrderStatus {
    /// Every status, in code order.
    pub const ALL: [OrderStatus; 29] = [
        Self::Received,
        Self::Unpaid,
        Self::Paid,
        Self::Processing,
        Self::Shipped,
        Self::OnHold,
        Self::WaitingForPayment,
        Self::Cancelled,
        Self::Completed,
        Self::Refunded,
        Self::Fraud,
        Self::Rejected,
        Self::Failure,
        Self::Retrying,
        Self::PartiallyRefunded,
        Self::SubMerchantPaymentApproved,
        Self::SubMerchantPaymentDisapproved,
        Self::SubMerchantPaymentErrored,
        Self::StillHasUnpaidInstallments,
        Self::StillHasUnpaidTerms,
        Self::Expired,
        Self::StillHasUnpaidSubMerchantPayments,
        Self::PartiallyPaid,
        Self::Terminated,
        Self::CardTokenization,
        Self::PreAuthorized,
        Self::Disputed,
        Self::PartiallyDisputed,
        Self::Suspect,
    ];

    /// Statuses of an order whose payment was captured.
    pub const PAID_STATUSES: [OrderStatus; 3] = [
        Self::Paid,
        Self::Completed,
        Self::SubMerchantPaymentApproved,
    ];

    /// Statuses of an order that will not be paid.
    pub const FAILED_STATUSES: [OrderStatus; 6] = [
        Self::Cancelled,
        Self::Fraud,
        Self::Rejected,
        Self::Failure,
        Self::Expired,
        Self::Terminated,
    ];

    /// Statuses of an order whose payment was refunded in full or in part.
    pub const REFUNDED_STATUSES: [OrderStatus; 2] = [Self::Refunded, Self::PartiallyRefunded];

    /// The numeric status code, such as 3 for [`OrderStatus::Paid`].
    pub fn code(self) -> u8 {
        self as u8
    }

    /// The status name that GET /order/{reference_id}/status returns, such as "Paid".
    pub fn name(self) -> &'static str {
        match self {
            Self::Received => "Received",
            Self::Unpaid => "Unpaid",
            Self::Paid => "Paid",
            Self::Processing => "Processing",
            Self::Shipped => "Shipped",
            Self::OnHold => "On hold",
            Self::WaitingForPayment => "Waiting for payment",
            Self::Cancelled => "Cancelled",
            Self::Completed => "Completed",
            Self::Refunded => "Refunded",
            Self::Fraud => "Fraud",
            Self::Rejected => "Rejected",
            Self::Failure => "Failure",
            Self::Retrying => "Retrying",
            Self::PartiallyRefunded => "Partially refunded",
            Self::SubMerchantPaymentApproved => "Sub merchant payment approved",
            Self::SubMerchantPaymentDisapproved => "Sub merchant payment disapproved",
            Self::SubMerchantPaymentErrored => "Sub merchant payment errored",
            Self::StillHasUnpaidInstallments => "Still has unpaid installments",
            Self::StillHasUnpaidTerms => "Still has unpaid terms",
            Self::Expired => "Expired",
            Self::StillHasUnpaidSubMerchantPayments => "Still has unpaid sub merchant payments",
            Self::PartiallyPaid => "Partially Paid",
            Self::Terminated => "Terminated",
            Self::CardTokenization => "Card Tokenization",
            Self::PreAuthorized => "Pre Authorized",
            Self::Disputed => "Disputed",
            Self::PartiallyDisputed => "Partially disputed",
            Self::Suspect => "Suspect",
        }
    }

    /// Returns the status with this code, or `None` when no status has it.
    pub fn from_code(code: impl Into<i64>) -> Option<Self> {
        let index = usize::try_from(code.into().checked_sub(1)?).ok()?;
        Self::ALL.get(index).copied()
    }

    /// Returns the status with this name, compared case-insensitively.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim();
        Self::ALL
            .into_iter()
            .find(|status| status.name().eq_ignore_ascii_case(name))
    }

    /// Reads a status from a JSON number such as `3` (or `3.0`), a numeric string such as
    /// `"3"`, or a name such as `"Paid"`. Returns `None` for anything else.
    pub fn from_json(value: &serde_json::Value) -> Option<Self> {
        match value {
            serde_json::Value::Number(number) => match number.as_i64() {
                Some(code) => Self::from_code(code),
                None => number
                    .as_f64()
                    .filter(|code| code.fract() == 0.0 && (1.0..=29.0).contains(code))
                    .and_then(|code| Self::from_code(code as i64)),
            },
            serde_json::Value::String(text) => text.parse().ok(),
            _ => None,
        }
    }

    /// Whether the order's payment was captured: Paid, Completed or Sub merchant payment approved.
    pub fn is_paid(self) -> bool {
        Self::PAID_STATUSES.contains(&self)
    }

    /// Whether the order will not be paid: Cancelled, Fraud, Rejected, Failure, Expired or
    /// Terminated.
    pub fn is_failed(self) -> bool {
        Self::FAILED_STATUSES.contains(&self)
    }

    /// Whether the order's payment was refunded: Refunded or Partially refunded.
    pub fn is_refunded(self) -> bool {
        Self::REFUNDED_STATUSES.contains(&self)
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for OrderStatus {
    type Err = TapsilatError;

    /// Parses a numeric code such as "3" or a status name such as "Paid", case-insensitively.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        let status = if !trimmed.is_empty() && trimmed.bytes().all(|b| b.is_ascii_digit()) {
            trimmed.parse::<i64>().ok().and_then(Self::from_code)
        } else {
            Self::from_name(trimmed)
        };
        status.ok_or_else(|| {
            TapsilatError::ValidationError(format!("Unknown order status: {}", value))
        })
    }
}

impl Serialize for OrderStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(self.code())
    }
}

impl<'de> Deserialize<'de> for OrderStatus {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        Self::from_json(&value)
            .ok_or_else(|| serde::de::Error::custom(format!("{} is not an order status", value)))
    }
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

/// The order-create request. Required: `locale`, a non-zero `amount` and `currency`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateOrderRequest {
    pub amount: f64,
    pub currency: String,
    pub locale: String,
    pub buyer: CreateBuyerRequest,
    #[serde(rename = "basket_items", skip_serializing_if = "Option::is_none")]
    pub basket_items: Option<Vec<BasketItemDTO>>,
    #[serde(rename = "billing_address", skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddressDTO>,
    #[serde(rename = "checkout_design", skip_serializing_if = "Option::is_none")]
    pub checkout_design: Option<CheckoutDesignDTO>,
    #[serde(rename = "conversation_id", skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(
        rename = "enabled_installments",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_installments: Option<Vec<i32>>,
    #[serde(
        rename = "external_reference_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<MetadataDTO>>,
    /// Saved cards to charge for the order.
    #[serde(rename = "order_cards", skip_serializing_if = "Option::is_none")]
    pub order_cards: Option<Vec<OrderCardDTO>>,
    /// The VPOS to process the order with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_vpos_id: Option<String>,
    #[serde(rename = "paid_amount", skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<f64>,
    #[serde(rename = "partial_payment", skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    #[serde(
        rename = "payment_failure_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_failure_url: Option<String>,
    #[serde(rename = "payment_methods", skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<bool>,
    /// "auth" (default) or "preauth".
    #[serde(rename = "payment_mode", skip_serializing_if = "Option::is_none")]
    pub payment_mode: Option<String>,
    #[serde(rename = "payment_options", skip_serializing_if = "Option::is_none")]
    pub payment_options: Option<Vec<String>>,
    #[serde(
        rename = "payment_success_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_success_url: Option<String>,
    #[serde(rename = "payment_terms", skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<Vec<PaymentTermDTO>>,
    #[serde(rename = "pf_sub_merchant", skip_serializing_if = "Option::is_none")]
    pub pf_sub_merchant: Option<OrderPFSubMerchantDTO>,
    #[serde(
        rename = "redirect_failure_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub redirect_failure_url: Option<String>,
    #[serde(
        rename = "redirect_success_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub redirect_success_url: Option<String>,
    #[serde(rename = "shipping_address", skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddressDTO>,
    #[serde(rename = "sub_organization", skip_serializing_if = "Option::is_none")]
    pub sub_organization: Option<SubOrganizationDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submerchants: Option<Vec<SubmerchantDTO>>,
    #[serde(rename = "tax_amount", skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<f64>,
    #[serde(rename = "three_d_force", skip_serializing_if = "Option::is_none")]
    pub three_d_force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consents: Option<Vec<OrderConsent>>,
}

/// A consent text the buyer must accept on the checkout page.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderConsent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderItemRequest {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateOrderResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
}

/// Refunds an order. `order_item_id` and `order_item_payment_id` optionally limit the
/// refund to one basket item or item payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundOrderRequest {
    pub amount: f64,
    #[serde(rename = "reference_id")]
    pub reference_id: String,
    #[serde(rename = "order_item_id", skip_serializing_if = "Option::is_none")]
    pub order_item_id: Option<String>,
    #[serde(
        rename = "order_item_payment_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_item_payment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundOrderResponse {
    pub order: Order,
    pub refund_amount: f64,
    pub refund_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetadataDTO {
    pub key: String,
    pub value: String,
}

/// A basket item. The response-only fields are filled when reading an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BasketItemDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    /// The item's merchant category code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<BasketItemPayerDTO>,
    /// The earliest time the item's payout may be released (RFC3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_release_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_float: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant_price: Option<String>,
    /// Response only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u64>,
    /// Response only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<f64>,
    /// Response only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable_amount: Option<f64>,
    /// Response only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paidable_amount: Option<f64>,
    /// Response only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_payments: Option<Vec<OrderItemPayment>>,
}

/// A payment made against a basket item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderItemPayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_bin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BasketItemPayerDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_office: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillingAddressDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbourhood: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_office: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

/// Customizes the checkout page branding for one order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutDesignDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Deprecated: checkout no longer renders custom order detail HTML; the value is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_detail_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_button_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_button_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_muted_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_muted_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode_enabled: Option<bool>,
    /// Colors used in dark mode, keyed by the light-mode field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_colors: Option<HashMap<String, String>>,
    /// Deprecated: checkout design no longer carries a redirect URL; the value is ignored.
    /// Use the order's `redirect_success_url` and `redirect_failure_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
}

/// Selects a saved card for the order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderCardDTO {
    pub card_id: String,
    pub card_sequence: i32,
}

/// A payment term sent with the order. `due_date` is required and, like `paid_date`,
/// must be an RFC3339 timestamp such as "2026-01-15T00:00:00Z".
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentTermDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_sequence: Option<i32>,
}

/// A payment term as returned in the order detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderPaymentTermDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<OrderItemPayment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// A payment attempt recorded on an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderPaymentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// A refund or cancellation request recorded on an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderRefundCancelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderPFSubMerchantDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_iso_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submerchant_nin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submerchant_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_no: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingAddressDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubOrganizationDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquirer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gsm_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_company_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_office: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubmerchantDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_basket_item_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderAccountingRequest {
    #[serde(rename = "order_reference_id")]
    pub order_reference_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderPostAuthRequest {
    pub amount: f64,
    #[serde(rename = "reference_id")]
    pub reference_id: String,
}

/// Adds a payment term to an existing order. `due_date` and `paid_date` use the
/// YYYY-MM-DD format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderPaymentTermCreateDTO {
    /// The order's ID (`CreateOrderResponse::order_id`), not its reference ID.
    pub order_id: String,
    pub term_reference_id: String,
    pub amount: f64,
    pub due_date: String,
    pub term_sequence: i32,
    pub required: bool,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
}

/// Replaces a payment term. The server stores every field as sent, so populate all of
/// them, not only the ones that change. Dates use the YYYY-MM-DD format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderPaymentTermUpdateDTO {
    pub term_reference_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_sequence: Option<i32>,
}

/// Refunds a payment made against a payment term.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderTermRefundRequest {
    /// The payment term's ID (`OrderPaymentTermDetail::id`).
    pub term_id: String,
    pub amount: f64,
    /// The order's reference ID; the server requires it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_payment_id: Option<String>,
}

/// Selects orders for [`crate::TapsilatClient::get_order_list_with_filter`]. The server
/// defaults `page` to 1 and `per_page` to 10 (at most 100) when they are zero.
#[derive(Debug, Clone, Default)]
pub struct OrderListFilter {
    pub page: u32,
    pub per_page: u32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub organization_id: Option<String>,
    pub conversation_id: Option<String>,
    pub related_reference_id: Option<String>,
    pub buyer_id: Option<String>,
    /// Numeric order status.
    pub status: Option<i32>,
}

/// Selects the order whose payments to list; set at least one identifier.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetOrderPaymentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

/// A payment made against an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderPayment {
    pub id: String,
    pub date: String,
    pub payment_mode: String,
    pub amount: f64,
    pub card_id: String,
    pub masked_card: String,
    pub card_holder_name: String,
    pub card_type: String,
    pub card_brand: String,
    pub card_bank: String,
    pub card_network: String,
    pub paid: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub acquirer_response: String,
    pub vpos_commission_amount: f64,
    pub vpos_commission_rate: f64,
    pub vpos_commission_fee_type: String,
    pub organization_commission_amount: f64,
    pub organization_commission_rate: f64,
    pub organization_commission_fee_type: String,
    pub total_amount: f64,
    pub acquirer_commission_amount: f64,
    pub acquirer_commission_rate: f64,
    pub acquirer_commission_fee_type: String,
    pub acquirer_name: String,
    pub customer_paid_commission_amount: f64,
    pub merchant_paid_commission_amount: f64,
    pub merchant_net_amount: f64,
}

/// The payments made against an order, with totals.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GetOrderPaymentsResponse {
    pub payments: Vec<OrderPayment>,
    pub total_amount: f64,
    pub total_customer_paid_commission_amount: f64,
    pub total_merchant_paid_commission_amount: f64,
    pub total_merchant_net_amount: f64,
    pub order_amount: f64,
}

/// A submerchant row returned by the order submerchant list.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderSubmerchantListItem {
    pub id: String,
    pub name: String,
    pub email: String,
    pub submerchant_type: String,
    pub submerchant_key: String,
    pub labels: String,
    pub status: String,
    pub acquirer: String,
}

/// A page of order submerchants; the rows come in the `row` key.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrderSubmerchantListResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    #[serde(rename = "row")]
    pub rows: Vec<OrderSubmerchantListItem>,
}

/// Charges an order, for example with its saved card.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderChargeRequest {
    pub order_reference_id: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub retry: bool,
}

/// The charge result. The server sends these keys in camelCase, unlike most responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct OrderChargeResponse {
    pub is_success: bool,
    pub message: String,
    pub status_code: String,
    pub error_code: String,
}

// Re-export Buyer from buyer.rs
use crate::types::buyer::Buyer;
pub use crate::types::buyer::CreateBuyerRequest;
