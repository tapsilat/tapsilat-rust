use tapsilat::{
    TapsilatClient, Config, CreateOrderRequest, CreateOrderItemRequest, CreateBuyerRequest, CreateAddressRequest,
    Currency, WebhookModule, WebhookVerificationConfig, CreateInstallmentPlanRequest, RefundOrderRequest
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Tapsilat Rust SDK - Advanced Usage Example");
    println!("=============================================\n");

    // 1. Advanced Client Configuration
    println!("1. Advanced client configuration...");
    let config = Config::new("your-api-key-here")
        .with_base_url("https://api.tapsilat.com")
        .with_timeout(60);
    
    let client = TapsilatClient::new(config)?;
    println!("✅ Advanced client configured\n");

    // 2. Complete Order with Buyer Information
    println!("2. Creating complete order with buyer info...");
    let order_request = CreateOrderRequest {
        amount: 299.99,
        currency: Currency::TRY,
        description: Some("Premium E-commerce Order".to_string()),
        buyer: Some(CreateBuyerRequest {
            name: "Ahmet".to_string(),
            surname: "Yılmaz".to_string(),
            email: "ahmet@example.com".to_string(),
            gsm: "5551234567".to_string(),
            identity_number: Some("12345678901".to_string()),
            address: Some(CreateAddressRequest {
                country: "Turkey".to_string(),
                city: "İstanbul".to_string(),
                district: Some("Kadıköy".to_string()),
                address_line_1: "Atatürk Cad. No: 123".to_string(),
                address_line_2: Some("Daire 5".to_string()),
                postal_code: Some("34710".to_string()),
            }),
        }),
        items: vec![
            CreateOrderItemRequest {
                name: "Premium Laptop".to_string(),
                price: 249.99,
                quantity: 1,
                description: Some("Gaming laptop".to_string()),
            },
            CreateOrderItemRequest {
                name: "Wireless Mouse".to_string(),
                price: 50.00,
                quantity: 1,
                description: Some("Ergonomic mouse".to_string()),
            }
        ],
        callback_url: Some("https://mystore.com/webhook/tapsilat".to_string()),
        metadata: Some(std::collections::HashMap::from([
            ("order_source".to_string(), serde_json::Value::String("web".to_string())),
            ("campaign_id".to_string(), serde_json::Value::String("BLACK_FRIDAY_2023".to_string())),
        ])),
    };

    println!("✅ Complete order request prepared");
    println!("   Buyer: {} {}", order_request.buyer.as_ref().unwrap().name, order_request.buyer.as_ref().unwrap().surname);
    println!("   Items: {} items, Total: {} {:?}", order_request.items.len(), order_request.amount, order_request.currency);

    // Note: Real API calls would be:
    // let order_response = client.orders().create(order_request)?;
    // println!("   Order ID: {}", order_response.order.id);
    // println!("   Checkout URL: {}", order_response.checkout_url.unwrap_or_default());
    println!("   (API call skipped - demo mode)\n");

    // 3. Installment Plan Example
    println!("3. Creating installment plan...");
    let installment_request = CreateInstallmentPlanRequest {
        order_id: "demo_order_123".to_string(),
        installment_count: 6,
        first_installment_date: "2024-01-15".to_string(),
    };
    
    println!("✅ Installment plan prepared: {} installments", installment_request.installment_count);
    // let installment_plan = client.installments().create_plan(installment_request)?;
    println!("   (API call skipped - demo mode)\n");

    // 4. Webhook Verification Example
    println!("4. Webhook verification example...");
    let webhook_payload = r#"{
        "event_type": "order.completed",
        "data": {
            "order_id": "order_123",
            "amount": 299.99,
            "currency": "TRY",
            "status": "completed"
        },
        "timestamp": "2023-12-01T10:30:00Z"
    }"#;
    
    let webhook_signature = "sha256=abc123def456...";
    let webhook_config = WebhookVerificationConfig {
        secret: "your-webhook-secret".to_string(),
        tolerance_seconds: Some(300), // 5 minutes
    };

    match WebhookModule::verify_webhook(webhook_payload, webhook_signature, &webhook_config) {
        Ok(result) => {
            if result.is_valid {
                println!("✅ Webhook verification successful");
            } else {
                println!("❌ Webhook verification failed: {}", result.error.unwrap_or_default());
            }
        }
        Err(e) => println!("❌ Webhook verification error: {}", e),
    }
    
    // Parse webhook
    match WebhookModule::parse_webhook(webhook_payload) {
        Ok(webhook_event) => {
            println!("✅ Webhook parsed: {:?}", webhook_event.event_type);
            if let Some(order_id) = webhook_event.data.order_id {
                println!("   Order ID from webhook: {}", order_id);
            }
        }
        Err(e) => println!("❌ Webhook parsing error: {}", e),
    }
    
    println!();

    // 5. Order Operations Example  
    println!("5. Order operations examples...");
    let demo_order_id = "demo_order_123";
    
    // Get order
    println!("   - Get order: GET /orders/{}", demo_order_id);
    // let order = client.orders().get(demo_order_id)?;
    
    // Get order status
    println!("   - Get order status: GET /orders/{}/status", demo_order_id);
    // let status = client.orders().get_status(demo_order_id)?;
    
    // Refund order (partial)
    let refund_request = RefundOrderRequest {
        amount: Some(50.00), // Partial refund
        reason: Some("Customer requested refund for one item".to_string()),
    };
    println!("   - Partial refund: POST /orders/{}/refund", demo_order_id);
    // let refund_response = client.orders().refund(demo_order_id, refund_request)?;
    
    println!("   (All API calls skipped - demo mode)\n");

    println!("🎉 Advanced usage example completed!");
    println!("\nKey Features Demonstrated:");
    println!("✅ Advanced client configuration");
    println!("✅ Complete order creation with buyer info");
    println!("✅ Installment plan creation");
    println!("✅ Webhook verification and parsing");
    println!("✅ Order management operations");
    println!("\n💡 To run with real API:");
    println!("1. Replace API keys with real values");
    println!("2. Uncomment the actual API calls");
    println!("3. Handle responses appropriately");

    Ok(())
}
