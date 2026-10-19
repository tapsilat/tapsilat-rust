# Tapsilat Rust SDK


**Enterprise-grade Rust SDK for Tapsilat Payment Processing Platform**

## About Tapsilat

**Tapsilat** is Turkey's leading fintech platform providing comprehensive payment processing solutions for businesses of all sizes. Our cutting-edge technology enables secure, fast, and reliable payment transactions with support for multiple payment methods, currencies, and advanced fraud protection.

---


## Quick Start

### Initialize the SDK

```rust
use tapsilat::{Config, TapsilatClient};

let client = TapsilatClient::from_api_key("your-bearer-token")?;

// Or with advanced configuration
let config = Config::new("your-bearer-token")
    .with_base_url("https://panel.tapsilat.dev/api/v1")
    .with_timeout(30); // seconds, applied to the whole request
let client = TapsilatClient::new(config)?;
```

The base URL points at API v1. Marketplace methods call API v2, derived from it by
replacing the trailing `/api/v1` with `/api/v2`.

### Create an Order

Request structs implement `Default`, so set the fields you need and fill the rest with
`..Default::default()`. Unset optional fields are left out of the request.

```rust
use tapsilat::{CreateBuyerRequest, CreateOrderRequest};

let order = client.create_order(CreateOrderRequest {
    amount: 150.75,
    currency: "TRY".to_string(),
    locale: "tr".to_string(),
    conversation_id: Some("order-12345".to_string()),
    buyer: CreateBuyerRequest {
        name: "John".to_string(),
        surname: "Doe".to_string(),
        email: Some("john.doe@example.com".to_string()),
        gsm_number: Some("+90 555 123 45 67".to_string()), // cleaned to +905551234567
        birth_date: Some("1990-01-02".to_string()),
        ..Default::default()
    },
    redirect_success_url: Some("https://shop.example/success".to_string()),
    redirect_failure_url: Some("https://shop.example/failure".to_string()),
    ..Default::default()
})?;

println!("Order ID: {:?}", order.order_id);
println!("Reference ID: {:?}", order.reference_id);
println!("Checkout URL: {:?}", order.checkout_url); // filled from the created order
```

Payment terms sent with an order need a `due_date`, and their dates are RFC3339
timestamps such as `"2026-01-15T00:00:00Z"`.

### Check Order Status

```rust
let status = client.get_order_status("reference-id")?;
println!("Order status: {}", status["status"]);
```

---

## Features

- Orders: create, read, list with filters, cancel, refund, charge, payments and transactions
- Payment terms: read, add, replace, delete and refund terms; terminate orders
- Card installments and the installment table
- Organization settings, currencies, users and sub-organizations
- Submerchants, settlement approval and payout events
- Virtual POS and VPOS submerchants
- Saved cards (tokenization)
- Subscriptions
- Marketplace sellers, payouts, payout operations and policies (API v2)
- HMAC-SHA256 webhook verification
- GSM, installment, email, amount and identity number validation

---

## API Methods & Examples

### Order Management

#### Get Order Details
```rust
let order = client.get_order("reference-id")?;
println!("Order amount: {:?}", order.amount);
println!("Order status: {:?} {:?}", order.status, order.status_enum);

let same_order = client.get_order_by_conversation_id("order-12345")?;
```

#### List Orders
```rust
use tapsilat::OrderListFilter;

let page = client.get_order_list(1, 10, None)?;
println!("Found {} orders", page["total"]);

let paid = client.get_order_list_with_filter(&OrderListFilter {
    page: 1,
    per_page: 50,
    start_date: Some("2026-01-01".to_string()),
    status: Some(3),
    ..Default::default()
})?;
```

#### Cancel Order
```rust
match client.cancel_order("reference-id") {
    Ok(_) => println!("Order cancelled successfully"),
    Err(e) => eprintln!("Cannot cancel order: {}", e),
}
```

#### Payments, Transactions and Checkout URL
```rust
use tapsilat::GetOrderPaymentsRequest;

let details = client.get_order_payment_details("reference-id", None)?;
let by_conversation = client.get_order_payment_details_by_conversation_id("order-12345")?;
let transactions = client.get_order_transactions("reference-id")?; // a JSON array, or null
let payments = client.get_order_payments(GetOrderPaymentsRequest {
    order_reference_id: Some("reference-id".to_string()),
    ..Default::default()
})?;
println!("Merchant net amount: {}", payments.total_merchant_net_amount);
let checkout_url = client.get_checkout_url("reference-id")?;
```

#### Charge an Order
```rust
use tapsilat::OrderChargeRequest;

let result = client.charge_order(OrderChargeRequest {
    order_reference_id: "reference-id".to_string(),
    retry: false,
})?;
println!("Charged: {} ({})", result.is_success, result.message);
```

### Refund Operations

```rust
use tapsilat::RefundOrderRequest;

// Partial refund, optionally limited to one basket item or item payment
let refund = client.refund_order(RefundOrderRequest {
    amount: 50.0,
    reference_id: "reference-id".to_string(),
    ..Default::default()
})?;

// Full refund
let full_refund = client.refund_all_order("reference-id")?;
```

### Payment Terms

Term dates use the `YYYY-MM-DD` format. `order_id` is the order's ID (`order_id` from
`create_order`), not its reference ID.

```rust
use tapsilat::{OrderPaymentTermCreateDTO, OrderPaymentTermUpdateDTO, OrderTermRefundRequest};

client.create_order_term(OrderPaymentTermCreateDTO {
    order_id: "order-id".to_string(),
    term_reference_id: "term-1".to_string(),
    amount: 50.0,
    due_date: "2026-02-01".to_string(),
    term_sequence: 1,
    required: true,
    status: "PENDING".to_string(),
    ..Default::default()
})?;

let term = client.get_order_term("term-1")?;

// An update replaces the term, so send every field
client.update_order_term(OrderPaymentTermUpdateDTO {
    term_reference_id: "term-1".to_string(),
    amount: Some(60.0),
    due_date: Some("2026-02-15".to_string()),
    required: Some(true),
    status: Some("PENDING".to_string()),
    term_sequence: Some(1),
    ..Default::default()
})?;

client.refund_order_term(OrderTermRefundRequest {
    term_id: "term-id".to_string(),
    amount: 10.0,
    reference_id: Some("reference-id".to_string()),
    term_payment_id: None,
})?;
client.delete_order_term("order-id", "term-1")?;
client.order_terminate("reference-id")?;
```

### Card Installments

```rust
use tapsilat::{CardInstallmentsRequest, InstallmentTableRequest};

let options = client.get_card_installments(&CardInstallmentsRequest {
    bin: "54066200".to_string(),
    amount: 1000.0,
    currency: "TRY".to_string(),
    is_three_d: true,
})?;
for option in options.installments {
    println!("{} x {} (total {})", option.installment, option.monthly_payment, option.total_amount);
}

let table = client.get_installment_table(&InstallmentTableRequest {
    amount: 1000.0,
    currency: "TRY".to_string(),
    is_three_d: false,
})?;
```

### Organization

```rust
let settings = client.organization().settings()?;
let currencies = client.organization().currencies()?;
let presets = client.list_organization_currency_presets()?;
client.create_organization_currency("EUR")?;

// Most endpoints accept currency units such as "TRY"; resolve one to its UUID when needed
let currency_id = client.resolve_currency_id("TRY")?;

let suborganizations = client.organization().list_suborganizations(1, 20)?;
let limits = client.get_organization_limits_by_id("organization-id")?;
```

### Submerchants

```rust
use tapsilat::{SubmerchantCreateRequest, SubmerchantPaymentAction};

let submerchants = client.submerchants();
submerchants.create(SubmerchantCreateRequest {
    locale: "tr".to_string(),
    conversation_id: "conv-1".to_string(),
    name: "Seller".to_string(),
    email: "seller@example.com".to_string(),
    gsm_number: "+905551234567".to_string(),
    sub_merchant_type: "PERSONAL".to_string(),
    identity_number: "11111111111".to_string(),
    currency_id: "TRY".to_string(),
    ..Default::default()
})?;

let page = submerchants.list(1, 20)?;
let submerchant = submerchants.get(&page.rows[0].id)?;

submerchants.approve_payment(SubmerchantPaymentAction {
    locale: "tr".to_string(),
    conversation_id: "conv-1".to_string(),
    payment_transaction_id: "transaction-id".to_string(),
    acquirer: "acquirer".to_string(),
})?;
```

### Virtual POS

A VPOS update replaces the whole configuration, so read it first and send every field
back.

```rust
use tapsilat::VposUpdateRequest;

let vpos = client.vpos();
let current = vpos.get("vpos-id")?;
vpos.update("vpos-id", VposUpdateRequest {
    name: current.name,
    bank_name: current.bank_name,
    failover: true,
    main_installment: true,
    ..Default::default()
})?;
let acquirers = vpos.list_acquirers()?;
```

### Saved Cards

```rust
let cards = client.tokenization().list_saved_cards(1, 10)?;
client.tokenization().delete_saved_card(&cards.rows[0].id)?;
```

### Marketplace (API v2)

Seller and payout revisions are opaque strings. Send them back exactly as received.

```rust
use tapsilat::{MarketplaceApprovalPolicyInput, MarketplacePayoutApproval, MarketplacePayoutFilter};

let marketplace = client.marketplace();
let sellers = marketplace.list_submerchants(1, 20)?;
let payouts = marketplace.list_payouts(&MarketplacePayoutFilter {
    status: Some("WAITING_APPROVAL".to_string()),
    page: 1,
    per_page: 20,
    ..Default::default()
})?;
marketplace.approve_payouts(MarketplacePayoutApproval {
    ids: payouts.rows.iter().map(|p| p.id.clone()).collect(),
})?;

// The approval policy currency must be a UUID; a unit such as "TRY" is resolved for you
marketplace.create_approval_policy(MarketplaceApprovalPolicyInput {
    name: "Large orders".to_string(),
    priority: 1,
    enabled: Some(true),
    min_amount: Some("1000".to_string()),
    currency_id: "TRY".to_string(),
    mode: "manual".to_string(),
    ..Default::default()
})?;
```

### Webhook Handling

Signatures are hex HMAC-SHA256 digests of the raw payload, with or without the
`sha256=` prefix, and are compared in constant time.

```rust
use tapsilat::WebhookModule;

fn handle_webhook(payload: &str, signature: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !WebhookModule::verify_webhook(payload, signature, secret)? {
        return Err("Invalid webhook signature".into());
    }
    let event = WebhookModule::parse_webhook(payload)?;
    println!("Received {:?}", event.event_type);
    Ok(())
}
```

### Error Handling

API errors carry the HTTP status, the body's `status`, `code` and message, any field
errors, and the raw body. A response that is not JSON, such as an HTML page from a wrong
base URL, is also reported as an `ApiError`.

```rust
use tapsilat::TapsilatError;

match client.cancel_order("reference-id") {
    Err(TapsilatError::ApiError { status_code, code, message, errors, .. }) => {
        eprintln!("{} [{:?}] {} {:?}", status_code, code, message, errors);
    }
    Err(other) => eprintln!("{}", other),
    Ok(response) => println!("{}", response),
}
```

### Validation Utilities

```rust
use tapsilat::Validators;

// Clean a phone number the way create_order does
let gsm = Validators::validate_gsm_number("+90 (555) 123-45-67")?; // "+905551234567"

// Require a Turkish mobile number and normalize it
let turkish = Validators::validate_gsm("0555 123 45 67")?; // "905551234567"

// Parse a comma-separated installment list
let installments = Validators::parse_installments("1,3,6")?; // [1, 3, 6]

Validators::validate_email("user@example.com")?;
Validators::validate_amount(99.99)?;
Validators::validate_identity_number("10000000146")?;
```

### Deprecated Methods

These methods call routes the API does not have and always fail. They are kept for
compatibility:

- `client.payments()` (`/payments`): use the order methods, such as `get_order_payments` and `charge_order`.
- Installment plans (`/installments`): use `get_card_installments` and the payment term methods.
- `terminate_order_term`: use `order_terminate` or `delete_order_term`.
- `get_organization_limit_user` and `set_organization_limit_user`.
- `get_organization_limits()`: use `get_organization_limits_by_id`.

---

## Authentication

Use Bearer Token authentication:

```rust
// Simple initialization
let client = TapsilatClient::from_api_key("your-bearer-token")?;

// Or via environment variable
use std::env;
let api_key = env::var("TAPSILAT_BEARER_TOKEN")?;
let client = TapsilatClient::from_api_key(api_key)?;
```

Get your API token from the [Tapsilat Dashboard](https://tapsilat.dev) → Settings → API Keys

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Resources

- **Website**: [https://tapsilat.dev](https://tapsilat.dev)
- **Issues**: [GitHub Issues](https://github.com/tapsilat/tapsilat-rust/issues)
- **Examples**: See [examples/advanced_usage.rs](examples/advanced_usage.rs) for a complete implementation

## Type System

All Rust types are organized in `src/types/` with comprehensive documentation including:
- Strong compile-time type safety
- Zero-cost abstractions
- Memory safety guarantees
- Comprehensive error handling with `Result` types

---

<div align="center">

[![Tapsilat](https://img.shields.io/badge/Tapsilat-Payment%20Solutions-blue?style=for-the-badge)](https://tapsilat.dev)

</div>
