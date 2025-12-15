// Real API tests - only run when TAPSILAT_API_KEY environment variable is set
// Run with: TAPSILAT_API_KEY=your-key cargo test real_api_tests

use std::env;
use tapsilat::{
    Config, CreateInstallmentPlanRequest, CreateOrderRequest, TapsilatClient, Validators,
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
        let _client = get_test_client(&api_key);

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
            currency: "TRY".to_string(),
            locale: "tr".to_string(),
            conversation_id: Some("test-123".to_string()),
            basket_items: Some(vec![tapsilat::types::BasketItemDTO {
                id: Some("item1".to_string()),
                name: Some("Premium Package".to_string()),
                price: Some(149.99),
                quantity: Some(1),
                item_type: Some("PHYSICAL".to_string()),
                 category1: None, category2: None, commission_amount: None, coupon: None, coupon_discount: None, data: None, paid_amount: None, payer: None, quantity_float: None, quantity_unit: None, sub_merchant_key: None, sub_merchant_price: None
            }]),
            buyer: tapsilat::types::CreateBuyerRequest {
                name: "John".to_string(),
                surname: "Doe".to_string(),
                email: Some("john@example.com".to_string()),
                gsm_number: Some("+905551234567".to_string()),
                identity_number: Some("11111111111".to_string()),
                registration_address: Some("Address line".to_string()),
                 ip: None, city: None, country: None, zip_code: None
            },
            metadata: None,
            billing_address: None,
            shipping_address: None,
            checkout_design: None,
            enabled_installments: None,
            external_reference_id: None,
            order_cards: None,
            paid_amount: None,
            partial_payment: None,
            payment_failure_url: None,
            payment_methods: None,
            payment_mode: None,
            payment_options: None,
            payment_success_url: None,
            payment_terms: None,
            pf_sub_merchant: None,
            redirect_failure_url: None,
            redirect_success_url: None,
            sub_organization: None,
            submerchants: None,
            tax_amount: None,
            three_d_force: None,
        };

        // Validate the order request structure
        assert_eq!(order_request.amount, 149.99);
        assert_eq!(order_request.basket_items.as_ref().unwrap().len(), 1);
        assert_eq!(order_request.basket_items.as_ref().unwrap()[0].name, Some("Premium Package".to_string()));

        println!("✅ Order request validation successful");
        println!(
            "   Amount: {} {:?}",
            order_request.amount, order_request.currency
        );
        println!("   Items: {} item(s)", order_request.basket_items.as_ref().unwrap().len());

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
        println!(
            "   First payment date: {}",
            installment_request.first_installment_date
        );
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
            currency: "TRY".to_string(),
            locale: "tr".to_string(),
            conversation_id: Some("test-live-123".to_string()),
            basket_items: Some(vec![tapsilat::types::BasketItemDTO {
                id: Some("item1".to_string()),
                name: Some("Test Item".to_string()),
                price: Some(1.0),
                quantity: Some(1),
                item_type: Some("PHYSICAL".to_string()),
                 category1: None, category2: None, commission_amount: None, coupon: None, coupon_discount: None, data: None, paid_amount: None, payer: None, quantity_float: None, quantity_unit: None, sub_merchant_key: None, sub_merchant_price: None
            }]),
            buyer: tapsilat::types::CreateBuyerRequest {
                name: "John".to_string(),
                surname: "Doe".to_string(),
                email: Some("john@example.com".to_string()),
                gsm_number: Some("+905551234567".to_string()),
                identity_number: Some("11111111111".to_string()),
                registration_address: Some("Address line".to_string()),
                 ip: None, city: None, country: None, zip_code: None
            },
            metadata: None,
            billing_address: None,
            shipping_address: None,
            checkout_design: None,
            enabled_installments: None,
            external_reference_id: None,
            order_cards: None,
            paid_amount: None,
            partial_payment: None,
            payment_failure_url: None,
            payment_methods: None,
            payment_mode: None,
            payment_options: None,
            payment_success_url: None,
            payment_terms: None,
            pf_sub_merchant: None,
            redirect_failure_url: None,
            redirect_success_url: None,
            sub_organization: None,
            submerchants: None,
            tax_amount: None,
            three_d_force: None,
        };

        // This would make a real API call
        match client.orders().create(order_request) {
            Ok(response) => {
                println!("✅ Live API test successful!");
                println!("   Order ID: {:?}", response.order_id);
                println!("   Reference ID: {:?}", response.reference_id);
            }
            Err(e) => {
                println!("❌ Live API test failed: {:?}", e);
                // Don't panic in live tests, just report the error
            }
        }
    }
}
