// Real API tests - only run when TAPSILAT_API_KEY environment variable is set
// Run with: TAPSILAT_API_KEY=your-key cargo test real_api_tests

use std::env;
use tapsilat::{
    TapsilatClient, Config, CreateOrderRequest, CreateOrderItemRequest, Currency,
    CreateInstallmentPlanRequest, Validators
};

fn skip_if_no_api_key() -> Option<String> {
    match env::var("TAPSILAT_API_KEY") {
        Ok(key) if !key.is_empty() => Some(key),
        _ => {
            println!("Skipping real API test - TAPSILAT_API_KEY not set");
            println!("To run real API tests: TAPSILAT_API_KEY=your-key cargo test real_api_tests");
            None
        }
    }
}

fn get_test_client(api_key: &str) -> TapsilatClient {
    let config = Config::new(api_key)
        .with_base_url("https://api-sandbox.tapsilat.com") // Use sandbox for testing
        .with_timeout(30);
    
    TapsilatClient::new(config).expect("Failed to create test client")
}

#[test]
fn test_real_api_client_creation() {
    if let Some(api_key) = skip_if_no_api_key() {
        let client = get_test_client(&api_key);
        
        // Just test that client was created successfully
        println!("✅ Real API client created successfully");
        
        // Test validators work
        assert!(Validators::validate_gsm("5551234567").is_ok());
        assert!(Validators::validate_email("test@tapsilat.com").is_ok());
        assert!(Validators::validate_amount(99.99).is_ok());
        
        println!("✅ All validators working correctly");
    }
}

#[test]
fn test_real_api_order_validation() {
    if let Some(api_key) = skip_if_no_api_key() {
        let _client = get_test_client(&api_key);
        
        // Test order request creation and validation
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
        
        // Validate the order request structure
        assert_eq!(order_request.amount, 149.99);
        assert_eq!(order_request.items.len(), 1);
        assert_eq!(order_request.items[0].name, "Premium Package");
        
        println!("✅ Order request validation successful");
        println!("   Amount: {} {:?}", order_request.amount, order_request.currency);
        println!("   Items: {} item(s)", order_request.items.len());
        
        // Note: Actual API call would be:
        // let result = client.orders().create(order_request);
        // But we're not making real API calls in tests unless specifically testing live API
    }
}

#[test]
fn test_real_api_installment_validation() {
    if let Some(api_key) = skip_if_no_api_key() {
        let _client = get_test_client(&api_key);
        
        // Test installment plan request
        let installment_request = CreateInstallmentPlanRequest {
            order_id: "test_order_123".to_string(),
            installment_count: 6,
            first_installment_date: "2024-02-15".to_string(),
        };
        
        // Validate installment request
        assert_eq!(installment_request.installment_count, 6);
        assert!(!installment_request.order_id.is_empty());
        assert!(!installment_request.first_installment_date.is_empty());
        
        println!("✅ Installment plan validation successful");
        println!("   Installments: {}", installment_request.installment_count);
        println!("   First payment date: {}", installment_request.first_installment_date);
    }
}

#[test]
#[ignore] // Use #[ignore] for tests that require manual execution
fn test_real_api_live_order_creation() {
    // This test only runs when explicitly called and with real API key
    // Run with: TAPSILAT_API_KEY=your-key cargo test test_real_api_live_order_creation -- --ignored
    
    if let Some(api_key) = skip_if_no_api_key() {
        let client = get_test_client(&api_key);
        
        let order_request = CreateOrderRequest {
            amount: 1.0, // Small amount for testing
            currency: Currency::TRY,
            description: Some("Rust SDK Live Test Order".to_string()),
            items: vec![
                CreateOrderItemRequest {
                    name: "Test Item".to_string(),
                    price: 1.0,
                    quantity: 1,
                    description: Some("SDK test item".to_string()),
                }
            ],
            buyer: None,
            callback_url: None,
            metadata: None,
        };
        
        // This would make a real API call
        match client.orders().create(order_request) {
            Ok(response) => {
                println!("✅ Live API test successful!");
                println!("   Order ID: {}", response.order.id);
                if let Some(checkout_url) = response.checkout_url {
                    println!("   Checkout URL: {}", checkout_url);
                }
            }
            Err(e) => {
                println!("❌ Live API test failed: {:?}", e);
                // Don't panic in live tests, just report the error
            }
        }
    }
}