use mockito::{Server, ServerGuard};
use serde_json::json;
use tapsilat::{
    Config, CreateInstallmentPlanRequest, CreateOrderRequest, RefundOrderRequest, TapsilatClient,
};

async fn setup_mock_server() -> ServerGuard {
    Server::new_async().await
}

#[tokio::test]
async fn test_order_creation_with_mock() {
    let mut server = setup_mock_server().await;

    // Mock successful order creation (matches actual API response format)
    let mock_response = json!({
        "order_id": "order_123",
        "reference_id": "ref_12345"
    });

    let _mock = server
        .mock("POST", "/order/create")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    // Configure client to use mock server
    let config = Config::new("test-api-key").with_base_url(&server.url());

    let client = TapsilatClient::new(config).unwrap();

    let order_request = CreateOrderRequest {
        amount: 149.99,
        currency: "TRY".to_string(),
        locale: "tr".to_string(),
        conversation_id: Some("test-123".to_string()),
        basket_items: Some(vec![tapsilat::types::BasketItemDTO {
            id: Some("item1".to_string()),
            name: Some("Test Item".to_string()),
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

    let result = client.orders().create(order_request);
    assert!(result.is_ok(), "Order creation should succeed with mock");

    let create_response = result.unwrap();
    assert_eq!(create_response.order_id, Some("order_123".to_string()));
    assert_eq!(create_response.reference_id, Some("ref_12345".to_string()));
}

#[tokio::test]
async fn test_order_get_with_mock() {
    let mut server = setup_mock_server().await;

    let mock_response = json!({
        "success": true,
        "data": {
            "id": "order_123",
            "amount": 299.99,
            "currency": "TRY",
            "status": "completed",
            "description": "Test order",
            "buyer": null,
            "items": [
                {
                    "name": "Test Item",
                    "price": 299.99,
                    "quantity": 1,
                    "description": null
                }
            ],
            "callback_url": null,
            "checkout_url": null,
            "created_at": "2023-12-01T10:30:00Z",
            "updated_at": "2023-12-01T10:30:00Z",
            "metadata": null
        }
    });

    let _mock = server
        .mock("GET", "/order/order_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let config = Config::new("test-api-key").with_base_url(&server.url());

    let client = TapsilatClient::new(config).unwrap();

    let result = client.orders().get("order_123");
    assert!(result.is_ok(), "Order get should succeed with mock");

    let order = result.unwrap();
    assert_eq!(order.id, "order_123");
    assert_eq!(order.amount, 299.99);
}

#[tokio::test]
async fn test_installment_plan_creation_with_mock() {
    let mut server = setup_mock_server().await;

    let mock_response = json!({
        "success": true,
        "data": {
            "id": "plan_456",
            "order_id": "order_123",
            "total_installments": 6,
            "installment_amount": 50.0,
            "currency": "TRY",
            "status": "pending",
            "installments": [
                {
                    "id": "inst_1",
                    "installment_number": 1,
                    "amount": 50.0,
                    "due_date": "2024-01-15",
                    "paid_at": null,
                    "status": "pending"
                }
            ],
            "created_at": "2023-12-01T10:30:00Z",
            "updated_at": "2023-12-01T10:30:00Z"
        }
    });

    let _mock = server
        .mock("POST", "/installments/plans")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let config = Config::new("test-api-key").with_base_url(&server.url());

    let client = TapsilatClient::new(config).unwrap();

    let installment_request = CreateInstallmentPlanRequest {
        order_id: "order_123".to_string(),
        installment_count: 6,
        first_installment_date: "2024-01-15".to_string(),
    };

    let result = client.installments().create_plan(installment_request);
    assert!(
        result.is_ok(),
        "Installment plan creation should succeed with mock"
    );

    let plan = result.unwrap();
    assert_eq!(plan.id, "plan_456");
    assert_eq!(plan.total_installments, 6);
    assert_eq!(plan.installment_amount, 50.0);
}

#[tokio::test]
async fn test_error_handling_with_mock() {
    let mut server = setup_mock_server().await;

    let mock_error_response = json!({
        "success": false,
        "message": "Invalid API key",
        "errors": ["Authentication failed"]
    });

    let _mock = server
        .mock("POST", "/order/create")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(mock_error_response.to_string())
        .create_async()
        .await;

    let config = Config::new("invalid-api-key").with_base_url(&server.url());

    let client = TapsilatClient::new(config).unwrap();

    let order_request = CreateOrderRequest {
        amount: 149.99,
        currency: "TRY".to_string(),
        locale: "tr".to_string(),
        conversation_id: Some("test-123".to_string()),
        basket_items: Some(vec![tapsilat::types::BasketItemDTO {
            id: Some("item1".to_string()),
             name: Some("Test Item".to_string()),
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

    let result = client.orders().create(order_request);
    assert!(result.is_err(), "Should return error for invalid API key");
}

#[tokio::test]
async fn test_order_refund_with_mock() {
    let mut server = setup_mock_server().await;

    let mock_response = json!({
        "success": true,
        "data": {
            "refund_id": "refund_789",
            "refund_amount": 50.0,
            "order": {
                "id": "order_123",
                "amount": 299.99,
                "currency": "TRY",
                "status": "partially_refunded",
                "description": "Test order",
                "buyer": null,
                "items": [],
                "callback_url": null,
                "checkout_url": null,
                "created_at": "2023-12-01T10:30:00Z",
                "updated_at": "2023-12-01T10:30:00Z",
                "metadata": null
            }
        }
    });

    let _mock = server
        .mock("POST", "/order/refund")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let config = Config::new("test-api-key").with_base_url(&server.url());

    let client = TapsilatClient::new(config).unwrap();

    let refund_request = RefundOrderRequest {
        amount: 50.0,
        reference_id: "order_123".to_string(), // In struct, this field exists
        order_item_id: None,
        order_item_payment_id: None,
    };

    // The method seems to be taking just the request object in source, so we match that.
    // However, the test was mocking /orders/order_123/refund which implies path param.
    // If the SDK implementation relies on request.reference_id, let's see.
    // The previous error said "this method takes 1 argument but 2 arguments were supplied".
    // So `client.orders().refund(request)` is checking out.
    let result = client.orders().refund(refund_request);
    assert!(result.is_ok(), "Order refund should succeed with mock");

    let refund_val = result.unwrap();
    // refund_val is serde_json::Value
    assert_eq!(refund_val["refund_id"], "refund_789");
    assert_eq!(refund_val["refund_amount"], 50.0);
}

#[tokio::test]
async fn test_pagination_with_mock() {
    let mut server = setup_mock_server().await;

    let mock_response = json!({
        "success": true,
        "data": {
            "data": [
                {
                    "id": "plan_1",
                    "order_id": "order_123",
                    "total_installments": 6,
                    "installment_amount": 50.0,
                    "currency": "TRY",
                    "status": "pending",
                    "installments": [],
                    "created_at": "2023-12-01T10:30:00Z",
                    "updated_at": "2023-12-01T10:30:00Z"
                }
            ],
            "pagination": {
                "current_page": 1,
                "per_page": 10,
                "total": 1,
                "total_pages": 1
            }
        }
    });

    let _mock = server
        .mock("GET", "/installments/plans?page=1&per_page=10")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let config = Config::new("test-api-key").with_base_url(&server.url());

    let client = TapsilatClient::new(config).unwrap();

    let pagination = tapsilat::PaginationParams {
        page: Some(1),
        per_page: Some(10),
    };

    let result = client.installments().list_plans(Some(pagination));
    assert!(result.is_ok(), "Pagination should work with mock");

    let paginated_response = result.unwrap();
    assert_eq!(paginated_response.data.len(), 1);
    assert_eq!(paginated_response.pagination.current_page, 1);
    assert_eq!(paginated_response.pagination.total, 1);
}
