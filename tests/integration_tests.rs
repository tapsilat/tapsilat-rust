use tapsilat::{Config, CreateOrderRequest, TapsilatClient};

#[test]
fn test_client_creation() {
    let client = TapsilatClient::from_api_key("test-api-key");
    assert!(client.is_ok());
}

#[test]
fn test_config_validation() {
    let config = Config::new("test-key");
    assert!(config.validate().is_ok());

    let empty_config = Config::new("");
    assert!(empty_config.validate().is_err());
}

#[test]
fn test_order_creation_request() {
    let request = CreateOrderRequest {
        amount: 100.0,
        currency: "TRY".to_string(),
        locale: "tr".to_string(),
        conversation_id: Some("test-123".to_string()),
        basket_items: Some(vec![tapsilat::types::BasketItemDTO {
            id: Some("item1".to_string()),
            name: Some("Test Item".to_string()),
            price: Some(100.0),
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

    // Should be valid
    assert_eq!(request.amount, 100.0);
    assert_eq!(request.currency, "TRY".to_string());
    assert!(request.basket_items.is_some());
    assert_eq!(request.basket_items.unwrap().len(), 1);
}

#[test]
fn test_validators() {
    use tapsilat::Validators;

    // GSM validation
    assert!(Validators::validate_gsm("5551234567").is_ok());
    assert!(Validators::validate_gsm("05551234567").is_ok());
    assert!(Validators::validate_gsm("+905551234567").is_ok());
    assert!(Validators::validate_gsm("4551234567").is_err()); // Doesn't start with 5

    // Email validation
    assert!(Validators::validate_email("test@example.com").is_ok());
    assert!(Validators::validate_email("invalid-email").is_err());

    // Amount validation
    assert!(Validators::validate_amount(10.50).is_ok());
    assert!(Validators::validate_amount(-5.0).is_err());
    assert!(Validators::validate_amount(0.0).is_err());

    // Installments validation
    assert!(Validators::validate_installments(1).is_ok());
    assert!(Validators::validate_installments(12).is_ok());
    assert!(Validators::validate_installments(0).is_err());
    assert!(Validators::validate_installments(13).is_err());
}
