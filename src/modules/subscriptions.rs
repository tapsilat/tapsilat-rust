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
        let response = self.client.make_request("POST", endpoint, Some(&request))?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets subscription details
    pub fn get(&self, request: SubscriptionGetRequest) -> Result<SubscriptionDetail> {
        let endpoint = "subscription";
        let response = self.client.make_request("POST", endpoint, Some(&request))?;
        Ok(serde_json::from_value(response)?)
    }

    /// Cancels a subscription
    pub fn cancel(&self, request: SubscriptionCancelRequest) -> Result<serde_json::Value> {
        let endpoint = "subscription/cancel";
        self.client.make_request("POST", endpoint, Some(&request))
    }

    /// Lists subscriptions with pagination
    pub fn list(&self, page: u32, per_page: u32) -> Result<serde_json::Value> {
        let endpoint = crate::client::Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .append_to("subscription/list");
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Gets redirect URL for a subscription
    pub fn redirect(
        &self,
        request: SubscriptionRedirectRequest,
    ) -> Result<SubscriptionRedirectResponse> {
        let endpoint = "subscription/redirect";
        let response = self.client.make_request("POST", endpoint, Some(&request))?;
        Ok(serde_json::from_value(response)?)
    }
}
