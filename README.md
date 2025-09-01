# Tapsilat Rust SDK

<div align="center">


**Enterprise-grade Rust SDK for Tapsilat Payment Processing Platform**

</div>

## About Tapsilat

**Tapsilat** is Turkey's leading fintech platform providing comprehensive payment processing solutions for businesses of all sizes. Our cutting-edge technology enables secure, fast, and reliable payment transactions with support for multiple payment methods, currencies, and advanced fraud protection.

---

## Installation

```bash
npm install @tapsilat/tapsilat-js
```

## Quick Start

### Initialize the SDK

```rust
use tapsilat::{Config, TapsilatClient};

let client = TapsilatClient::from_api_key("your-bearer-token")?;

// Or with advanced configuration
let config = Config::new("your-bearer-token")
    .with_base_url("https://acquiring.tapsilat.dev/api/v1")
    .with_timeout(30);
let client = TapsilatClient::new(config)?;
```

### Create an Order

```rust
use tapsilat::{CreateOrderRequest, CreateBuyerRequest, Currency};

let order_request = CreateOrderRequest {
    amount: 150.75,
    currency: Currency::TRY,
    locale: Some("tr".to_string()),
    conversation_id: Some("order-12345".to_string()),
    description: Some("Premium subscription - Monthly plan".to_string()),
    buyer: Some(CreateBuyerRequest {
        name: "John".to_string(),
        surname: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        phone: Some("5551234567".to_string()),
        identity_number: None,
        shipping_address: None,
        billing_address: None,
    }),
    items: vec![],
    callback_url: None,
    metadata: None,
};

let order = client.create_order(order_request)?;
println!("Order ID: {}", order.order_id);
```

### Check Order Status

```rust
let status = client.get_order_status(&order.order_id)?;
println!("Order status: {}", status);
```

---

## Features

### Core Payment Operations
- Secure authentication with bearer tokens
- Complete payment lifecycle management
- Multi-currency support (TRY, USD, EUR, GBP)
- Advanced filtering and pagination

### Payment Term Management
- Create and manage installment plans
- Update payment terms (amount, dates, status)
- Delete payment terms
- Term-specific refunds and termination
- Complete order termination

### Validation & Utilities
- Turkish GSM number validation and formatting
- Installment validation with flexible input formats
- Input validation for all request parameters
- Webhook signature verification

### Technical Features
- Full Rust type safety
- Zero-copy deserialization with serde
- Async/await support
- Configuration management
- Request/response validation
- Comprehensive error handling

---

## SDK Compatibility

This Rust SDK provides full feature parity with Tapsilat's JavaScript and Python SDKs:

| Feature Category | Rust | JavaScript | Python | 
|-----------------|------|------------|--------|
| Order Management | Yes | Yes | Yes |
| Payment Terms | Yes | Yes | Yes |
| GSM Validation | Yes | Yes | Yes |
| Installment Validation | Yes | Yes | Yes |
| Webhook Verification | Yes | Yes | No |
| Type Safety | Yes | Yes | No |
| Memory Safety | Yes | No | No |

---

## API Methods & Examples

### Order Management

#### Create Order
```rust
use tapsilat::CreateOrderItemRequest;

let order_request = CreateOrderRequest {
    amount: 299.99,
    currency: Currency::TRY,
    locale: Some("tr".to_string()),
    conversation_id: Some("order-12345".to_string()),
    description: Some("Product purchase".to_string()),
    buyer: Some(CreateBuyerRequest {
        name: "John".to_string(),
        surname: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        phone: Some("5551234567".to_string()),
        identity_number: None,
        shipping_address: None,
        billing_address: None,
    }),
    items: vec![CreateOrderItemRequest {
        name: "Premium Product".to_string(),
        price: 299.99,
        quantity: 1,
        description: Some("High quality product".to_string()),
    }],
    callback_url: Some("https://mystore.com/success".to_string()),
    metadata: None,
};

let order = client.create_order(order_request)?;
```

#### Get Order Details
```rust
let order = client.get_order("order-id")?;
println!("Order amount: {}", order.order.amount);
println!("Order status: {:?}", order.order.status);
```

#### Check Order Status
```rust
let status = client.get_order_status("order-id")?;
println!("Payment status: {}", status);
```

#### List Orders
```rust
let orders = client.get_order_list(Some(1), Some(10))?;
println!("Found {} orders", orders.total);
```

#### Cancel Order
```rust
match client.cancel_order("order-id") {
    Ok(_) => println!("Order cancelled successfully"),
    Err(e) => eprintln!("Cannot cancel order: {}", e),
}
```

### Payment Operations

#### Get Payment Details
```rust
let payment_details = client.get_order_payment_details("order-id")?;
println!("Payment details: {:?}", payment_details);
```

#### Get Transaction History
```rust
let transactions = client.get_order_transactions("order-id")?;
println!("Transaction history: {:?}", transactions);
```

#### Get Checkout URL
```rust
let checkout_url = client.get_checkout_url("order-id")?;
println!("Redirect customer to: {:?}", checkout_url);
```

### Refund Operations

#### Process Partial Refund
```rust
let refund_result = client.refund_order("order-id", Some(50.0))?;
println!("Refund processed: {:?}", refund_result);
```

#### Process Full Refund
```rust
let full_refund = client.refund_all_order("order-id")?;
println!("Full refund processed: {:?}", full_refund);
```

### Webhook Handling

#### Verify Webhook Signature
```rust
use tapsilat::WebhookModule;

// In your webhook handler
fn handle_webhook(payload: &str, signature: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    let is_valid = WebhookModule::verify_webhook(payload, signature, secret)?;
    
    if !is_valid {
        return Err("Invalid webhook signature".into());
    }
    
    let webhook_event = WebhookModule::parse_webhook(payload)?;
    
    match webhook_event.event_type {
        tapsilat::WebhookEventType::OrderCompleted => {
            println!("Order completed: {:?}", webhook_event.data.order_id);
            // Process successful payment
        }
        tapsilat::WebhookEventType::OrderFailed => {
            println!("Order failed: {:?}", webhook_event.data.order_id);
            // Handle failed payment
        }
        _ => {}
    }
    
    Ok(())
}
```

### Payment Term Management

#### Create Installment Plan
```rust
use tapsilat::CreateInstallmentPlanRequest;

let installment_request = CreateInstallmentPlanRequest {
    order_id: "order-id".to_string(),
    installment_count: 6,
    first_installment_date: "2024-12-31".to_string(),
};

let plan = client.installments().create_plan(installment_request)?;
println!("Installment plan created: {:?}", plan);
```

### Validation Utilities

#### GSM Number Validation
```rust
use tapsilat::Validators;

// Validate and normalize Turkish GSM numbers
let gsm_result = Validators::validate_gsm("0555 123 45 67")?;
println!("Normalized GSM: {}", gsm_result); // "905551234567"

// Supports multiple formats
let formats = vec![
    "0555 123 45 67",  // National format
    "555 123 45 67",   // Local format  
    "5551234567",      // No formatting
    "+905551234567",   // International format
];

for gsm in formats {
    match Validators::validate_gsm(gsm) {
        Ok(normalized) => println!("{} -> {}", gsm, normalized),
        Err(e) => eprintln!("Invalid GSM {}: {}", gsm, e),
    }
}
```

#### Installments Validation
```rust
// Validate installment counts
let valid_installments = vec![1, 3, 6, 12];
for count in valid_installments {
    match Validators::validate_installments(count) {
        Ok(_) => println!("{} installments: Valid", count),
        Err(e) => eprintln!("{} installments: {}", count, e),
    }
}
```

#### Email and Amount Validation
```rust
// Email validation
Validators::validate_email("user@example.com")?;

// Amount validation (currency precision)
Validators::validate_amount(99.99)?;

// Turkish identity number validation
Validators::validate_identity_number("12345678901")?;
```

---

## Advanced Configuration

The SDK can be customized with various configuration options:

```rust
use tapsilat::Config;

let config = Config::new("your-bearer-token")
    .with_base_url("https://acquiring.tapsilat.dev/api/v1")
    .with_timeout(30); // 30 seconds

let client = TapsilatClient::new(config)?;
```

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
