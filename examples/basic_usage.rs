use tapsilat::{TapsilatClient, CreateOrderRequest, CreateOrderItemRequest, Currency, Validators};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Tapsilat Rust SDK - Basic Usage Example");
    println!("==========================================\n");

    // 1. Client Creation
    println!("1. Creating Tapsilat client...");
    let client = TapsilatClient::from_api_key("test-api-key")?;
    println!("✅ Client created successfully\n");

    // 2. Validation Examples
    println!("2. Testing validators...");
    
    // GSM validation
    match Validators::validate_gsm("5551234567") {
        Ok(normalized) => println!("✅ GSM validation passed: {}", normalized),
        Err(e) => println!("❌ GSM validation failed: {}", e),
    }
    
    // Email validation
    match Validators::validate_email("test@tapsilat.com") {
        Ok(_) => println!("✅ Email validation passed"),
        Err(e) => println!("❌ Email validation failed: {}", e),
    }
    
    // Amount validation
    match Validators::validate_amount(99.99) {
        Ok(_) => println!("✅ Amount validation passed"),
        Err(e) => println!("❌ Amount validation failed: {}", e),
    }
    
    println!();

    // 3. Order Creation Example (won't actually send due to test API key)
    println!("3. Creating order request...");
    let order_request = CreateOrderRequest {
        amount: 149.99,
        currency: Currency::TRY,
        description: Some("Test Order from Rust SDK".to_string()),
        items: vec![
            CreateOrderItemRequest {
                name: "Premium Package".to_string(),
                price: 149.99,
                quantity: 1,
                description: Some("Monthly subscription".to_string()),
            }
        ],
        buyer: None,
        callback_url: Some("https://your-site.com/webhook".to_string()),
        metadata: None,
    };
    
    println!("✅ Order request prepared:");
    println!("   Amount: {} {:?}", order_request.amount, order_request.currency);
    println!("   Items: {} item(s)", order_request.items.len());
    
    // Note: Actual API call would be:
    // let order_response = client.orders().create(order_request)?;
    println!("   (API call skipped - using test key)\n");

    println!("🎉 Basic usage example completed successfully!");
    println!("Next steps:");
    println!("- Replace 'test-api-key' with your real Tapsilat API key");
    println!("- Check advanced_usage.rs for more complex examples");
    
    Ok(())
}
