//! Tapsilat Elements session requests and responses, using a mock server.

use mockito::{Matcher, Server, ServerGuard};
use serde_json::json;
use tapsilat::types::*;
use tapsilat::{Config, TapsilatClient, TapsilatError};

fn client(server: &ServerGuard) -> TapsilatClient {
    TapsilatClient::new(Config::new("token_123").with_base_url(format!("{}/api/v1", server.url())))
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
        .match_header("authorization", "Bearer token_123")
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_header("cache-control", "no-store")
        .with_body(body.to_string())
}

#[test]
fn create_pay_session_sends_only_set_fields_and_returns_the_secret() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "POST",
        "/api/v1/elements/sessions",
        200,
        json!({
            "id": "es_test_8Kq2nV0rX4",
            "purpose": "pay",
            "status": "requires_payment_method",
            "order_reference_id": "order-ref-1",
            "attempts": 0,
            "max_attempts": 3,
            "created_at": "2026-10-22T09:00:00Z",
            "expires_at": "2026-10-22T09:30:00Z",
            "client_secret": "es_test_8Kq2nV0rX4_secret_q8Zr0bM1",
            "elements_base_url": "https://checkout.tapsilat.dev"
        }),
    )
    .match_header("content-type", "application/json")
    .match_body(Matcher::Json(json!({
        "purpose": "pay",
        "order_reference_id": "order-ref-1",
        "save_card": true
    })))
    .create();

    let created = client(&server)
        .create_elements_session(ElementsSessionCreateRequest {
            purpose: ElementsSession::PURPOSE_PAY.to_string(),
            order_reference_id: Some("order-ref-1".to_string()),
            save_card: true,
            ..Default::default()
        })
        .unwrap();
    mock.assert();

    let session = &created.session;
    assert_eq!(session.id.as_deref(), Some("es_test_8Kq2nV0rX4"));
    assert_eq!(
        session.purpose.as_deref(),
        Some(ElementsSession::PURPOSE_PAY)
    );
    assert_eq!(
        session.status.as_deref(),
        Some(ElementsSession::STATUS_REQUIRES_PAYMENT_METHOD)
    );
    assert_eq!(session.order_reference_id.as_deref(), Some("order-ref-1"));
    assert_eq!(session.attempts, Some(0));
    assert_eq!(session.max_attempts, Some(3));
    assert_eq!(session.created_at.as_deref(), Some("2026-10-22T09:00:00Z"));
    assert_eq!(session.expires_at.as_deref(), Some("2026-10-22T09:30:00Z"));
    assert_eq!(session.completed_at, None);
    assert_eq!(session.card, None);
    assert_eq!(session.last_error, None);
    assert_eq!(created.client_secret, "es_test_8Kq2nV0rX4_secret_q8Zr0bM1");
    assert_eq!(
        created.elements_base_url.as_deref(),
        Some("https://checkout.tapsilat.dev")
    );

    let printed = format!("{:?}", created);
    assert!(!printed.contains("es_test_8Kq2nV0rX4_secret_q8Zr0bM1"));
    assert!(printed.contains(r#"client_secret: "[redacted]""#));
    assert!(serde_json::to_string(&created)
        .unwrap()
        .contains(r#""client_secret":"es_test_8Kq2nV0rX4_secret_q8Zr0bM1""#));
}

#[test]
fn create_tokenize_session_sends_user_and_mode() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "POST",
        "/api/v1/elements/sessions",
        200,
        json!({
            "id": "es_test_1",
            "purpose": "tokenize",
            "status": "requires_payment_method",
            "user_id": "user_123",
            "client_secret": "es_test_1_secret_x"
        }),
    )
    .match_body(Matcher::Json(json!({
        "purpose": "tokenize",
        "user_id": "user_123",
        "user_email": "john@doe.com",
        "mode": 3
    })))
    .create();

    let created = client(&server)
        .elements()
        .create_session(ElementsSessionCreateRequest {
            purpose: ElementsSession::PURPOSE_TOKENIZE.to_string(),
            user_id: Some("user_123".to_string()),
            user_email: Some("john@doe.com".to_string()),
            mode: Some(ElementsSessionCreateRequest::MODE_NON_SECURE),
            ..Default::default()
        })
        .unwrap();
    mock.assert();

    assert_eq!(created.session.user_id.as_deref(), Some("user_123"));
    assert_eq!(created.client_secret, "es_test_1_secret_x");
    assert_eq!(created.elements_base_url, None);
}

#[test]
fn get_session_decodes_card_and_last_error() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v1/elements/sessions/es_test_8Kq2nV0rX4",
        200,
        json!({
            "id": "es_test_8Kq2nV0rX4",
            "purpose": "pay",
            "status": "failed",
            "order_reference_id": "order-ref-1",
            "attempts": 3,
            "max_attempts": 3,
            "completed_at": "2026-10-22T09:12:00Z",
            "card": {"id": "card-1", "brand": "visa", "last4": "4242", "exp_month": 12, "exp_year": 2030},
            "last_error": {"code": "card_declined", "message": "The card was declined.", "retryable": false}
        }),
    )
    .match_body("")
    .create();

    let session = client(&server)
        .get_elements_session("es_test_8Kq2nV0rX4")
        .unwrap();
    mock.assert();

    assert_eq!(
        session.status.as_deref(),
        Some(ElementsSession::STATUS_FAILED)
    );
    assert_eq!(session.attempts, Some(3));
    assert_eq!(
        session.completed_at.as_deref(),
        Some("2026-10-22T09:12:00Z")
    );
    assert_eq!(
        session.card,
        Some(ElementsCardSummary {
            id: Some("card-1".to_string()),
            brand: Some(ElementsCardSummary::BRAND_VISA.to_string()),
            last4: Some("4242".to_string()),
            exp_month: Some(12),
            exp_year: Some(2030),
        })
    );
    assert_eq!(
        session.last_error,
        Some(ElementsError {
            code: Some("card_declined".to_string()),
            message: Some("The card was declined.".to_string()),
            retryable: Some(false),
        })
    );
}

#[test]
fn get_session_reads_null_card_and_error_and_unknown_values() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v1/elements/sessions/es_mock_1",
        200,
        json!({
            "id": "es_mock_1",
            "purpose": "pay",
            "status": "a_status_added_later",
            "card": null,
            "last_error": null,
            "a_field_added_later": {"nested": true}
        }),
    )
    .create();

    let session = client(&server).get_elements_session("es_mock_1").unwrap();
    mock.assert();
    assert_eq!(session.status.as_deref(), Some("a_status_added_later"));
    assert_eq!(session.card, None);
    assert_eq!(session.last_error, None);
}

#[test]
fn get_session_escapes_the_id() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v1/elements/sessions/es_test_1%2F..%2Fx",
        200,
        json!({"id": "es_test_1"}),
    )
    .create();

    client(&server)
        .get_elements_session("es_test_1/../x")
        .unwrap();
    mock.assert();
}

#[test]
fn cancel_session_posts_without_a_body() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "POST",
        "/api/v1/elements/sessions/es_test_8Kq2nV0rX4/cancel",
        200,
        json!({"id": "es_test_8Kq2nV0rX4", "purpose": "pay", "status": "canceled", "attempts": 0, "max_attempts": 3}),
    )
    .match_header("content-type", Matcher::Missing)
    .match_body("")
    .create();

    let session = client(&server)
        .cancel_elements_session("es_test_8Kq2nV0rX4")
        .unwrap();
    mock.assert();
    assert_eq!(
        session.status.as_deref(),
        Some(ElementsSession::STATUS_CANCELED)
    );
}

/// Checks that a nested `{"error": {...}}` body surfaces on `TapsilatError::ApiError`.
fn assert_api_error(
    error: TapsilatError,
    want_status: u16,
    want_code: &str,
    want_message: &str,
    want_retryable: bool,
) {
    assert!(
        error
            .to_string()
            .contains(&format!("[{}] {}", want_code, want_message)),
        "{error}"
    );
    match error {
        TapsilatError::ApiError {
            status_code,
            code,
            message,
            retryable,
            ..
        } => {
            assert_eq!(status_code, want_status);
            assert_eq!(code.as_deref(), Some(want_code));
            assert_eq!(message, want_message);
            assert_eq!(retryable, want_retryable);
        }
        other => panic!("unexpected error {other:?}"),
    }
}

#[test]
fn create_session_not_enabled_error() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "POST",
        "/api/v1/elements/sessions",
        403,
        json!({"error": {"code": "elements_not_enabled", "message": "Elements is not enabled for this organization"}}),
    )
    .create();

    let error = client(&server)
        .create_elements_session(ElementsSessionCreateRequest {
            purpose: ElementsSession::PURPOSE_PAY.to_string(),
            ..Default::default()
        })
        .unwrap_err();
    mock.assert();
    assert_api_error(
        error,
        403,
        "elements_not_enabled",
        "Elements is not enabled for this organization",
        false,
    );
}

#[test]
fn get_session_not_found_error() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "GET",
        "/api/v1/elements/sessions/es_test_missing",
        404,
        json!({"error": {"code": "session_not_found", "message": "session not found"}}),
    )
    .create();

    let error = client(&server)
        .get_elements_session("es_test_missing")
        .unwrap_err();
    mock.assert();
    assert_api_error(error, 404, "session_not_found", "session not found", false);
}

#[test]
fn cancel_session_unavailable_error_is_retryable() {
    let mut server = Server::new();
    let mock = json_mock(
        &mut server,
        "POST",
        "/api/v1/elements/sessions/es_test_1/cancel",
        503,
        json!({"error": {"code": "unavailable", "message": "try again", "retryable": true}}),
    )
    .create();

    let error = client(&server)
        .cancel_elements_session("es_test_1")
        .unwrap_err();
    mock.assert();
    assert_api_error(error, 503, "unavailable", "try again", true);
}
