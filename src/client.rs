use crate::config::Config;
use crate::error::{Result, TapsilatError};
use crate::modules::{InstallmentModule, OrderModule, PaymentModule, WebhookModule};
use crate::types::*;
use serde_json::Value;

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

    // Direct Order Operations
    pub fn create_order(&self, request: CreateOrderRequest) -> Result<OrderResponse> {
        let response = self.make_request("POST", "orders", Some(&request))?;
        serde_json::from_value(response).map_err(|e| {
            TapsilatError::ConfigError(format!("Failed to parse order response: {}", e))
        })
    }

    pub fn get_order(&self, order_id: &str) -> Result<OrderResponse> {
        let endpoint = format!("orders/{}", order_id);
        let response = self.make_request::<()>("GET", &endpoint, None)?;
        serde_json::from_value(response).map_err(|e| {
            TapsilatError::ConfigError(format!("Failed to parse order response: {}", e))
        })
    }

    pub fn get_order_by_conversation_id(&self, conversation_id: &str) -> Result<OrderResponse> {
        let endpoint = format!("orders/conversation/{}", conversation_id);
        let response = self.make_request::<()>("GET", &endpoint, None)?;
        serde_json::from_value(response).map_err(|e| {
            TapsilatError::ConfigError(format!("Failed to parse order response: {}", e))
        })
    }

    pub fn cancel_order(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/cancel", order_id);
        self.make_request::<()>("POST", &endpoint, None)
    }

    pub fn refund_order(&self, order_id: &str, amount: Option<f64>) -> Result<Value> {
        let endpoint = format!("orders/{}/refund", order_id);
        let request = match amount {
            Some(amt) => serde_json::json!({"amount": amt}),
            None => serde_json::json!({}),
        };
        self.make_request("POST", &endpoint, Some(&request))
    }

    pub fn refund_all_order(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/refund-all", order_id);
        self.make_request::<()>("POST", &endpoint, None)
    }

    pub fn get_order_list(&self, page: Option<u32>, limit: Option<u32>) -> Result<Value> {
        let mut endpoint = "orders".to_string();
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        if !params.is_empty() {
            endpoint = format!("{}?{}", endpoint, params.join("&"));
        }

        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_order_status(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/status", order_id);
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_order_transactions(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/transactions", order_id);
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_order_payment_details(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/payment-details", order_id);
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn get_checkout_url(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/checkout-url", order_id);
        self.make_request::<()>("GET", &endpoint, None)
    }

    pub fn order_manual_callback(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/manual-callback", order_id);
        self.make_request::<()>("POST", &endpoint, None)
    }

    // Order Term Operations
    pub fn create_order_term(&self, order_id: &str, request: Value) -> Result<Value> {
        let endpoint = format!("orders/{}/terms", order_id);
        self.make_request("POST", &endpoint, Some(&request))
    }

    pub fn update_order_term(
        &self,
        order_id: &str,
        term_id: &str,
        request: Value,
    ) -> Result<Value> {
        let endpoint = format!("orders/{}/terms/{}", order_id, term_id);
        self.make_request("PUT", &endpoint, Some(&request))
    }

    pub fn delete_order_term(&self, order_id: &str, term_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/terms/{}", order_id, term_id);
        self.make_request::<()>("DELETE", &endpoint, None)
    }

    pub fn refund_order_term(
        &self,
        order_id: &str,
        term_id: &str,
        amount: Option<f64>,
    ) -> Result<Value> {
        let endpoint = format!("orders/{}/terms/{}/refund", order_id, term_id);
        let request = match amount {
            Some(amt) => serde_json::json!({"amount": amt}),
            None => serde_json::json!({}),
        };
        self.make_request("POST", &endpoint, Some(&request))
    }

    pub fn order_terminate(&self, order_id: &str) -> Result<Value> {
        let endpoint = format!("orders/{}/terminate", order_id);
        self.make_request::<()>("POST", &endpoint, None)
    }

    // Webhook Operations
    pub fn verify_webhook(&self, payload: &str, signature: &str, secret: &str) -> Result<bool> {
        WebhookModule::verify_webhook(payload, signature, secret)
    }

    /// Access to payment operations (for backward compatibility)
    pub fn payments(&self) -> PaymentModule {
        PaymentModule::new(std::sync::Arc::new(self.clone()))
    }

    /// Access to order operations (for backward compatibility)
    pub fn orders(&self) -> OrderModule {
        OrderModule::new(std::sync::Arc::new(self.clone()))
    }

    /// Access to installment operations (for backward compatibility)
    pub fn installments(&self) -> InstallmentModule {
        InstallmentModule::new(std::sync::Arc::new(self.clone()))
    }

    /// Access to webhook operations (for backward compatibility)
    pub fn webhooks() -> &'static WebhookModule {
        &WebhookModule
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
        eprintln!(
            "   Authorization: Bearer {}...{}",
            &self.config.api_key[..10],
            &self.config.api_key[self.config.api_key.len() - 10..]
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
            return Err(TapsilatError::ConfigError(
                "API returned empty response".to_string(),
            ));
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
