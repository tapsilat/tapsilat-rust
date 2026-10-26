//! Request and response contract tests against the panel REST API, using a mock server.

use mockito::{Matcher, Server, ServerGuard};
use serde_json::json;
use tapsilat::types::*;
use tapsilat::{Config, TapsilatClient, TapsilatError};

fn client(server: &ServerGuard) -> TapsilatClient {
    TapsilatClient::new(Config::new("test-key").with_base_url(format!("{}/api/v1", server.url())))
        .unwrap()
}

fn json_mock(
    server: &mut ServerGuard,
    method: &str,
    path: &str,
    status: usize,
    body: serde_json::Value,
) -> mockito::Mock {
    server
        .mock(method, path)
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_body(body.to_string())
}

#[test]
fn requests_carry_auth_accept_and_user_agent_headers() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/ref-1/status",
        200,
        json!({"status": "Paid"}),
    )
    .match_header("authorization", "Bearer test-key")
    .match_header("accept", "application/json")
    .match_header("user-agent", Matcher::Regex("^tapsilat-rust/".into()))
    .create();

    let status = client(&server).get_order_status("ref-1").unwrap();
    assert_eq!(status["status"], "Paid");
    mock.assert();
}

#[test]
fn create_order_cleans_gsm_omits_unset_fields_and_fills_checkout_url() {
    let mut server = Server::new();
    let create = json_mock(&mut server, "POST", "/api/v1/order/create", 200, json!({"order_id": "order-1", "reference_id": "ref-1"}))
        .match_body(Matcher::Json(json!({
            "amount": 100.0,
            "currency": "TRY",
            "locale": "tr",
            "buyer": {"name": "John", "surname": "Doe", "gsm_number": "+905551234567", "birth_date": "1990-01-02"},
            "order_cards": [{"card_id": "card-1", "card_sequence": 1}],
            "payment_terms": [{"amount": 50.0, "due_date": "2026-01-15T00:00:00Z", "term_sequence": 1}],
            "checkout_design": {"accent_color": "#123456", "dark_mode_enabled": true, "dark_colors": {"text_color": "#ffffff"}}
        })))
        .create();
    let detail = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/ref-1",
        200,
        json!({"reference_id": "ref-1", "checkout_url": "https://checkout.example/ref-1"}),
    )
    .create();

    let response = client(&server)
        .create_order(CreateOrderRequest {
            amount: 100.0,
            currency: "TRY".into(),
            locale: "tr".into(),
            buyer: CreateBuyerRequest {
                name: "John".into(),
                surname: "Doe".into(),
                gsm_number: Some("+90 (555) 123 45 67".into()),
                birth_date: Some("1990-01-02".into()),
                ..Default::default()
            },
            order_cards: Some(vec![OrderCardDTO {
                card_id: "card-1".into(),
                card_sequence: 1,
            }]),
            payment_terms: Some(vec![PaymentTermDTO {
                amount: Some(50.0),
                due_date: Some("2026-01-15T00:00:00Z".into()),
                term_sequence: Some(1),
                ..Default::default()
            }]),
            checkout_design: Some(CheckoutDesignDTO {
                accent_color: Some("#123456".into()),
                dark_mode_enabled: Some(true),
                dark_colors: Some([("text_color".to_string(), "#ffffff".to_string())].into()),
                ..Default::default()
            }),
            ..Default::default()
        })
        .unwrap();

    create.assert();
    detail.assert();
    assert_eq!(response.order_id.as_deref(), Some("order-1"));
    assert_eq!(
        response.checkout_url.as_deref(),
        Some("https://checkout.example/ref-1")
    );
}

#[test]
fn create_order_rejects_an_invalid_gsm_number_before_sending() {
    let server = Server::new();
    let result = client(&server).create_order(CreateOrderRequest {
        amount: 1.0,
        currency: "TRY".into(),
        locale: "tr".into(),
        buyer: CreateBuyerRequest {
            gsm_number: Some("call me".into()),
            ..Default::default()
        },
        ..Default::default()
    });
    assert!(matches!(result, Err(TapsilatError::ValidationError(_))));
}

#[test]
fn order_detail_decodes_without_an_envelope() {
    let mut server = Server::new();
    let mock = json_mock(&mut server, "GET", "/api/v1/order/ref%2F1", 200, json!({
        "reference_id": "ref/1",
        "amount": "100.00",
        "status": 3,
        "status_enum": "Paid",
        "customer_paid_amount": 101.5,
        "buyer": {"name": "John", "birth_date": "1990-01-02"},
        "basket_items": [{"id": "item-1", "price": 100.0, "mcc": "5411", "item_payments": [{"id": "pay-1", "type": "CREDIT_CARD", "refunded_date": "2026-01-01"}]}],
        "payment_terms": [{"id": "term-1", "term_reference_id": "t-1", "amount": 50.0, "due_date": "2026-01-15T00:00:00Z"}],
        "payment_requests": [{"id": "req-1", "amount": "100.00", "paid": true, "type": "auth"}],
        "refund_cancel_requests": [{"id": "rc-1", "status": 1, "type": 2}],
        "redirect_success_url": "https://shop.example/ok",
        "checkout_url": "https://checkout.example/ref-1"
    }))
    .create();

    let order = client(&server).get_order("ref/1").unwrap();
    mock.assert();
    assert_eq!(order.status, Some(3));
    assert_eq!(order.customer_paid_amount, Some(101.5));
    assert_eq!(
        order.buyer.unwrap().birth_date.as_deref(),
        Some("1990-01-02")
    );
    let item = &order.basket_items.unwrap()[0];
    assert_eq!(item.mcc.as_deref(), Some("5411"));
    assert_eq!(
        item.item_payments.as_ref().unwrap()[0].type_.as_deref(),
        Some("CREDIT_CARD")
    );
    assert_eq!(
        order.payment_terms.unwrap()[0].id.as_deref(),
        Some("term-1")
    );
    assert_eq!(order.payment_requests.unwrap()[0].paid, Some(true));
    assert_eq!(order.refund_cancel_requests.unwrap()[0].type_, Some(2));
    assert_eq!(
        order.redirect_success_url.as_deref(),
        Some("https://shop.example/ok")
    );
}

#[test]
fn order_by_conversation_id_returns_the_order_detail() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/conversation/conv-1",
        200,
        json!({"reference_id": "ref-1", "status": 2}),
    )
    .create();

    let order = client(&server)
        .get_order_by_conversation_id("conv-1")
        .unwrap();
    mock.assert();
    assert_eq!(order.reference_id.as_deref(), Some("ref-1"));
}

#[test]
fn order_list_sends_every_filter() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/list?page=2&per_page=50&start_date=2026-01-01&end_date=2026-01-31&conversation_id=c%201&buyer_id=b-1&status=3",
        200,
        json!({"rows": [], "total": 0, "page": 2, "per_page": 50, "total_pages": 0}),
    )
    .create();

    client(&server)
        .get_order_list_with_filter(&OrderListFilter {
            page: 2,
            per_page: 50,
            start_date: Some("2026-01-01".into()),
            end_date: Some("2026-01-31".into()),
            conversation_id: Some("c 1".into()),
            buyer_id: Some("b-1".into()),
            status: Some(3),
            ..Default::default()
        })
        .unwrap();
    mock.assert();
}

#[test]
fn order_submerchants_and_transactions_decode_the_panel_shapes() {
    let mut server = Server::new();
    let list = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/submerchants?page=1&per_page=10",
        200,
        json!({
            "row": [{"id": "sm-1", "name": "Seller", "submerchant_key": "key-1"}],
            "total": 1
        }),
    )
    .create();
    let transactions = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/ref-1/transactions",
        200,
        json!([{"id": "tx-1"}]),
    )
    .create();
    let none = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/ref-2/transactions",
        200,
        json!(null),
    )
    .create();

    let client = client(&server);
    let page = client.orders().list_submerchants(1, 10).unwrap();
    assert_eq!(page.rows[0].submerchant_key, "key-1");
    assert_eq!(
        client.get_order_transactions("ref-1").unwrap()[0]["id"],
        "tx-1"
    );
    assert!(client.get_order_transactions("ref-2").unwrap().is_null());
    list.assert();
    transactions.assert();
    none.assert();
}

#[test]
fn payment_terms_use_the_term_routes() {
    let mut server = Server::new();
    let get = json_mock(
        &mut server,
        "GET",
        "/api/v1/order/term?term_reference_id=t%2B1",
        200,
        json!({"term_sequence": 1, "due_date": {"seconds": 1, "nanos": 0}}),
    )
    .create();
    let create = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/term",
        200,
        json!({"message": "ok", "code": 0}),
    )
    .match_body(Matcher::Json(json!({
        "order_id": "order-1", "term_reference_id": "t-1", "amount": 10.0, "due_date": "2026-02-01",
        "term_sequence": 2, "required": true, "status": "PENDING"
    })))
    .create();
    let update = json_mock(
        &mut server,
        "PATCH",
        "/api/v1/order/term",
        200,
        json!({"message": "ok"}),
    )
    .match_body(Matcher::Json(
        json!({"term_reference_id": "t-1", "amount": 12.0, "due_date": "2026-02-02"}),
    ))
    .create();
    let delete = json_mock(
        &mut server,
        "DELETE",
        "/api/v1/order/term",
        200,
        json!({"message": "ok"}),
    )
    .match_body(Matcher::Json(
        json!({"order_id": "order-1", "term_reference_id": "t-1"}),
    ))
    .create();
    let refund = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/term/refund",
        200,
        json!({"is_success": true}),
    )
    .match_body(Matcher::Json(
        json!({"term_id": "term-1", "amount": 5.0, "reference_id": "ref-1"}),
    ))
    .create();

    let client = client(&server);
    assert_eq!(client.get_order_term("t+1").unwrap()["term_sequence"], 1);
    client
        .create_order_term(OrderPaymentTermCreateDTO {
            order_id: "order-1".into(),
            term_reference_id: "t-1".into(),
            amount: 10.0,
            due_date: "2026-02-01".into(),
            term_sequence: 2,
            required: true,
            status: "PENDING".into(),
            ..Default::default()
        })
        .unwrap();
    client
        .update_order_term(OrderPaymentTermUpdateDTO {
            term_reference_id: "t-1".into(),
            amount: Some(12.0),
            due_date: Some("2026-02-02".into()),
            ..Default::default()
        })
        .unwrap();
    client.delete_order_term("order-1", "t-1").unwrap();
    client
        .refund_order_term(OrderTermRefundRequest {
            term_id: "term-1".into(),
            amount: 5.0,
            reference_id: Some("ref-1".into()),
            term_payment_id: None,
        })
        .unwrap();

    for mock in [get, create, update, delete, refund] {
        mock.assert();
    }
}

#[test]
fn order_actions_use_the_panel_routes() {
    let mut server = Server::new();
    let callback = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/callback",
        200,
        json!({"message": "ok"}),
    )
    .match_body(Matcher::Json(
        json!({"reference_id": "ref-1", "conversation_id": "conv-1"}),
    ))
    .create();
    let related = json_mock(
        &mut server,
        "PATCH",
        "/api/v1/order/releated",
        200,
        json!({"status": 1, "is_success": true}),
    )
    .match_body(Matcher::Json(
        json!({"reference_id": "ref-1", "related_reference_id": "ref-0"}),
    ))
    .create();
    let refund_all = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/refund-all",
        200,
        json!({"status": "OK", "is_success": true}),
    )
    .match_body(Matcher::Json(json!({"reference_id": "ref-1"})))
    .create();
    let refund = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/refund",
        200,
        json!({"status": "OK", "message": "done", "is_success": true}),
    )
    .match_body(Matcher::Json(
        json!({"amount": 5.0, "reference_id": "ref-1", "order_item_id": "item-1"}),
    ))
    .create();

    let client = client(&server);
    client
        .order_manual_callback("ref-1", Some("conv-1".into()))
        .unwrap();
    client.order_related_update("ref-1", "ref-0").unwrap();
    client.refund_all_order("ref-1").unwrap();
    let refunded = client
        .refund_order(RefundOrderRequest {
            amount: 5.0,
            reference_id: "ref-1".into(),
            order_item_id: Some("item-1".into()),
            order_item_payment_id: None,
        })
        .unwrap();
    assert_eq!(refunded["is_success"], true);

    for mock in [callback, related, refund_all, refund] {
        mock.assert();
    }
}

#[test]
fn payments_payment_details_and_charge() {
    let mut server = Server::new();
    let payments = json_mock(&mut server, "POST", "/api/v1/order/payments", 200, json!({
        "payments": [{"id": "p-1", "amount": 100.0, "merchant_net_amount": 97.5, "type": "CREDIT_CARD"}],
        "total_amount": 100.0,
        "total_merchant_net_amount": 97.5,
        "order_amount": 100.0
    }))
    .match_body(Matcher::Json(json!({"order_reference_id": "ref-1"})))
    .create();
    let details = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/payment-details",
        200,
        json!({"rows": []}),
    )
    .match_body(Matcher::Json(json!({"conversation_id": "conv-1"})))
    .create();
    let charge = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/charge",
        200,
        json!({"isSuccess": true, "message": "charged", "statusCode": "00", "errorCode": ""}),
    )
    .match_body(Matcher::Json(
        json!({"order_reference_id": "ref-1", "retry": true}),
    ))
    .create();

    let client = client(&server);
    let response = client
        .get_order_payments(GetOrderPaymentsRequest {
            order_reference_id: Some("ref-1".into()),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(response.payments[0].type_, "CREDIT_CARD");
    assert_eq!(response.total_merchant_net_amount, 97.5);
    client
        .get_order_payment_details_by_conversation_id("conv-1")
        .unwrap();
    let charged = client
        .charge_order(OrderChargeRequest {
            order_reference_id: "ref-1".into(),
            retry: true,
        })
        .unwrap();
    assert!(charged.is_success);
    assert_eq!(charged.status_code, "00");

    for mock in [payments, details, charge] {
        mock.assert();
    }
}

#[test]
fn card_installments_and_installment_table() {
    let mut server = Server::new();
    let installments = json_mock(&mut server, "GET", "/api/v1/card/installments?bin=54066200&amount=100.5&currency=TRY&is_three_d=true", 200, json!({
        "is_installment_available": true,
        "installments": [{"installment": 3, "amount": "34.00", "total_amount": "102.00", "commission_rate": 1.5, "monthly_payment": "34.00"}]
    }))
    .create();
    let table = json_mock(&mut server, "GET", "/api/v1/card/installment-table?amount=100&currency=TRY", 200, json!({
        "groups": [{"bank": "Bank", "program": "Bonus", "program_code": "BONUS", "rows": [{"installment": 2, "is_interest_free": true}]}]
    }))
    .create();

    let client = client(&server);
    let options = client
        .get_card_installments(&CardInstallmentsRequest {
            bin: "54066200".into(),
            amount: 100.5,
            currency: "TRY".into(),
            is_three_d: true,
        })
        .unwrap();
    assert!(options.is_installment_available);
    assert_eq!(options.installments[0].commission_rate, 1.5);
    let groups = client
        .get_installment_table(&InstallmentTableRequest {
            amount: 100.0,
            currency: "TRY".into(),
            is_three_d: false,
        })
        .unwrap();
    assert!(groups.groups[0].rows[0].is_interest_free);
    installments.assert();
    table.assert();
}

#[test]
fn organization_routes() {
    let mut server = Server::new();
    let meta = json_mock(
        &mut server,
        "GET",
        "/api/v1/organization/metas/checkout%20theme",
        200,
        json!({"value": "dark"}),
    )
    .create();
    let limits = json_mock(
        &mut server,
        "GET",
        "/api/v1/organization/org-1/limits",
        200,
        json!({"limits": []}),
    )
    .create();
    let presets = json_mock(
        &mut server,
        "GET",
        "/api/v1/organization/currency-presets",
        200,
        json!({"items": [{"currency_code": "EUR", "currency_unit": "EUR", "minor_unit": 2}]}),
    )
    .create();
    let token = json_mock(
        &mut server,
        "POST",
        "/api/v1/organization/user/token",
        200,
        json!({"token": "tok", "user_id": "u-1"}),
    )
    .match_body(Matcher::Json(
        json!({"email": "a@example.com", "expire": 60, "user_id": "u-1"}),
    ))
    .create();
    let settings = json_mock(
        &mut server,
        "GET",
        "/api/v1/organization/settings",
        200,
        json!({"ttl": 30, "has_kyb_kyx_template": true}),
    )
    .create();
    let detail = json_mock(
        &mut server,
        "GET",
        "/api/v1/organization/suborganizations/sub-1",
        200,
        json!({"id": "sub-1", "parent_id": "org-1"}),
    )
    .create();

    let client = client(&server);
    client.get_organization_meta("checkout theme").unwrap();
    client.get_organization_limits_by_id("org-1").unwrap();
    assert_eq!(
        client.list_organization_currency_presets().unwrap().items[0].minor_unit,
        2
    );
    let created = client
        .create_organization_user_token(OrgUserTokenCreateRequest {
            email: Some("a@example.com".into()),
            expire: Some(60),
            user_id: Some("u-1".into()),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(created.token, "tok");
    assert!(
        client
            .organization()
            .settings()
            .unwrap()
            .has_kyb_kyx_template
    );
    assert_eq!(
        client
            .organization()
            .get_suborganization_detail("sub-1")
            .unwrap()
            .parent_id,
        "org-1"
    );

    for mock in [meta, limits, presets, token, settings, detail] {
        mock.assert();
    }
}

#[test]
fn submerchant_routes() {
    let mut server = Server::new();
    let list = json_mock(
        &mut server,
        "GET",
        "/api/v1/submerchants?page=1&per_page=20",
        200,
        json!({"row": [{"id": "sm-1", "status": "ACTIVE"}], "total": 1}),
    )
    .create();
    let create = json_mock(
        &mut server,
        "POST",
        "/api/v1/submerchants",
        200,
        json!({"code": 0, "message": "created"}),
    )
    .match_body(Matcher::PartialJson(
        json!({"name": "Seller", "sub_merchant_type": "PERSONAL", "currency_id": "TRY"}),
    ))
    .create();
    let update = json_mock(
        &mut server,
        "PATCH",
        "/api/v1/submerchants/sm-1",
        200,
        json!({"message": "updated"}),
    )
    .create();
    let delete = json_mock(
        &mut server,
        "DELETE",
        "/api/v1/submerchants/sm-1",
        200,
        json!({"message": "deleted"}),
    )
    .create();
    let item = json_mock(
        &mut server,
        "PUT",
        "/api/v1/submerchants/payment/item",
        200,
        json!({"operation_id": "op-1", "status": "SUCCESS"}),
    )
    .match_body(Matcher::PartialJson(json!({"sub_merchant_price": "10.50"})))
    .create();
    let event = json_mock(&mut server, "POST", "/api/v1/submerchant-payout-events", 202, json!({"id": "ev-1", "occurred_at": "2026-01-01T00:00:00Z"}))
        .match_body(Matcher::Json(json!({"idempotency_key": "k-1", "order_reference": "ref-1", "item_id": "item-1", "event_type": "DELIVERED"})))
        .create();

    let submerchants = client(&server).submerchants();
    assert_eq!(submerchants.list(1, 20).unwrap().rows[0].status, "ACTIVE");
    let request = SubmerchantCreateRequest {
        name: "Seller".into(),
        sub_merchant_type: "PERSONAL".into(),
        currency_id: "TRY".into(),
        ..Default::default()
    };
    assert_eq!(
        submerchants.create(request.clone()).unwrap().message,
        "created"
    );
    submerchants.update("sm-1", request).unwrap();
    submerchants.delete("sm-1").unwrap();
    submerchants
        .update_payment_item(SubmerchantPaymentItemUpdate {
            sub_merchant_price: "10.50".into(),
            ..Default::default()
        })
        .unwrap();
    let recorded = submerchants
        .record_payout_event(SubmerchantPayoutEventRequest {
            idempotency_key: "k-1".into(),
            order_reference: "ref-1".into(),
            item_id: "item-1".into(),
            event_type: "DELIVERED".into(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(recorded.id, "ev-1");

    for mock in [list, create, update, delete, item, event] {
        mock.assert();
    }
}

#[test]
fn vpos_update_sends_the_full_configuration() {
    let mut server = Server::new();
    let update = json_mock(
        &mut server,
        "PATCH",
        "/api/v1/vpos/vpos-1",
        200,
        json!({"code": 0, "message": "updated"}),
    )
    .match_body(Matcher::PartialJson(json!({
        "name": "Main", "failover": true, "tls": true, "pf_id": "pf-1",
        "main_installment": true, "main_without_installment": false, "type": "BANK"
    })))
    .create();
    let list = json_mock(
        &mut server,
        "GET",
        "/api/v1/vpos?page=1&per_page=10&suborganization_id=sub-1",
        200,
        json!({"rows": [{"id": "vpos-1"}]}),
    )
    .create();
    let templates = json_mock(&mut server, "GET", "/api/v1/vpos/acquirer-templates", 200, json!({"items": [{"acquirer_id": "a-1", "defaults": {"credentials": {"merchant": "required"}}}]}))
        .create();

    let vpos = client(&server).vpos();
    vpos.update(
        "vpos-1",
        VposUpdateRequest {
            name: "Main".into(),
            type_: "BANK".into(),
            failover: true,
            tls: true,
            pf_id: "pf-1".into(),
            main_installment: true,
            ..Default::default()
        },
    )
    .unwrap();
    let page = vpos
        .list_with_filter(
            1,
            10,
            &VposListFilter {
                suborganization_id: Some("sub-1".into()),
            },
        )
        .unwrap();
    assert_eq!(page.rows[0].id, "vpos-1");
    let items = vpos.list_acquirer_templates().unwrap().items;
    assert_eq!(items[0].defaults.credentials["merchant"], "required");

    for mock in [update, list, templates] {
        mock.assert();
    }
}

#[test]
fn saved_card_routes() {
    let mut server = Server::new();
    let list = json_mock(
        &mut server,
        "GET",
        "/api/v1/tokenization/card/list?page=1&per_page=10",
        200,
        json!({"rows": [{"id": "card-1", "last_four": "0004"}]}),
    )
    .create();
    let delete = json_mock(
        &mut server,
        "DELETE",
        "/api/v1/tokenization/card/card-1",
        200,
        json!({"success": true}),
    )
    .create();

    let cards = client(&server).tokenization();
    assert_eq!(
        cards.list_saved_cards(1, 10).unwrap().rows[0].last_four,
        "0004"
    );
    assert!(cards.delete_saved_card("card-1").unwrap().success);
    list.assert();
    delete.assert();
}

#[test]
fn marketplace_routes_use_api_v2_and_keep_revisions_opaque() {
    let mut server = Server::new();
    let revision = "2026-10-19T10:00:00.123456789Z";
    let seller = json_mock(
        &mut server,
        "GET",
        "/api/v2/submerchants/s-1",
        200,
        json!({"id": "s-1", "revision": revision, "approval_mode": null}),
    )
    .create();
    let update = json_mock(
        &mut server,
        "PATCH",
        "/api/v2/submerchants/s-1",
        200,
        json!({"approval_required": true}),
    )
    .match_body(Matcher::PartialJson(
        json!({"revision": revision, "name": "Seller"}),
    ))
    .create();
    let approve = json_mock(
        &mut server,
        "POST",
        "/api/v2/submerchant-payouts/approve",
        202,
        json!({"status": "accepted"}),
    )
    .match_body(Matcher::Json(json!({"ids": ["p-1", "p-2"]})))
    .create();
    let contracts = json_mock(
        &mut server,
        "GET",
        "/api/v2/marketplace/seller-contracts?currency_id=TRY&seller_type=PERSONAL&operation=create&vpos_id=v-1&vpos_id=v-2",
        200,
        json!({"contracts": {"v-1": {"supported": true, "fields": {"iban": true}}}}),
    )
    .create();
    let payouts = json_mock(
        &mut server,
        "GET",
        "/api/v2/submerchant-payouts?currency=TRY&page=1&per_page=20",
        200,
        json!({"rows": [{"id": "p-1", "revision": revision}]}),
    )
    .create();
    let delete = server
        .mock("DELETE", "/api/v2/marketplace/release-policies/rel-1")
        .with_status(204)
        .create();

    let marketplace = client(&server).marketplace();
    let current = marketplace.get_submerchant("s-1").unwrap();
    assert_eq!(current.revision, revision);
    assert!(current.approval_mode.is_none());
    let response = marketplace
        .update_submerchant(
            "s-1",
            MarketplaceProfileUpdate {
                profile: MarketplaceSubmerchantCreateRequest {
                    name: "Seller".into(),
                    ..Default::default()
                },
                revision: current.revision,
            },
        )
        .unwrap();
    assert!(response.approval_required);
    marketplace
        .approve_payouts(MarketplacePayoutApproval {
            ids: vec!["p-1".into(), "p-2".into()],
        })
        .unwrap();
    let found = marketplace
        .get_seller_contracts(&MarketplaceContractFilter {
            currency_id: "TRY".into(),
            seller_type: "PERSONAL".into(),
            operation: "create".into(),
            vpos_ids: vec!["v-1".into(), "v-2".into()],
        })
        .unwrap();
    assert!(found.contracts["v-1"].fields["iban"]);
    let page = marketplace
        .list_payouts(&MarketplacePayoutFilter {
            currency: Some("TRY".into()),
            page: 1,
            per_page: 20,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(page.rows[0].revision, revision);
    marketplace.delete_release_policy("rel-1").unwrap();

    for mock in [seller, update, approve, contracts, payouts, delete] {
        mock.assert();
    }
}

#[test]
fn approval_policy_currency_units_are_resolved_to_uuids() {
    let mut server = Server::new();
    let uuid = "3fa85f64-5717-4562-b3fc-2c963f66afa6";
    let currencies = json_mock(
        &mut server,
        "GET",
        "/api/v1/organization/currencies",
        200,
        json!({"currencies": [{"id": uuid, "currency_unit": "TRY"}]}),
    )
    .expect(1)
    .create();
    let create = json_mock(&mut server, "POST", "/api/v2/marketplace/approval-policies", 201, json!({"id": "pol-1"}))
        .match_body(Matcher::Json(json!({
            "name": "Large", "priority": 1, "enabled": true, "provider": "", "min_amount": "1000",
            "max_amount": null, "currency_id": uuid, "submerchant_ids": [], "labels_any": [], "mode": "manual"
        })))
        .create();
    let update = json_mock(
        &mut server,
        "PATCH",
        "/api/v2/marketplace/approval-policies/pol-1",
        200,
        json!({}),
    )
    .match_body(Matcher::PartialJson(json!({"currency_id": uuid})))
    .create();

    let client = client(&server);
    let input = MarketplaceApprovalPolicyInput {
        name: "Large".into(),
        priority: 1,
        enabled: Some(true),
        min_amount: Some("1000".into()),
        currency_id: "try".into(),
        mode: "manual".into(),
        ..Default::default()
    };
    client
        .marketplace()
        .create_approval_policy(input.clone())
        .unwrap();
    client
        .marketplace()
        .update_approval_policy("pol-1", input)
        .unwrap();
    assert_eq!(client.resolve_currency_id(uuid).unwrap(), uuid);
    assert!(matches!(
        client.resolve_currency_id("XYZ"),
        Err(TapsilatError::ValidationError(_))
    ));

    currencies.assert();
    create.assert();
    update.assert();
}

#[test]
fn validation_errors_expose_code_and_field_errors() {
    let mut server = Server::new();
    let _mock = json_mock(
        &mut server,
        "POST",
        "/api/v1/order/cancel",
        400,
        json!({
            "message": "INVALID_REQUEST_BODY",
            "code": 400000,
            "errors": {"reference_id": "required"}
        }),
    )
    .create();

    let error = client(&server).cancel_order("").unwrap_err();
    match &error {
        TapsilatError::ApiError {
            status_code,
            message,
            code,
            errors,
            status,
            ..
        } => {
            assert_eq!(*status_code, 400);
            assert_eq!(message, "INVALID_REQUEST_BODY");
            assert_eq!(code.as_deref(), Some("400000"));
            assert_eq!(status, "Bad Request");
            assert_eq!(errors.as_ref().unwrap()["reference_id"], "required");
        }
        other => panic!("unexpected error {other:?}"),
    }
    assert_eq!(
        error.to_string(),
        "API request failed with status 400 (Bad Request): [400000] INVALID_REQUEST_BODY"
    );
}

#[test]
fn error_string_with_numeric_code_is_parsed() {
    let mut server = Server::new();
    let _mock = json_mock(
        &mut server,
        "GET",
        "/api/v2/submerchant-payouts/missing",
        424,
        json!({"error": "payout is not eligible", "code": 9}),
    )
    .create();

    match client(&server).marketplace().get_payout("missing") {
        Err(TapsilatError::ApiError {
            status_code,
            message,
            code,
            body,
            ..
        }) => {
            assert_eq!(status_code, 424);
            assert_eq!(message, "payout is not eligible");
            assert_eq!(code.as_deref(), Some("9"));
            assert!(body.contains("not eligible"));
        }
        other => panic!("unexpected result {other:?}"),
    }
}

#[test]
fn an_html_page_is_reported_as_an_api_error() {
    let mut server = Server::new();
    let _mock = server
        .mock("GET", "/api/v1/order/ref-1/status")
        .with_status(200)
        .with_header("content-type", "text/html; charset=utf-8")
        .with_body("<!doctype html><html></html>")
        .create();

    match client(&server).get_order_status("ref-1") {
        Err(TapsilatError::ApiError {
            status_code,
            message,
            body,
            ..
        }) => {
            assert_eq!(status_code, 200);
            assert!(message.contains("unexpected non-JSON response"));
            assert!(body.starts_with("<!doctype html>"));
        }
        other => panic!("unexpected result {other:?}"),
    }
}

#[test]
fn health_check_returns_the_plain_text_answer() {
    let mut server = Server::new();
    let _mock = server
        .mock("GET", "/api/v1/health")
        .with_status(200)
        .with_header("content-type", "text/plain; charset=utf-8")
        .with_body("OK")
        .create();

    assert_eq!(client(&server).health_check().unwrap(), json!("OK"));
}
