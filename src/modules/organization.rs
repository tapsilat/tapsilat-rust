use crate::error::Result;
use crate::types::*;
use std::sync::Arc;

pub struct OrganizationModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl OrganizationModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Retrieves organization settings
    pub fn get_settings(&self) -> Result<serde_json::Value> {
        self.client.make_request::<()>("GET", "organization/settings", None)
    }

    /// Retrieves organization callback (webhook) settings
    pub fn get_callback(&self) -> Result<serde_json::Value> {
        self.client.make_request::<()>("GET", "organization/callback", None)
    }

    /// Updates organization callback (webhook) settings
    pub fn update_callback(&self, request: CallbackURLDTO) -> Result<serde_json::Value> {
        self.client.make_request("PATCH", "organization/callback", Some(&request))
    }

    /// Creates a new business entity
    pub fn create_business(&self, request: OrgCreateBusinessRequest) -> Result<serde_json::Value> {
        self.client.make_request("POST", "organization/business/create", Some(&request))
    }

    /// Retrieves supported currencies
    pub fn get_currencies(&self) -> Result<serde_json::Value> {
        self.client.make_request::<()>("GET", "organization/currencies", None)
    }

    /// Retrieves limit information for a specific user
    pub fn get_limit_user(&self, user_id: &str) -> Result<serde_json::Value> {
        let endpoint = format!("organization/limit/user?user_id={}", user_id);
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Sets limit for a specific user
    pub fn set_limit_user(&self, request: SetLimitUserRequest) -> Result<serde_json::Value> {
        self.client.make_request("POST", "organization/limit/user", Some(&request))
    }

    /// Retrieves organization overall limits
    pub fn get_limits(&self) -> Result<serde_json::Value> {
        self.client.make_request::<()>("GET", "organization/limits", None)
    }

    /// Lists virtual POS terminals
    pub fn list_vpos(&self, currency_id: &str) -> Result<serde_json::Value> {
        let payload = serde_json::json!({ "currency_id": currency_id });
        self.client.make_request("POST", "organization/list-vpos", Some(&payload))
    }

    /// Retrieves meta information
    pub fn get_meta(&self, name: &str) -> Result<serde_json::Value> {
        let endpoint = format!("organization/meta/{}", name);
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Retrieves supported scopes
    pub fn get_scopes(&self) -> Result<serde_json::Value> {
        self.client.make_request::<()>("GET", "organization/scopes", None)
    }

    /// Retrieves list of sub-organizations
    pub fn get_suborganizations(&self, page: u32, per_page: u32) -> Result<serde_json::Value> {
        let endpoint = format!("organization/suborganizations?page={}&per_page={}", page, per_page);
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Creates a new user
    pub fn create_user(&self, request: OrgCreateUserReq) -> Result<serde_json::Value> {
        self.client.make_request("POST", "organization/user/create", Some(&request))
    }

    /// Verifies an organization user
    pub fn verify_user(&self, user_id: &str) -> Result<serde_json::Value> {
        let payload = serde_json::json!({ "user_id": user_id });
        self.client.make_request("POST", "organization/user/verify", Some(&payload))
    }

    /// Verifies an organization user via mobile
    pub fn verify_user_mobile(&self, user_id: &str) -> Result<serde_json::Value> {
        let payload = serde_json::json!({ "user_id": user_id });
        self.client.make_request("POST", "organization/user/verify-mobile", Some(&payload))
    }
}
