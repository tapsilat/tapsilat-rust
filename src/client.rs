//! HTTP client implementation for the Tapsilat API.
//!
//! This module contains the main [`TapsilatClient`] which handles all HTTP communication
//! with the Tapsilat API, including authentication, request/response processing, and error handling.

use crate::config::Config;
use crate::error::{Result, TapsilatError};
use crate::modules::{
    InstallmentModule, MarketplaceModule, OrderModule, OrganizationModule, PaymentModule,
    SubmerchantModule, SubscriptionModule, TokenizationModule, VposModule, WebhookModule,
};
use crate::types::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const USER_AGENT: &str = concat!("tapsilat-rust/", env!("CARGO_PKG_VERSION"));

/// Main client for interacting with the Tapsilat API.
///
/// The `TapsilatClient` provides both direct methods for API operations and modular
/// interfaces through accessor methods like `orders()`, `payments()`, etc.
#[derive(Clone)]
pub struct TapsilatClient {
    config: Config,
    http_client: ureq::Agent,
    currency_ids: Arc<Mutex<Option<HashMap<String, String>>>>,
}

struct RawResponse {
    status: u16,
    content_type: String,
    body: String,
}

impl TapsilatClient {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let mut builder = ureq::Agent::config_builder().http_status_as_error(false);
        if config.timeout > 0 {
            builder = builder.timeout_global(Some(Duration::from_secs(config.timeout)));
        }
        let http_client: ureq::Agent = builder.build().into();

        Ok(Self {
            config,
            http_client,
            currency_ids: Arc::new(Mutex::new(None)),
        })
    }

    pub fn from_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = Config::new(api_key);
        Self::new(config)
    }

    /// Access to payment operations
    pub fn payments(&self) -> PaymentModule {
        PaymentModule::new(Arc::new(self.clone()))
    }

    /// Access to order operations
    pub fn orders(&self) -> OrderModule {
        OrderModule::new(Arc::new(self.clone()))
    }

    /// Access to organization operations
    pub fn organization(&self) -> OrganizationModule {
        OrganizationModule::new(Arc::new(self.clone()))
    }

    /// Access to card installment operations
    pub fn installments(&self) -> InstallmentModule {
        InstallmentModule::new(Arc::new(self.clone()))
    }

    /// Access to subscription operations
    pub fn subscriptions(&self) -> SubscriptionModule {
        SubscriptionModule::new(Arc::new(self.clone()))
    }

    /// Access to submerchant operations
    pub fn submerchants(&self) -> SubmerchantModule {
        SubmerchantModule::new(Arc::new(self.clone()))
    }

    /// Access to virtual POS operations
    pub fn vpos(&self) -> VposModule {
        VposModule::new(Arc::new(self.clone()))
    }

    /// Access to saved card (tokenization) operations
    pub fn tokenization(&self) -> TokenizationModule {
        TokenizationModule::new(Arc::new(self.clone()))
    }

    /// Access to marketplace seller, payout and policy operations (API v2)
    pub fn marketplace(&self) -> MarketplaceModule {
        MarketplaceModule::new(Arc::new(self.clone()))
    }

    /// Access to webhook operations
    pub fn webhooks() -> &'static WebhookModule {
        &WebhookModule
    }

    /// Returns the configured base URL for another API version: a trailing `/api/vN` is
    /// replaced with `/api/{version}`, otherwise `/api/{version}` is appended.
    pub fn versioned_base_url(&self, version: &str) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        if let Some((head, last)) = base.rsplit_once('/') {
            let is_version = last.len() > 1
                && last.starts_with('v')
                && last[1..].chars().all(|c| c.is_ascii_digit());
            if is_version && head.ends_with("/api") {
                return format!("{}/{}", head, version);
            }
        }
        format!("{}/api/{}", base, version)
    }

    /// Returns the organization currency UUID for `reference`. A UUID is returned unchanged;
    /// a unit such as "TRY" is looked up in the organization's currencies, which are cached.
    /// Most endpoints accept units directly, so this is only needed where a UUID is required.
    pub fn resolve_currency_id(&self, reference: &str) -> Result<String> {
        let trimmed = reference.trim();
        if trimmed.is_empty() {
            return Err(TapsilatError::ValidationError(
                "currency_id is required".to_string(),
            ));
        }
        if is_uuid(trimmed) {
            return Ok(trimmed.to_string());
        }

        let cached = self
            .currency_ids
            .lock()
            .ok()
            .and_then(|cache| cache.clone());
        let ids = match cached {
            Some(ids) => ids,
            None => {
                let response: OrganizationCurrenciesResponse =
                    self.request::<(), _>("GET", "organization/currencies", None)?;
                let ids: HashMap<String, String> = response
                    .currencies
                    .into_iter()
                    .filter(|c| !c.id.trim().is_empty() && !c.currency_unit.trim().is_empty())
                    .map(|c| (c.currency_unit.trim().to_uppercase(), c.id))
                    .collect();
                if let Ok(mut cache) = self.currency_ids.lock() {
                    *cache = Some(ids.clone());
                }
                ids
            }
        };

        ids.get(&trimmed.to_uppercase()).cloned().ok_or_else(|| {
            TapsilatError::ValidationError(format!(
                "unknown currency reference {:?}; use a currency UUID or organization currency unit",
                reference
            ))
        })
    }

    pub(crate) fn resolve_optional_currency_id(&self, reference: &str) -> Result<String> {
        if reference.trim().is_empty() {
            return Ok(reference.to_string());
        }
        self.resolve_currency_id(reference)
    }

    pub(crate) fn invalidate_currency_cache(&self) {
        if let Ok(mut cache) = self.currency_ids.lock() {
            *cache = None;
        }
    }

    // Direct Operations (Routing to modules for backward/direct compatibility mostly, or implementing essentials)

    /// Creates an order. The buyer's GSM number is cleaned first, and the checkout URL is
    /// filled from the created order when it can be read.
    pub fn create_order(&self, request: CreateOrderRequest) -> Result<CreateOrderResponse> {
        self.orders().create(request)
    }

    pub fn get_order(&self, reference_id: &str) -> Result<Order> {
        self.orders().get(reference_id)
    }

    /// Returns the order with the given conversation ID. The server fills a subset of the
    /// order detail fields for this lookup.
    pub fn get_order_by_conversation_id(&self, conversation_id: &str) -> Result<OrderDetail> {
        self.orders().get_by_conversation_id(conversation_id)
    }

    pub fn cancel_order(&self, reference_id: &str) -> Result<Value> {
        self.orders().cancel(reference_id)
    }

    pub fn refund_order(&self, request: RefundOrderRequest) -> Result<Value> {
        self.orders().refund(request)
    }

    pub fn refund_all_order(&self, reference_id: &str) -> Result<Value> {
        self.orders().refund_all(reference_id)
    }

    // Updated signature to match Python's get_order_list
    pub fn get_order_list(
        &self,
        page: u32,
        per_page: u32,
        buyer_id: Option<String>,
    ) -> Result<Value> {
        self.orders().list(page, per_page, buyer_id)
    }

    /// Lists orders with every filter the order list endpoint accepts.
    pub fn get_order_list_with_filter(&self, filter: &OrderListFilter) -> Result<Value> {
        self.orders().list_with_filter(filter)
    }

    pub fn get_order_submerchants(&self, page: u32, per_page: u32) -> Result<Value> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .append_to("order/submerchants");
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_order_status(&self, reference_id: &str) -> Result<Value> {
        self.orders().get_status(reference_id)
    }

    /// Returns the order's transactions as a JSON array, or null when there are none.
    pub fn get_order_transactions(&self, reference_id: &str) -> Result<Value> {
        let endpoint = format!("order/{}/transactions", path_escape(reference_id));
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_order_payment_details(
        &self,
        reference_id: &str,
        conversation_id: Option<String>,
    ) -> Result<Value> {
        if let Some(cid) = conversation_id {
            let endpoint = "order/payment-details";
            let payload = serde_json::json!({
                "conversation_id": cid,
                "reference_id": reference_id
            });
            self.make_request("POST", endpoint, Some(&payload))
        } else {
            let endpoint = format!("order/{}/payment-details", path_escape(reference_id));
            self.make_request::<()>("GET", &endpoint, None)
        }
    }

    /// Returns the payment details of the order with the given conversation ID.
    pub fn get_order_payment_details_by_conversation_id(
        &self,
        conversation_id: &str,
    ) -> Result<Value> {
        self.orders()
            .get_payment_details_by_conversation_id(conversation_id)
    }

    /// Lists the payments made against an order, with commission totals.
    pub fn get_order_payments(
        &self,
        request: GetOrderPaymentsRequest,
    ) -> Result<GetOrderPaymentsResponse> {
        self.orders().get_payments(request)
    }

    /// Charges an order, for example with its saved card.
    pub fn charge_order(&self, request: OrderChargeRequest) -> Result<OrderChargeResponse> {
        self.orders().charge(request)
    }

    /// Returns the installment options and commission rates for a card BIN and amount.
    pub fn get_card_installments(
        &self,
        request: &CardInstallmentsRequest,
    ) -> Result<CardInstallmentsResponse> {
        self.installments().get_card_installments(request)
    }

    /// Returns installment options grouped by bank card program.
    pub fn get_installment_table(
        &self,
        request: &InstallmentTableRequest,
    ) -> Result<InstallmentTableResponse> {
        self.installments().get_installment_table(request)
    }

    pub fn get_checkout_url(&self, reference_id: &str) -> Result<String> {
        self.orders().get_checkout_url(reference_id)
    }

    pub fn order_manual_callback(
        &self,
        reference_id: &str,
        conversation_id: Option<String>,
    ) -> Result<Value> {
        self.orders().manual_callback(reference_id, conversation_id)
    }

    pub fn get_system_order_statuses(&self) -> Result<Value> {
        self.make_request::<()>("GET", "system/order-statuses", None)
    }

    pub fn get_organization_settings(&self) -> Result<Value> {
        self.organization().get_settings()
    }

    pub fn get_organization_callback(&self) -> Result<Value> {
        self.organization().get_callback()
    }

    pub fn update_organization_callback(&self, request: CallbackURLDTO) -> Result<Value> {
        self.organization().update_callback(request)
    }

    pub fn create_organization_business(&self, request: OrgCreateBusinessRequest) -> Result<Value> {
        self.organization().create_business(request)
    }

    pub fn get_organization_currencies(&self) -> Result<Value> {
        self.organization().get_currencies()
    }

    /// Lists the currencies the organization can add.
    pub fn list_organization_currency_presets(
        &self,
    ) -> Result<OrganizationCurrencyPresetsResponse> {
        self.organization().list_currency_presets()
    }

    /// Adds a currency to the organization.
    pub fn create_organization_currency(
        &self,
        currency_code: &str,
    ) -> Result<CreateOrganizationCurrencyResponse> {
        self.organization().create_currency(currency_code)
    }

    #[deprecated(note = "the API has no organization user limit route; this request always fails")]
    #[allow(deprecated)]
    pub fn get_organization_limit_user(&self, user_id: &str) -> Result<Value> {
        self.organization().get_limit_user(user_id)
    }

    #[deprecated(note = "the API has no organization user limit route; this request always fails")]
    #[allow(deprecated)]
    pub fn set_organization_limit_user(&self, request: SetLimitUserRequest) -> Result<Value> {
        self.organization().set_limit_user(request)
    }

    #[deprecated(
        note = "use get_organization_limits_by_id; the limits route needs an organization ID"
    )]
    #[allow(deprecated)]
    pub fn get_organization_limits(&self) -> Result<Value> {
        self.organization().get_limits()
    }

    /// Returns the limits of the given organization.
    pub fn get_organization_limits_by_id(&self, organization_id: &str) -> Result<Value> {
        self.organization().get_limits_by_id(organization_id)
    }

    pub fn list_organization_vpos(&self, currency_id: &str) -> Result<Value> {
        self.organization().list_vpos(currency_id)
    }

    pub fn get_organization_meta(&self, name: &str) -> Result<Value> {
        self.organization().get_meta(name)
    }

    pub fn get_organization_scopes(&self) -> Result<Value> {
        self.organization().get_scopes()
    }

    pub fn get_organization_suborganizations(&self, page: u32, per_page: u32) -> Result<Value> {
        self.organization().get_suborganizations(page, per_page)
    }

    pub fn create_organization_user(&self, request: OrgCreateUserReq) -> Result<Value> {
        self.organization().create_user(request)
    }

    /// Creates an access token for an organization user.
    pub fn create_organization_user_token(
        &self,
        request: OrgUserTokenCreateRequest,
    ) -> Result<OrgUserTokenCreateResponse> {
        self.organization().create_user_token(request)
    }

    pub fn verify_organization_user(&self, user_id: &str) -> Result<Value> {
        self.organization().verify_user(user_id)
    }

    pub fn verify_organization_user_mobile(&self, user_id: &str) -> Result<Value> {
        self.organization().verify_user_mobile(user_id)
    }

    pub fn order_add_basket_item(&self, request: Value) -> Result<Value> {
        self.orders().add_basket_item(request)
    }

    pub fn order_remove_basket_item(&self, order_id: &str, item_id: &str) -> Result<Value> {
        self.orders().remove_basket_item(order_id, item_id)
    }

    pub fn order_update_basket_item(&self, request: Value) -> Result<Value> {
        self.orders().update_basket_item(request)
    }

    /// Checks that the API is reachable; the server answers with the text "OK".
    pub fn health_check(&self) -> Result<Value> {
        let url = self.endpoint_url(None, "health");
        let raw = self.send_raw("GET", &url, None)?;
        if raw.status >= 400 {
            return Err(TapsilatError::from_api_response(raw.status, &raw.body));
        }
        Ok(serde_json::from_str(&raw.body).unwrap_or(Value::String(raw.body)))
    }

    // Order Term Operations (Delegated to module or direct)

    pub fn create_order_term(&self, request: OrderPaymentTermCreateDTO) -> Result<Value> {
        self.orders().create_term(request)
    }

    pub fn update_order_term(&self, request: OrderPaymentTermUpdateDTO) -> Result<Value> {
        self.orders().update_term(request)
    }

    pub fn delete_order_term(&self, order_id: &str, term_reference_id: &str) -> Result<Value> {
        self.orders().delete_term(order_id, term_reference_id)
    }

    pub fn refund_order_term(&self, request: OrderTermRefundRequest) -> Result<Value> {
        self.orders().refund_term(request)
    }

    pub fn get_order_term(&self, term_reference_id: &str) -> Result<Value> {
        self.orders().get_term(term_reference_id)
    }

    pub fn order_terminate(&self, reference_id: &str) -> Result<Value> {
        self.orders().terminate(reference_id)
    }

    #[deprecated(
        note = "the API has no term terminate route; this request always fails. Use order_terminate or delete_order_term"
    )]
    #[allow(deprecated)]
    pub fn terminate_order_term(
        &self,
        term_reference_id: &str,
        reason: Option<String>,
    ) -> Result<Value> {
        self.orders().terminate_term(term_reference_id, reason)
    }

    pub fn order_accounting(&self, request: OrderAccountingRequest) -> Result<Value> {
        self.orders().accounting(request)
    }

    pub fn order_postauth(&self, request: OrderPostAuthRequest) -> Result<Value> {
        self.orders().postauth(request)
    }

    pub fn order_related_update(
        &self,
        reference_id: &str,
        related_reference_id: &str,
    ) -> Result<Value> {
        self.orders()
            .related_update(reference_id, related_reference_id)
    }

    // Webhook Operations

    /// Verifies the `Tapsilat-Signature` header of a callback; see
    /// [`WebhookModule::verify_webhook_with`].
    pub fn verify_webhook(&self, payload: &str, signature: &str, secret: &str) -> Result<bool> {
        WebhookModule::verify_webhook(payload, signature, secret)
    }

    // Subscription Operations

    pub fn get_subscription(&self, request: SubscriptionGetRequest) -> Result<SubscriptionDetail> {
        self.subscriptions().get(request)
    }

    pub fn cancel_subscription(&self, request: SubscriptionCancelRequest) -> Result<Value> {
        self.subscriptions().cancel(request)
    }

    pub fn create_subscription(
        &self,
        request: SubscriptionCreateRequest,
    ) -> Result<SubscriptionCreateResponse> {
        self.subscriptions().create(request)
    }

    pub fn list_subscriptions(&self, page: u32, per_page: u32) -> Result<Value> {
        self.subscriptions().list(page, per_page)
    }

    pub fn redirect_subscription(
        &self,
        request: SubscriptionRedirectRequest,
    ) -> Result<SubscriptionRedirectResponse> {
        self.subscriptions().redirect(request)
    }

    /// Sends a request to the configured (v1) API and returns the decoded JSON body,
    /// or `Value::Null` for an empty body.
    pub(crate) fn make_request<T>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<Value>
    where
        T: Serialize,
    {
        self.send_json(method, None, endpoint, body)
    }

    /// Sends a request to the given API version, such as "v2".
    pub(crate) fn make_versioned_request<T>(
        &self,
        version: &str,
        method: &str,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<Value>
    where
        T: Serialize,
    {
        self.send_json(method, Some(version), endpoint, body)
    }

    /// Sends a request to the configured (v1) API and decodes the response into `R`.
    pub(crate) fn request<T, R>(&self, method: &str, endpoint: &str, body: Option<&T>) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let value = self.make_request(method, endpoint, body)?;
        Ok(serde_json::from_value(value)?)
    }

    /// Sends a request to API v2 and decodes the response into `R`.
    pub(crate) fn request_v2<T, R>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let value = self.make_versioned_request("v2", method, endpoint, body)?;
        Ok(serde_json::from_value(value)?)
    }

    fn endpoint_url(&self, version: Option<&str>, endpoint: &str) -> String {
        let base = match version {
            Some(version) => self.versioned_base_url(version),
            None => self.config.base_url.trim_end_matches('/').to_string(),
        };
        format!("{}/{}", base, endpoint.trim_start_matches('/'))
    }

    fn send_json<T>(
        &self,
        method: &str,
        version: Option<&str>,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<Value>
    where
        T: Serialize,
    {
        let url = self.endpoint_url(version, endpoint);
        let payload = match body {
            Some(body) => Some(serde_json::to_vec(body)?),
            None => None,
        };
        let raw = self.send_raw(method, &url, payload)?;

        if raw.status >= 400 {
            return Err(TapsilatError::from_api_response(raw.status, &raw.body));
        }
        // A deployment serving the panel UI answers unknown routes with its HTML page and status 200
        if raw.content_type.to_ascii_lowercase().contains("text/html")
            || raw.body.trim_start().starts_with('<')
        {
            return Err(TapsilatError::unexpected_response(
                raw.status,
                &raw.content_type,
                &raw.body,
            ));
        }
        if raw.body.trim().is_empty() {
            return Ok(Value::Null);
        }
        Ok(serde_json::from_str(&raw.body)?)
    }

    fn send_raw(&self, method: &str, url: &str, body: Option<Vec<u8>>) -> Result<RawResponse> {
        let authorization = format!("Bearer {}", self.config.api_key);
        let agent = &self.http_client;

        macro_rules! with_headers {
            ($builder:expr) => {
                $builder
                    .header("Authorization", &authorization)
                    .header("Accept", "application/json")
                    .header("User-Agent", USER_AGENT)
            };
        }
        macro_rules! send_body {
            ($builder:expr) => {
                match body {
                    Some(data) => with_headers!($builder)
                        .header("Content-Type", "application/json")
                        .send(&data[..]),
                    None => with_headers!($builder).send_empty(),
                }
            };
        }

        let result = match method.to_ascii_uppercase().as_str() {
            "GET" => with_headers!(agent.get(url)).call(),
            "POST" => send_body!(agent.post(url)),
            "PUT" => send_body!(agent.put(url)),
            "PATCH" => send_body!(agent.patch(url)),
            "DELETE" => match body {
                Some(data) => with_headers!(agent.delete(url).force_send_body())
                    .header("Content-Type", "application/json")
                    .send(&data[..]),
                None => with_headers!(agent.delete(url)).call(),
            },
            _ => {
                return Err(TapsilatError::ConfigError(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        let mut response = result?;
        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let body = response.body_mut().read_to_string()?;

        Ok(RawResponse {
            status,
            content_type,
            body,
        })
    }
}

/// Escapes a value for use as a single URL path segment.
pub(crate) fn path_escape(value: &str) -> String {
    percent_encode(value)
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char)
            }
            _ => encoded.push_str(&format!("%{:02X}", byte)),
        }
    }
    encoded
}

fn is_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && bytes.iter().enumerate().all(|(i, b)| match i {
            8 | 13 | 18 | 23 => *b == b'-',
            _ => b.is_ascii_hexdigit(),
        })
}

/// Builds a URL query string, encoding every value and keeping insertion order.
#[derive(Debug, Default, Clone)]
pub(crate) struct Query(Vec<(String, String)>);

impl Query {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds a parameter.
    pub(crate) fn set(mut self, key: &str, value: impl ToString) -> Self {
        self.0.push((key.to_string(), value.to_string()));
        self
    }

    /// Adds a parameter when the value is present and not empty.
    pub(crate) fn opt<V: ToString>(self, key: &str, value: Option<V>) -> Self {
        match value.map(|v| v.to_string()) {
            Some(value) if !value.is_empty() => self.set(key, value),
            _ => self,
        }
    }

    /// Returns `path` with the encoded query appended.
    pub(crate) fn append_to(&self, path: &str) -> String {
        if self.0.is_empty() {
            return path.to_string();
        }
        let query: Vec<String> = self
            .0
            .iter()
            .map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
            .collect();
        format!("{}?{}", path, query.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versioned_base_url_replaces_or_appends_the_version() {
        let client =
            |url: &str| TapsilatClient::new(Config::new("key").with_base_url(url)).unwrap();
        assert_eq!(
            client("https://panel.tapsilat.dev/api/v1").versioned_base_url("v2"),
            "https://panel.tapsilat.dev/api/v2"
        );
        assert_eq!(
            client("https://panel.tapsilat.dev/api/v1/").versioned_base_url("v2"),
            "https://panel.tapsilat.dev/api/v2"
        );
        assert_eq!(
            client("http://127.0.0.1:8080").versioned_base_url("v2"),
            "http://127.0.0.1:8080/api/v2"
        );
        assert_eq!(
            client("https://example.com/proxy").versioned_base_url("v2"),
            "https://example.com/proxy/api/v2"
        );
    }

    #[test]
    fn query_and_path_values_are_escaped() {
        assert_eq!(path_escape("a/b c"), "a%2Fb%20c");
        assert_eq!(
            Query::new()
                .set("page", 1)
                .opt("buyer_id", Some("x&y=z"))
                .opt::<&str>("empty", Some(""))
                .opt::<&str>("none", None)
                .append_to("order/list"),
            "order/list?page=1&buyer_id=x%26y%3Dz"
        );
    }

    #[test]
    fn uuid_detection() {
        assert!(is_uuid("3fa85f64-5717-4562-b3fc-2c963f66afa6"));
        assert!(!is_uuid("TRY"));
        assert!(!is_uuid("3fa85f64-5717-4562-b3fc-2c963f66afaz"));
    }
}
