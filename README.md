# Tapsilat Rust SDK

[![Crates.io](https://img.shields.io/crates/v/tapsilat)](https://crates.io/crates/tapsilat)
[![Documentation](https://docs.rs/tapsilat/badge.svg)](https://docs.rs/tapsilat)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/tapsilat/tapsilat-rust/workflows/CI/badge.svg)](https://github.com/tapsilat/tapsilat-rust/actions)

Enterprise-grade **Rust SDK** for the **Tapsilat Payment Processing Platform**. Built for high-performance financial applications requiring type safety, reliability, and comprehensive payment processing capabilities.

## Features

- **Complete Payment Processing** - Orders, refunds, installments, and payment terms
- **Type Safety** - Comprehensive Rust type system with validation
- **Multi-Currency Support** - TRY, USD, EUR, GBP with proper formatting
- **Turkish GSM Validation** - Built-in Turkish phone number validation
- **Webhook Security** - Cryptographic signature verification
- **High Performance** - Zero-copy deserialization with serde
- **Testing Ready** - Mock server support for integration testing
- **Comprehensive Validation** - Email, amounts, installments, identity numbers
- **Async Support** - Compatible with tokio and async-std
- **Rich Documentation** - Extensive examples and API documentation

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tapsilat = "2025.9.28"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] } # For async support
```

### Basic Usage

```rust
use tapsilat::{
    Config, TapsilatClient, CreateOrderRequest, CreateOrderItemRequest, 
    Currency, CreateBuyerRequest
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let config = Config::new("your-api-key")
        .with_base_url("https://acquiring.tapsilat.dev/api/v1")
        .with_timeout(30);
    
    let client = TapsilatClient::new(config)?;

    // Create a payment order
    let order_request = CreateOrderRequest {
        amount: 299.99,
        currency: Currency::TRY,
        locale: Some("tr".to_string()),
        conversation_id: Some("order-12345".to_string()),
        description: Some("Premium Subscription".to_string()),
        buyer: Some(CreateBuyerRequest {
            name: "Ahmet".to_string(),
            surname: "Yılmaz".to_string(),
            email: "ahmet@example.com".to_string(),
            phone: Some("5551234567".to_string()),
            identity_number: None,
            shipping_address: None,
            billing_address: None,
        }),
        items: vec![CreateOrderItemRequest {
            name: "Monthly Subscription".to_string(),
            price: 299.99,
            quantity: 1,
            description: Some("Premium plan subscription".to_string()),
        }],
        callback_url: Some("https://yoursite.com/webhook".to_string()),
        metadata: None,
    };

    // Create the order
    match client.create_order(order_request).await? {
        response => {
            println!("Order created successfully!");
            println!("Order ID: {}", response.order_id);
            println!("Reference: {}", response.reference_id);
        }
    }

    Ok(())
}
```

## API Documentation

### Client Initialization

```rust
use tapsilat::{Config, TapsilatClient};

// Basic configuration
let client = TapsilatClient::from_api_key("your-api-key")?;

// Advanced configuration
let config = Config::new("your-api-key")
    .with_base_url("https://acquiring.tapsilat.dev/api/v1")
    .with_timeout(60);
let client = TapsilatClient::new(config)?;
```

### Order Management

#### Create Order

```rust
let order_response = client.create_order(order_request)?;
println!("Order ID: {}", order_response.order_id);
```

#### Get Order Details

```rust
let order = client.get_order("order-id")?;
println!("Order status: {:?}", order.order.status);
```

#### List Orders

```rust
let orders = client.get_order_list(Some(1), Some(10))?;
println!("Found {} orders", orders.total);
```

#### Cancel Order

```rust
let result = client.cancel_order("order-id")?;
println!("Order cancelled successfully");
```

### Modular API Interface

For better organization, you can use the modular interface:

```rust
// Using module-based API
let order_response = client.orders().create(order_request)?;
let installment_plan = client.installments().create_plan(plan_request)?;
let payment_result = client.payments().process(payment_request)?;
```

### Webhook Handling

```rust
use tapsilat::{WebhookModule, WebhookVerificationConfig};

// Simple verification
let is_valid = WebhookModule::verify_webhook(
    payload,
    signature,
    "your-webhook-secret"
)?;

// Advanced verification with tolerance
let config = WebhookVerificationConfig {
    secret: "your-webhook-secret".to_string(),
    tolerance_seconds: Some(300), // 5 minutes
};

let result = WebhookModule::verify_webhook_advanced(
    payload,
    signature,
    &config
)?;

if result.is_valid {
    let webhook_event = WebhookModule::parse_webhook(payload)?;
    println!("Event: {:?}", webhook_event.event_type);
}
```

### Validation Utilities

The SDK includes comprehensive validation for Turkish business requirements:

```rust
use tapsilat::Validators;

// GSM number validation (Turkish mobile numbers)
let normalized_gsm = Validators::validate_gsm("5551234567")?;
// Returns: "905551234567"

// Email validation
Validators::validate_email("user@example.com")?;

// Amount validation (currency precision)
Validators::validate_amount(99.99)?;

// Installment validation (1-12 installments)
Validators::validate_installments(6)?;

// Turkish identity number validation
Validators::validate_identity_number("12345678901")?;
```

### Error Handling

The SDK provides comprehensive error types:

```rust
use tapsilat::error::{TapsilatError, Result};

match client.create_order(invalid_request) {
    Ok(response) => println!("Success: {}", response.order_id),
    Err(TapsilatError::ValidationError(msg)) => {
        eprintln!("Validation failed: {}", msg);
    }
    Err(TapsilatError::ApiError { status_code, message }) => {
        eprintln!("API error {}: {}", status_code, message);
    }
    Err(TapsilatError::ConfigError(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Advanced Usage

### Installment Plans

```rust
use tapsilat::CreateInstallmentPlanRequest;

let installment_request = CreateInstallmentPlanRequest {
    order_id: "order-123".to_string(),
    installment_count: 12,
    first_installment_date: "2024-02-15".to_string(),
};

let plan = client.installments().create_plan(installment_request)?;
println!("Installment plan created: {:?}", plan);
```

### Multi-Currency Support

```rust
use tapsilat::Currency;

let usd_order = CreateOrderRequest {
    amount: 29.99,
    currency: Currency::USD,
    // ... other fields
};

let eur_order = CreateOrderRequest {
    amount: 25.99,
    currency: Currency::EUR,
    // ... other fields
};
```

### Environment Configuration

```rust
use std::env;

let api_key = env::var("TAPSILAT_API_KEY")
    .expect("TAPSILAT_API_KEY must be set");

let base_url = env::var("TAPSILAT_BASE_URL")
    .unwrap_or_else(|_| "https://acquiring.tapsilat.dev/api/v1".to_string());

let config = Config::new(api_key)
    .with_base_url(&base_url)
    .with_timeout(30);
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run integration tests
cargo test --test integration_tests

# Run with real API (requires TAPSILAT_API_KEY)
TAPSILAT_API_KEY=your-key cargo test real_api_tests

# Run specific test with output
cargo test test_order_creation -- --nocapture
```

### Mock Server Testing

```rust
// The SDK includes comprehensive mock server support
cargo test --test mock_server_tests
```

## Examples

The repository includes comprehensive examples:

```bash
# Basic usage example
cargo run --example basic_usage

# Advanced SDK demonstration
cargo run --example advanced_usage

# Webhook handling example
cargo run --example webhook_handling
```

## Security Best Practices

1. **Environment Variables**: Store API keys in environment variables
2. **Webhook Verification**: Always verify webhook signatures
3. **HTTPS Only**: Use HTTPS URLs for all API calls
4. **Key Rotation**: Regularly rotate your API keys
5. **Logging**: Avoid logging sensitive payment information

```rust
// Good: Use environment variables
let api_key = env::var("TAPSILAT_API_KEY")?;

// Bad: Hardcode API keys
let api_key = "sk_live_..."; // Never do this!

// Good: Verify webhooks
let is_valid = WebhookModule::verify_webhook(payload, signature, secret)?;
if !is_valid {
    return Err("Invalid webhook signature");
}
```

## Multi-Language SDK Support

Tapsilat provides SDKs for multiple programming languages:

- **JavaScript/TypeScript**: [@tapsilat/tapsilat-js](https://github.com/tapsilat/tapsilat-js)
- **Python**: [tapsilat-py](https://github.com/tapsilat/tapsilat-py) 
- **Rust**: [tapsilat-rust](https://github.com/tapsilat/tapsilat-rust) (this repository)
- **.NET**: Coming soon

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/tapsilat/tapsilat-rust.git
cd tapsilat-rust

# Install dependencies and run tests
cargo build
cargo test

# Run examples
cargo run --example advanced_usage

# Check code formatting
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
```

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run formatting (`cargo fmt`)
7. Run linting (`cargo clippy`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to the branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

## Support & Documentation

- **Documentation**: [docs.rs/tapsilat](https://docs.rs/tapsilat)
- **API Reference**: [Tapsilat API Documentation](https://docs.tapsilat.com)
- **Issues**: [GitHub Issues](https://github.com/tapsilat/tapsilat-rust/issues)
- **Discord**: [Tapsilat Developers](https://discord.gg/tapsilat)
- **Email**: [sdk@tapsilat.com](mailto:sdk@tapsilat.com)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust by the Tapsilat team
- Powered by the Rust ecosystem
- Special thanks to all contributors

---

**Made with Rust** - Empowering businesses with secure, fast payment processing