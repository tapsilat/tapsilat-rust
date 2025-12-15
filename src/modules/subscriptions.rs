use crate::error::Result;
use crate::types::{
    SubscriptionCancelRequest, SubscriptionCreateRequest, SubscriptionCreateResponse,
    SubscriptionDetail, SubscriptionGetRequest, SubscriptionRedirectRequest,
    SubscriptionRedirectResponse,
};
use std::sync::Arc;

pub struct SubscriptionModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl SubscriptionModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Creates a new subscription
    pub fn create(&self, request: SubscriptionCreateRequest) -> Result<SubscriptionCreateResponse> {
        let endpoint = "subscription/create";
        let response = self
            .client
            .make_request("POST", endpoint, Some(&request))?;
        serde_json::from_value(response).map_err(|e| {
            crate::error::TapsilatError::ConfigError(format!(
                "Failed to parse subscription create response: {}",
                e
            ))
        })
    }

    /// Gets subscription details
    pub fn get(&self, request: SubscriptionGetRequest) -> Result<SubscriptionDetail> {
        let endpoint = "subscription";
        let response = self
            .client
            .make_request("POST", endpoint, Some(&request))?;
        serde_json::from_value(response).map_err(|e| {
            crate::error::TapsilatError::ConfigError(format!(
                "Failed to parse subscription detail response: {}",
                e
            ))
        })
    }

    /// Cancels a subscription
    pub fn cancel(&self, request: SubscriptionCancelRequest) -> Result<serde_json::Value> {
        let endpoint = "subscription/cancel";
        self.client.make_request("POST", endpoint, Some(&request))
    }

    /// Lists subscriptions with pagination
    pub fn list(&self, page: u32, per_page: u32) -> Result<serde_json::Value> {
        let mut endpoint = "subscription/list".to_string();
        endpoint = format!("{}?page={}&per_page={}", endpoint, page, per_page);
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Gets redirect URL for a subscription
    pub fn redirect(&self, request: SubscriptionRedirectRequest) -> Result<SubscriptionRedirectResponse> {
        let endpoint = "subscription/redirect";
        let response = self
            .client
            .make_request("POST", endpoint, Some(&request))?;
        serde_json::from_value(response).map_err(|e| {
            crate::error::TapsilatError::ConfigError(format!(
                "Failed to parse subscription redirect response: {}",
                e
            ))
        })
    }
}
