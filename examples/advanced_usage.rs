use chrono::Utc;

use std::env;
use tapsilat::{
    types::{
        BasketItemDTO, BillingAddressDTO, SubscriptionBilling, SubscriptionCreateRequest,
        SubscriptionUser,
    },
    Config, CreateBuyerRequest, CreateOrderRequest, TapsilatClient, Validators,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Tapsilat Rust SDK - Comprehensive Test Arena");
    println!("===============================================\n");

    // Get API key from environment
    let api_key = env::var("TAPSILAT_API_KEY").unwrap_or_else(|_| "<API_KEY>".to_string());

    println!(
        "🚀 Running with API KEY: {}...{}",
        &api_key[..8],
        &api_key[api_key.len() - 4..]
    );
    println!();

    // 1. Advanced Client Configuration
    println!("=== 1. CLIENT CONFIGURATION ===");
    let config = Config::new(&api_key)
        .with_base_url("https://panel.tapsilat.dev/api/v1")
        .with_timeout(30);

    let client = TapsilatClient::new(config)?;
    println!("✅ Client configured successfully");

    // 2. Validator Testing
    println!("\n=== 2. VALIDATOR TESTING ===");
    // GSM
    let gsm = "5551234567";
    match Validators::validate_gsm_number(gsm) {
        Ok(normalized) => println!("   ✅ GSM {} → {}", gsm, normalized),
        Err(e) => println!("   ❌ GSM {} → Error: {}", gsm, e),
    }

    // Amount
    let amount = 10.99;
    match Validators::validate_amount(amount) {
        Ok(_) => println!("   ✅ Amount {} → Valid", amount),
        Err(e) => println!("   ❌ Amount {} → Error: {}", amount, e),
    }

    // 3. Order Creation
    println!("\n=== 3. ORDER LIFECYCLE TESTING ===");

    let buyer = CreateBuyerRequest {
        name: "Ahmet".to_string(),
        surname: "Yılmaz".to_string(),
        email: Some("ahmet.yilmaz@gmail.com".to_string()),
        gsm_number: Some("5321234567".to_string()),
        identity_number: None,
        registration_address: None,
        ip: None,
        city: Some("Istanbul".to_string()),
        country: Some("Turkey".to_string()),
        zip_code: None,
    };

    let billing_address = BillingAddressDTO {
        address: Some("Maslak Mah. Dereboyu Cad.".to_string()),
        city: Some("Istanbul".to_string()),
        country: Some("Turkey".to_string()),
        contact_name: Some("Ahmet Yilmaz".to_string()),
        // populate other fields as needed or leave None
        billing_type: None,
        citizenship: None,
        contact_phone: None,
        district: None,
        tax_office: None,
        title: None,
        vat_number: None,
        zip_code: None,
    };

    let basket_item = BasketItemDTO {
        name: Some("Test Product".to_string()),
        price: Some(299.99),
        item_type: Some("PHYSICAL".to_string()), // Example
        category1: Some("Electronics".to_string()),
        // Initialize other Option fields to None
        category2: None,
        commission_amount: None,
        coupon: None,
        coupon_discount: None,
        data: None,
        id: None,
        paid_amount: None,
        payer: None,
        quantity: Some(1),
        quantity_float: None,
        quantity_unit: None,
        sub_merchant_key: None,
        sub_merchant_price: None,
    };

    let order_request = CreateOrderRequest {
        amount: 299.99,
        currency: "TRY".to_string(),
        locale: "tr".to_string(),
        conversation_id: Some(format!("order-{}", Utc::now().timestamp())),
        buyer: buyer.clone(),
        basket_items: Some(vec![basket_item]),
        billing_address: Some(billing_address),
        shipping_address: None, // Can populate if needed
        payment_success_url: Some("https://example.com/success".to_string()),
        payment_failure_url: Some("https://example.com/fail".to_string()),
        // Initialize other Option fields to None
        checkout_design: None,
        enabled_installments: None,
        external_reference_id: None,
        metadata: None,
        order_cards: None,
        paid_amount: None,
        partial_payment: None,
        payment_methods: None,
        payment_mode: None,
        payment_options: None,
        payment_terms: None,
        pf_sub_merchant: None,
        redirect_failure_url: None,
        redirect_success_url: None,
        sub_organization: None,
        submerchants: None,
        tax_amount: None,
        three_d_force: None,
        consents: None,
    };

    println!("📦 Creating Order...");
    let mut created_reference_id = None;

    match client.create_order(order_request.clone()) {
        Ok(create_response) => {
            println!("   ✅ Order Created Successfully!");
            if let Some(oid) = &create_response.order_id {
                println!("      Order ID: {}", oid);
            }
            if let Some(ref_id) = &create_response.reference_id {
                println!("      Reference ID: {}", ref_id);
                created_reference_id = Some(ref_id.clone());
            }
            if let Some(url) = &create_response.checkout_url {
                println!("      Checkout URL: {}", url);
            }
        }
        Err(e) => println!("   ❌ Order Creation Failed: {}", e),
    }

    // 4. Order Retrieval
    if let Some(ref_id) = &created_reference_id {
        println!("\n=== 4. ORDER RETRIEVAL TESTING ===");
        match client.get_order(ref_id) {
            Ok(order) => {
                println!("   ✅ Order Retrieved!");
                println!("      ID: {}", order.id.unwrap_or_default());
                println!("      Status: {:?}", order.status);
            }
            Err(e) => println!("   ❌ Order Retrieval Failed: {}", e),
        }

        println!("\n📊 Getting Order Status...");
        match client.get_order_status(ref_id) {
            Ok(status) => println!("   ✅ Status: {:?}", status),
            Err(e) => println!("   ❌ Status Failed: {}", e),
        }
    }

    // 5. Order List
    println!("\n=== 5. ORDER LIST TESTING ===");
    match client.get_order_list(1, 10, None) {
        Ok(list) => println!("   ✅ Order List Retrieved: {:?}", list),
        Err(e) => println!("   ❌ Order List Failed: {}", e),
    }

    // 6. Subscriptions (NEW)
    println!("\n=== 6. SUBSCRIPTION TESTING ===");
    let sub_request = SubscriptionCreateRequest {
        amount: Some(100.0),
        currency: Some("TRY".to_string()),
        period: Some(1), // Monthly?
        title: Some("Test Subscription".to_string()),
        billing: Some(SubscriptionBilling {
            contact_name: Some("Sub Subscriber".to_string()),
            city: Some("Istanbul".to_string()),
            country: Some("TR".to_string()),
            address: None,
            vat_number: None,
            zip_code: None,
        }),
        user: Some(SubscriptionUser {
            first_name: Some("Ahmet".to_string()),
            last_name: Some("Yılmaz".to_string()),
            email: Some("ahmet.yilmaz@gmail.com".to_string()),
            phone: Some("5321234567".to_string()),
            city: Some("Istanbul".to_string()),
            country: Some("Turkey".to_string()),
            address: None,
            id: None,
            identity_number: None,
            zip_code: None,
        }),
        // Initialize others to None
        card_id: None,
        cycle: None,
        external_reference_id: Some("ext_sub_01".to_string()),
        failure_url: None,
        payment_date: None,
        price_option: None,
        success_url: None,
    };

    match client.create_subscription(sub_request) {
        Ok(sub_resp) => {
            println!("   ✅ Subscription Created!");
            println!("      Ref ID: {:?}", sub_resp.reference_id);

            // List Subscriptions
            match client.list_subscriptions(1, 5) {
                Ok(list) => println!("   ✅ Subscriptions List: {:?}", list),
                Err(e) => println!("   ❌ List Subscriptions Failed: {}", e),
            }
        }
        Err(e) => println!("   ❌ Subscription Creation Failed: {}", e),
    }

    // 7. Term Management (Terminating a term)
    println!("\n=== 7. TERM MANAGEMENT (Termination) ===");
    // Attempting on a dummy term ID, expected to fail or 404, but tests the method signature
    match client.terminate_order_term("non_existent_term_id", Some("Test Reason".to_string())) {
        Ok(resp) => println!(
            "   ✅ Term Terminated (Unexpected success for dummy ID): {:?}",
            resp
        ),
        Err(e) => println!(
            "   ✅ Term Termination Failed as expected (dummy ID): {}",
            e
        ),
    }

    // 8. API Health
    println!("\n=== 8. HEALTH CHECK ===");
    match client.health_check() {
        Ok(h) => println!("   ✅ Health: {:?}", h),
        Err(e) => println!("   ❌ Health Check Failed: {}", e),
    }

    println!("\n🎉 Advanced usage test completed!");
    Ok(())
}
