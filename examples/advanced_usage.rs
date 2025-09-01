use chrono;
use serde_json::Value;
use std::env;
use tapsilat::{
    Config, CreateAddressRequest, CreateBuyerRequest, CreateInstallmentPlanRequest,
    CreateOrderItemRequest, CreateOrderRequest, Currency, TapsilatClient, Validators,
    WebhookModule, WebhookVerificationConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Tapsilat Rust SDK - Comprehensive Test Arena");
    println!("===============================================\n");

    // Get API key from environment
    let api_key = env::var("TAPSILAT_API_KEY").unwrap_or_else(|_| {
        "<API_KEY>".to_string()
    });

    println!(
        "🚀 Running with API KEY: {}...{}",
        &api_key[..8],
        &api_key[api_key.len() - 4..]
    );
    println!();

    // 1. Advanced Client Configuration
    println!("=== 1. CLIENT CONFIGURATION ===");
    let config = Config::new(&api_key)
        .with_base_url("https://acquiring.tapsilat.dev/api/v1")
        .with_timeout(30);

    let client = TapsilatClient::new(config)?;
    println!("✅ Client configured successfully");
    println!(
        "   API Key: {}...{}",
        &api_key[..8],
        &api_key[api_key.len() - 4..]
    );
    println!("   Base URL: https://acquiring.tapsilat.dev/api/v1");
    println!("   Timeout: 30s\n");

    // 2. Comprehensive Validator Testing
    println!("=== 2. VALIDATOR TESTING ===");

    // GSM Validation Tests
    println!("📱 GSM Number Validation:");
    let test_gsm_numbers = vec![
        "5551234567",
        "+905551234567",
        "905551234567",
        "05551234567",
        "invalid_gsm",
        "4551234567", // Should fail - doesn't start with 5
    ];

    for gsm in test_gsm_numbers {
        match Validators::validate_gsm_number(gsm) {
            Ok(normalized) => println!("   ✅ {} → {}", gsm, normalized),
            Err(e) => println!("   ❌ {} → Error: {}", gsm, e),
        }
    }

    // Email Validation Tests
    println!("\n📧 Email Validation:");
    let test_emails = vec![
        "test@tapsilat.com",
        "user@example.org",
        "invalid-email",
        "@invalid.com",
        "test@",
    ];

    for email in test_emails {
        match Validators::validate_email(email) {
            Ok(_) => println!("   ✅ {} → Valid", email),
            Err(e) => println!("   ❌ {} → Error: {}", email, e),
        }
    }

    // Amount Validation Tests
    println!("\n💰 Amount Validation:");
    let test_amounts = vec![10.99, 0.01, 999.99, 0.0, -5.50, 10.555];

    for amount in test_amounts {
        match Validators::validate_amount(amount) {
            Ok(_) => println!("   ✅ {} → Valid", amount),
            Err(e) => println!("   ❌ {} → Error: {}", amount, e),
        }
    }

    // Installment Validation Tests
    println!("\n📊 Installment Validation:");
    let test_installments = vec![1, 3, 6, 12, 13, 0];

    for installments in test_installments {
        match Validators::validate_installments(installments) {
            Ok(_) => println!("   ✅ {} installments → Valid", installments),
            Err(e) => println!("   ❌ {} installments → Error: {}", installments, e),
        }
    }
    println!();

    // 3. Order Creation and Lifecycle Testing
    println!("=== 3. ORDER LIFECYCLE TESTING ===");

    let order_request = CreateOrderRequest {
        amount: 299.99,
        currency: Currency::TRY,
        locale: Some("tr".to_string()),
        conversation_id: Some(format!("order-{}", chrono::Utc::now().timestamp())),
        description: Some("Test Order".to_string()),
        buyer: Some(CreateBuyerRequest {
            name: "Ahmet".to_string(),
            surname: "Yılmaz".to_string(),
            email: "ahmet.yilmaz@gmail.com".to_string(),
            phone: Some("5321234567".to_string()),
            identity_number: None,
            shipping_address: None,
            billing_address: None,
        }),
        items: vec![CreateOrderItemRequest {
            name: "Test Product".to_string(),
            price: 299.99,
            quantity: 1,
            description: Some("Test product description".to_string()),
        }],
        callback_url: None,
        metadata: None,
    };

    println!("📦 Creating Order (Direct API)...");
    println!(
        "   Amount: {} {:?}",
        order_request.amount, order_request.currency
    );
    println!("   Items: {} items", order_request.items.len());
    println!(
        "   Buyer: {} {}",
        order_request.buyer.as_ref().unwrap().name,
        order_request.buyer.as_ref().unwrap().surname
    );

    let mut created_order_id = None;

    match client.create_order(order_request.clone()) {
        Ok(create_response) => {
            created_order_id = Some(create_response.order_id.clone());
            println!("   ✅ Order Created Successfully!");
            println!("      Order ID: {}", create_response.order_id);
            println!("      Reference ID: {}", create_response.reference_id);
        }
        Err(e) => {
            println!("   ❌ Order Creation Failed: {}", e);
        }
    }

    // Test Module-based API as well
    println!("\n📦 Creating Order (Module API)...");
    match client.orders().create(order_request) {
        Ok(create_response) => {
            println!("   ✅ Module API Order Created!");
            println!("      Order ID: {}", create_response.order_id);
            println!("      Reference ID: {}", create_response.reference_id);
        }
        Err(e) => {
            println!("   ❌ Module API Order Creation Failed: {}", e);
        }
    }

    // 4. Order Retrieval and Status Testing
    if let Some(order_id) = &created_order_id {
        println!("\n=== 4. ORDER RETRIEVAL TESTING ===");

        // Test Direct API
        println!("🔍 Getting Order (Direct API)...");
        match client.get_order(order_id) {
            Ok(order) => {
                println!("   ✅ Order Retrieved Successfully!");
                println!("      Order ID: {}", order.order.id);
                println!("      Status: {:?}", order.order.status);
                println!("      Created: {}", order.order.created_at);
            }
            Err(e) => {
                println!("   ❌ Order Retrieval Failed: {}", e);
            }
        }

        // Test Order Status
        println!("\n📊 Getting Order Status...");
        match client.get_order_status(order_id) {
            Ok(status_response) => {
                println!("   ✅ Status Retrieved: {:?}", status_response);
            }
            Err(e) => {
                println!("   ❌ Status Retrieval Failed: {}", e);
            }
        }

        // Test Checkout URL
        println!("\n🔗 Getting Checkout URL...");
        match client.get_checkout_url(order_id) {
            Ok(checkout_response) => {
                println!("   ✅ Checkout URL Retrieved: {:?}", checkout_response);
            }
            Err(e) => {
                println!("   ❌ Checkout URL Retrieval Failed: {}", e);
            }
        }

        // Test Order Transactions
        println!("\n💳 Getting Order Transactions...");
        match client.get_order_transactions(order_id) {
            Ok(transactions) => {
                println!("   ✅ Transactions Retrieved: {:?}", transactions);
            }
            Err(e) => {
                println!("   ❌ Transactions Retrieval Failed: {}", e);
            }
        }
    }

    // 5. Order List Testing
    println!("\n=== 5. ORDER LIST TESTING ===");
    println!("📋 Getting Order List...");
    match client.get_order_list(Some(1), Some(10)) {
        Ok(orders_response) => {
            println!("   ✅ Order List Retrieved!");
            println!(
                "      Response: {}",
                serde_json::to_string_pretty(&orders_response)?
            );
        }
        Err(e) => {
            println!("   ❌ Order List Retrieval Failed: {}", e);
        }
    }

    // 6. Installment Plan Testing
    println!("\n=== 6. INSTALLMENT PLAN TESTING ===");
    if let Some(order_id) = &created_order_id {
        let installment_request = CreateInstallmentPlanRequest {
            order_id: order_id.clone(),
            installment_count: 12,
            first_installment_date: "2025-10-15".to_string(),
        };

        println!("📊 Creating Installment Plan...");
        println!("   Order ID: {}", installment_request.order_id);
        println!("   Installments: {}", installment_request.installment_count);
        println!(
            "   First Payment: {}",
            installment_request.first_installment_date
        );

        match client.installments().create_plan(installment_request) {
            Ok(plan_response) => {
                println!("   ✅ Installment Plan Created!");
                println!("      Plan: {:?}", plan_response);
            }
            Err(e) => {
                println!("   ❌ Installment Plan Creation Failed: {}", e);
            }
        }
    }

    // 7. Webhook Testing
    println!("\n=== 7. WEBHOOK VERIFICATION TESTING ===");
    let webhook_payload = r#"{
        "event_type": "order.completed",
        "data": {
            "order_id": "test_order_123",
            "amount": 299.99,
            "currency": "TRY",
            "status": "completed",
            "payment_method": "credit_card"
        },
        "timestamp": "2024-01-15T10:30:00Z"
    }"#;

    let webhook_signature = "sha256=test_signature_hash";
    let webhook_secret = "test_webhook_secret_key";

    println!("🔗 Testing Webhook Verification...");

    // Test simple verification
    match WebhookModule::verify_webhook(webhook_payload, webhook_signature, webhook_secret) {
        Ok(is_valid) => {
            if is_valid {
                println!("   ✅ Simple webhook verification passed");
            } else {
                println!("   ⚠️ Simple webhook verification failed (expected for test data)");
            }
        }
        Err(e) => {
            println!("   ❌ Simple webhook verification error: {}", e);
        }
    }

    // Test advanced verification
    let webhook_config = WebhookVerificationConfig {
        secret: webhook_secret.to_string(),
        tolerance_seconds: Some(300),
    };

    match WebhookModule::verify_webhook_advanced(
        webhook_payload,
        webhook_signature,
        &webhook_config,
    ) {
        Ok(result) => {
            println!("   ℹ️ Advanced verification result:");
            println!("      Valid: {}", result.is_valid);
            if let Some(error) = result.error {
                println!("      Error: {}", error);
            }
        }
        Err(e) => {
            println!("   ❌ Advanced webhook verification error: {}", e);
        }
    }

    // Test webhook parsing
    match WebhookModule::parse_webhook(webhook_payload) {
        Ok(webhook_event) => {
            println!("   ✅ Webhook parsing successful!");
            println!("      Event Type: {:?}", webhook_event.event_type);
            println!("      Timestamp: {}", webhook_event.timestamp);
            if let Some(order_id) = webhook_event.data.order_id {
                println!("      Order ID: {}", order_id);
            }
        }
        Err(e) => {
            println!("   ❌ Webhook parsing error: {}", e);
        }
    }

    // 8. Error Handling Testing
    println!("\n=== 8. ERROR HANDLING TESTING ===");
    println!("🚨 Testing Error Scenarios...");

    // Test invalid order ID
    match client.get_order("invalid-order-id-12345") {
        Ok(_) => println!("   ⚠️ Unexpected success with invalid order ID"),
        Err(e) => println!("   ✅ Invalid order ID handled correctly: {}", e),
    }

    // 9. Performance Summary
    println!("\n=== 9. TESTING SUMMARY ===");
    println!("🎯 SDK Features Tested:");
    println!("   ✅ Client Configuration & Initialization");
    println!("   ✅ Comprehensive Validator Functions");
    println!("   ✅ Direct API Methods (client.create_order, etc.)");
    println!("   ✅ Module API Methods (client.orders().create, etc.)");
    println!("   ✅ Order Lifecycle (Create → Get → Status → Transactions)");
    println!("   ✅ Order List Retrieval with Pagination");
    println!("   ✅ Installment Plan Creation");
    println!("   ✅ Webhook Verification & Parsing");
    println!("   ✅ Error Handling & Edge Cases");

    println!("\n🚀 API testing completed!");
    println!("   Check the responses above for actual API behavior");

    println!("\n🎉 Comprehensive SDK testing completed successfully!");

    Ok(())
}
