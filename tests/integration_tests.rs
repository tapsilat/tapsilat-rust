use tapsilat::{TapsilatClient, Config, CreateOrderRequest, CreateOrderItemRequest, Currency};

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
        currency: Currency::TRY,
        description: Some("Test order".to_string()),
        items: vec![
            CreateOrderItemRequest {
                name: "Test Item".to_string(),
                price: 100.0,
                quantity: 1,
                description: None,
            }
        ],
        buyer: None,
        callback_url: None,
        metadata: None,
    };
    
    // Should be valid
    assert_eq!(request.amount, 100.0);
    assert!(matches!(request.currency, Currency::TRY));
    assert_eq!(request.items.len(), 1);
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