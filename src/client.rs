//! HTTP client implementation for the Tapsilat API.
//!
//! This module contains the main [`TapsilatClient`] which handles all HTTP communication
//! with the Tapsilat API, including authentication, request/response processing, and error handling.

use crate::config::Config;
use crate::error::{Result, TapsilatError};
use crate::modules::{InstallmentModule, OrderModule, PaymentModule, SubscriptionModule, WebhookModule};
use crate::types::*;
use serde_json::Value;

/// Main client for interacting with the Tapsilat API.
///
/// The `TapsilatClient` provides both direct methods for API operations and modular
/// interfaces through accessor methods like `orders()`, `payments()`, etc.
#[derive(Clone)]
pub struct TapsilatClient {
    config: Config,
    http_client: ureq::Agent,
}

impl TapsilatClient {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let http_client = ureq::Agent::new_with_defaults();

        Ok(Self {
            config,
            http_client,
        })
    }

    pub fn from_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = Config::new(api_key);
        Self::new(config)
    }
    
    /// Access to payment operations
    pub fn payments(&self) -> PaymentModule {
        PaymentModule::new(std::sync::Arc::new(self.clone()))
    }

    /// Access to order operations
    pub fn orders(&self) -> OrderModule {
        OrderModule::new(std::sync::Arc::new(self.clone()))
    }

    /// Access to installment operations
    pub fn installments(&self) -> InstallmentModule {
        InstallmentModule::new(std::sync::Arc::new(self.clone()))
    }

    /// Access to subscription operations
    pub fn subscriptions(&self) -> SubscriptionModule {
        SubscriptionModule::new(std::sync::Arc::new(self.clone()))
    }

    /// Access to webhook operations
    pub fn webhooks() -> &'static WebhookModule {
        &WebhookModule
    }

    // Direct Operations (Routing to modules for backward/direct compatibility mostly, or implementing essentials)

    pub fn create_order(&self, request: CreateOrderRequest) -> Result<CreateOrderResponse> {
        self.orders().create(request)
    }

    pub fn get_order(&self, reference_id: &str) -> Result<Order> {
        self.orders().get(reference_id)
    }
    
    pub fn get_order_by_conversation_id(&self, conversation_id: &str) -> Result<OrderResponse> {
        let endpoint = format!("order/conversation/{}", conversation_id);
        let response = self.make_request::<()>("GET", &endpoint, None)?;
        serde_json::from_value(response).map_err(|e| {
             TapsilatError::ConfigError(format!("Failed to parse order response: {}", e))
        })
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
    pub fn get_order_list(&self, page: u32, per_page: u32, buyer_id: Option<String>) -> Result<Value> {
        self.orders().list(page, per_page, buyer_id)
    }
    
    pub fn get_order_submerchants(&self, page: u32, per_page: u32) -> Result<Value> {
        let mut endpoint = "order/submerchants".to_string();
        endpoint = format!("{}?page={}&per_page={}", endpoint, page, per_page);
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_order_status(&self, reference_id: &str) -> Result<Value> {
        self.orders().get_status(reference_id)
    }

    pub fn get_order_transactions(&self, reference_id: &str) -> Result<Value> {
        let endpoint = format!("order/{}/transactions", reference_id);
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_order_payment_details(&self, reference_id: &str, conversation_id: Option<String>) -> Result<Value> {
        if let Some(cid) = conversation_id {
            let endpoint = "order/payment-details";
             let payload = serde_json::json!({
                 "conversation_id": cid,
                 "reference_id": reference_id
             });
            self.make_request("POST", endpoint, Some(&payload))
        } else {
             let endpoint = format!("order/{}/payment-details", reference_id);
             self.make_request::<()>("GET", &endpoint, None)
        }
    }

    pub fn get_checkout_url(&self, reference_id: &str) -> Result<String> {
        self.orders().get_checkout_url(reference_id)
    }

    pub fn order_manual_callback(&self, reference_id: &str, conversation_id: Option<String>) -> Result<Value> {
        self.orders().manual_callback(reference_id, conversation_id)
    }
    
    pub fn get_system_order_statuses(&self) -> Result<Value> {
        self.make_request::<()>("GET", "system/order-statuses", None)
    }
    
    pub fn get_organization_settings(&self) -> Result<Value> {
        self.make_request::<()>("GET", "organization/settings", None)
    }
    
    pub fn health_check(&self) -> Result<Value> {
        self.make_request::<()>("GET", "health", None)
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
        let endpoint = format!("order/term/{}", term_reference_id);
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn order_terminate(&self, reference_id: &str) -> Result<Value> {
        self.orders().terminate(reference_id)
    }

    pub fn terminate_order_term(&self, term_reference_id: &str, reason: Option<String>) -> Result<Value> {
        self.orders().terminate_term(term_reference_id, reason)
    }
    
    pub fn order_accounting(&self, request: OrderAccountingRequest) -> Result<Value> {
        self.orders().accounting(request)
    }
    
    pub fn order_postauth(&self, request: OrderPostAuthRequest) -> Result<Value> {
        self.orders().postauth(request)
    }
    
    pub fn order_related_update(&self, reference_id: &str, related_reference_id: &str) -> Result<Value> {
        self.orders().related_update(reference_id, related_reference_id)
    }

    // Webhook Operations
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

    pub fn create_subscription(&self, request: SubscriptionCreateRequest) -> Result<SubscriptionCreateResponse> {
        self.subscriptions().create(request)
    }

    pub fn list_subscriptions(&self, page: u32, per_page: u32) -> Result<Value> {
        self.subscriptions().list(page, per_page)
    }

    pub fn redirect_subscription(&self, request: SubscriptionRedirectRequest) -> Result<SubscriptionRedirectResponse> {
        self.subscriptions().redirect(request)
    }

    pub(crate) fn make_request<T>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<serde_json::Value>
    where
        T: serde::Serialize,
    {
        let url = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        );

        // Debug logging
        eprintln!("\n🚀 HTTP Request Debug:");
        eprintln!("   Method: {}", method);
        eprintln!("   URL: {}", url);
        let mask_key = if self.config.api_key.len() > 10 {
            format!("{}...{}", &self.config.api_key[..4], &self.config.api_key[self.config.api_key.len()-4..])
        } else {
            "***".to_string()
        };

        eprintln!(
            "   Authorization: Bearer {}",
            mask_key
        );

        if let Some(body) = &body {
            let body_json = serde_json::to_string_pretty(body).unwrap_or_default();
            eprintln!("   Request Body:\n{}", body_json);
        } else {
            eprintln!("   Request Body: (empty)");
        }

        let mut response = match method.to_uppercase().as_str() {
            "GET" => self
                .http_client
                .get(&url)
                .header("Authorization", &format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .header(
                    "User-Agent",
                    &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                )
                .call()?,
            "POST" => match body {
                Some(data) => self
                    .http_client
                    .post(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send_json(data)?,
                None => self
                    .http_client
                    .post(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send("")?,
            },
            "PUT" => match body {
                Some(data) => self
                    .http_client
                    .put(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send_json(data)?,
                None => self
                    .http_client
                    .put(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send("")?,
            },
            "DELETE" => self
                .http_client
                .delete(&url)
                .header("Authorization", &format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .header(
                    "User-Agent",
                    &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                )
                .call()?,
            _ => {
                return Err(TapsilatError::ConfigError(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        if response.status().as_u16() >= 400 {
            let status_code = response.status().as_u16();
            let body_text = response.body_mut().read_to_string().unwrap_or_default();

            // Debug logging for errors
            eprintln!("\n❌ HTTP Error Response Debug:");
            eprintln!("   Status: {} {}", status_code, response.status());
            eprintln!("   Error Body:\n{}", body_text);

            let error_body: serde_json::Value =
                serde_json::from_str(&body_text).unwrap_or_default();
            let message = error_body["message"]
                .as_str()
                .unwrap_or("Unknown API error")
                .to_string();

            return Err(TapsilatError::ApiError {
                status_code,
                message,
            });
        }

        let body_text = response.body_mut().read_to_string().map_err(|e| {
            TapsilatError::ConfigError(format!("Failed to read response body: {}", e))
        })?;

        // Debug logging
        eprintln!("\n📥 HTTP Response Debug:");
        eprintln!("   Status: {}", response.status());
        eprintln!("   Response Body:\n{}", body_text);

        if body_text.trim().is_empty() {
             // For some endpoints like terminate or cancel, an empty body might be fine or return just 200 OK.
             // But usually we expect JSON. If it's empty, return null Value.
             return Ok(serde_json::Value::Null);
        }

        let json_response: serde_json::Value = serde_json::from_str(&body_text).map_err(|e| {
            TapsilatError::ConfigError(format!(
                "Failed to parse response JSON: {}. Response was: {}",
                e, body_text
            ))
        })?;

        Ok(json_response)
    }
}
