# Tapsilat Rust SDK Example

This directory contains a comprehensive example demonstrating the Tapsilat Rust SDK usage.

## Available Example

### Advanced Usage (`advanced_usage.rs`)
Complete demonstration of the Tapsilat Rust SDK including:
- Client configuration and initialization
- Order creation with buyer information
- Direct API methods
- Installment plan creation
- Webhook verification and parsing
- All available SDK operations
- Both direct and module-based API usage

**Run with:**
```bash
cargo run --example advanced_usage
```

## Features Demonstrated

- ✅ **Direct API**: Direct method calls like `client.create_order()`
- ✅ **Module API**: Traditional approach like `client.orders().create()`
- ✅ **Validation**: GSM number, installment, and other data validation
- ✅ **Webhooks**: Signature verification and payload parsing
- ✅ **Complete Integration**: Real-world usage patterns

## API Compatibility

The SDK provides two API styles:

```rust
// Direct API
let order = client.create_order(request)?;
let checkout_url = client.get_checkout_url(&order_id)?;
client.verify_webhook(payload, signature, secret)?;

// Module API
let order = client.orders().create(request)?;
let checkout_url = client.payments().get_checkout_url(&order_id)?;
```
