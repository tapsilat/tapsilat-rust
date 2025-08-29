use mockito::{Server, ServerGuard};
use serde_json::json;
use tapsilat::{
    TapsilatClient, Config, CreateOrderRequest, CreateOrderItemRequest, Currency,
    CreateInstallmentPlanRequest, RefundOrderRequest
};

async fn setup_mock_server() -> ServerGuard {
    Server::new_async().await
}

#[tokio::test]
async fn test_order_creation_with_mock() {
    let mut server = setup_mock_server().await;
    
    // Mock successful order creation
    let mock_response = json!({
        "success": true,
        "data": {
            "order": {
                "id": "order_123",
                "amount": 149.99,
                "currency": "TRY",
                "status": "pending",
                "description": "Test Order",
                "buyer": null,
                "items": [
                    {
                        "name": "Test Item",
                        "price": 149.99,
                        "quantity": 1,
                        "description": null
                    }
                ],
                "callback_url": null,
                "checkout_url": null,
                "created_at": "2023-12-01T10:30:00Z",
                "updated_at": "2023-12-01T10:30:00Z",
                "metadata": null
            },
            "checkout_url": "https://checkout.tapsilat.com/order_123"
        },
        "message": "Order created successfully"
    });

    let _mock = server
        .mock("POST", "/orders")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    // Configure client to use mock server
    let config = Config::new("test-api-key")
        .with_base_url(&server.url());
    
    let client = TapsilatClient::new(config).unwrap();

    let order_request = CreateOrderRequest {
        amount: 149.99,
        currency: Currency::TRY,
        description: Some("Test Order".to_string()),
        items: vec![CreateOrderItemRequest {
            name: "Test Item".to_string(),
            price: 149.99,
            quantity: 1,
            description: None,
        }],
        buyer: None,
        callback_url: None,
        metadata: None,
    };

    let result = client.orders().create(order_request);
    assert!(result.is_ok(), "Order creation should succeed with mock");

    let order_response = result.unwrap();
    assert_eq!(order_response.order.id, "order_123");
    assert_eq!(order_response.order.amount, 149.99);
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
        .mock("GET", "/orders/order_123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let config = Config::new("test-api-key")
        .with_base_url(&server.url());
    
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

    let config = Config::new("test-api-key")
        .with_base_url(&server.url());
    
    let client = TapsilatClient::new(config).unwrap();

    let installment_request = CreateInstallmentPlanRequest {
        order_id: "order_123".to_string(),
        installment_count: 6,
        first_installment_date: "2024-01-15".to_string(),
    };

    let result = client.installments().create_plan(installment_request);
    assert!(result.is_ok(), "Installment plan creation should succeed with mock");

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
        .mock("POST", "/orders")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(mock_error_response.to_string())
        .create_async()
        .await;

    let config = Config::new("invalid-api-key")
        .with_base_url(&server.url());
    
    let client = TapsilatClient::new(config).unwrap();

    let order_request = CreateOrderRequest {
        amount: 149.99,
        currency: Currency::TRY,
        description: Some("Test Order".to_string()),
        items: vec![CreateOrderItemRequest {
            name: "Test Item".to_string(),
            price: 149.99,
            quantity: 1,
            description: None,
        }],
        buyer: None,
        callback_url: None,
        metadata: None,
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
        .mock("POST", "/orders/order_123/refund")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let config = Config::new("test-api-key")
        .with_base_url(&server.url());
    
    let client = TapsilatClient::new(config).unwrap();

    let refund_request = RefundOrderRequest {
        amount: Some(50.0),
        reason: Some("Customer request".to_string()),
    };

    let result = client.orders().refund("order_123", refund_request);
    assert!(result.is_ok(), "Order refund should succeed with mock");

    let refund = result.unwrap();
    assert_eq!(refund.refund_id, "refund_789");
    assert_eq!(refund.refund_amount, 50.0);
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

    let config = Config::new("test-api-key")
        .with_base_url(&server.url());
    
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