use crate::client::{path_escape, Query};
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
        self.client
            .make_request::<()>("GET", "organization/settings", None)
    }

    /// Retrieves organization settings as [`OrganizationSettings`]
    pub fn settings(&self) -> Result<OrganizationSettings> {
        self.client
            .request::<(), _>("GET", "organization/settings", None)
    }

    /// Retrieves organization callback (webhook) settings
    pub fn get_callback(&self) -> Result<serde_json::Value> {
        self.client
            .make_request::<()>("GET", "organization/callback", None)
    }

    /// Updates organization callback (webhook) settings
    pub fn update_callback(&self, request: CallbackURLDTO) -> Result<serde_json::Value> {
        self.client
            .make_request("PATCH", "organization/callback", Some(&request))
    }

    /// Creates a new business entity
    pub fn create_business(&self, request: OrgCreateBusinessRequest) -> Result<serde_json::Value> {
        self.client
            .make_request("POST", "organization/business/create", Some(&request))
    }

    /// Retrieves the organization's currencies; the response is `{currencies: [...]}`
    pub fn get_currencies(&self) -> Result<serde_json::Value> {
        self.client
            .make_request::<()>("GET", "organization/currencies", None)
    }

    /// Retrieves the organization's currencies as [`OrganizationCurrenciesResponse`]
    pub fn currencies(&self) -> Result<OrganizationCurrenciesResponse> {
        self.client
            .request::<(), _>("GET", "organization/currencies", None)
    }

    /// Lists the currencies the organization can add.
    pub fn list_currency_presets(&self) -> Result<OrganizationCurrencyPresetsResponse> {
        self.client
            .request::<(), _>("GET", "organization/currency-presets", None)
    }

    /// Adds a currency to the organization by its code, such as "EUR".
    pub fn create_currency(
        &self,
        currency_code: &str,
    ) -> Result<CreateOrganizationCurrencyResponse> {
        let payload = serde_json::json!({ "currency_code": currency_code.trim().to_uppercase() });
        let response = self
            .client
            .request("POST", "organization/currencies", Some(&payload))?;
        self.client.invalidate_currency_cache();
        Ok(response)
    }

    /// Retrieves limit information for a specific user
    #[deprecated(note = "the API has no organization user limit route; this request always fails")]
    pub fn get_limit_user(&self, user_id: &str) -> Result<serde_json::Value> {
        let endpoint = Query::new()
            .set("user_id", user_id)
            .append_to("organization/limit/user");
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Sets limit for a specific user
    #[deprecated(note = "the API has no organization user limit route; this request always fails")]
    pub fn set_limit_user(&self, request: SetLimitUserRequest) -> Result<serde_json::Value> {
        self.client
            .make_request("POST", "organization/limit/user", Some(&request))
    }

    /// Retrieves organization overall limits
    #[deprecated(note = "use get_limits_by_id; the limits route needs an organization ID")]
    pub fn get_limits(&self) -> Result<serde_json::Value> {
        self.client
            .make_request::<()>("GET", "organization/limits", None)
    }

    /// Retrieves the limits of the given organization
    pub fn get_limits_by_id(&self, organization_id: &str) -> Result<serde_json::Value> {
        let endpoint = format!("organization/{}/limits", path_escape(organization_id));
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Lists virtual POS terminals
    pub fn list_vpos(&self, currency_id: &str) -> Result<serde_json::Value> {
        let payload = serde_json::json!({ "currency_id": currency_id });
        self.client
            .make_request("POST", "organization/list-vpos", Some(&payload))
    }

    /// Retrieves an organization meta value by name
    pub fn get_meta(&self, name: &str) -> Result<serde_json::Value> {
        let endpoint = format!("organization/metas/{}", path_escape(name));
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Retrieves supported scopes
    pub fn get_scopes(&self) -> Result<serde_json::Value> {
        self.client
            .make_request::<()>("GET", "organization/scopes", None)
    }

    /// Retrieves list of sub-organizations; the response is `{rows, total, page, per_page, total_pages}`
    pub fn get_suborganizations(&self, page: u32, per_page: u32) -> Result<serde_json::Value> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .append_to("organization/suborganizations");
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Lists sub-organizations.
    pub fn list_suborganizations(
        &self,
        page: u32,
        per_page: u32,
    ) -> Result<SuborganizationListResponse> {
        let value = self.get_suborganizations(page, per_page)?;
        Ok(serde_json::from_value(value)?)
    }

    /// Retrieves a sub-organization; the server answers 404 when it does not exist.
    pub fn get_suborganization_detail(&self, id: &str) -> Result<SuborganizationDetail> {
        let endpoint = format!("organization/suborganizations/{}", path_escape(id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Returns the submerchant mapped to a sub-organization.
    pub fn get_suborganization_submerchant(
        &self,
        suborganization_id: &str,
    ) -> Result<SuborganizationSubmerchantMapping> {
        let endpoint = format!(
            "organization/suborganizations/{}/submerchant",
            path_escape(suborganization_id)
        );
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Creates a new user
    pub fn create_user(&self, request: OrgCreateUserReq) -> Result<serde_json::Value> {
        self.client
            .make_request("POST", "organization/user/create", Some(&request))
    }

    /// Creates an access token for an organization user; `expire` is given in minutes.
    pub fn create_user_token(
        &self,
        request: OrgUserTokenCreateRequest,
    ) -> Result<OrgUserTokenCreateResponse> {
        self.client
            .request("POST", "organization/user/token", Some(&request))
    }

    /// Verifies an organization user
    pub fn verify_user(&self, user_id: &str) -> Result<serde_json::Value> {
        let payload = serde_json::json!({ "user_id": user_id });
        self.client
            .make_request("POST", "organization/user/verify", Some(&payload))
    }

    /// Verifies an organization user via mobile
    pub fn verify_user_mobile(&self, user_id: &str) -> Result<serde_json::Value> {
        let payload = serde_json::json!({ "user_id": user_id });
        self.client
            .make_request("POST", "organization/user/verify-mobile", Some(&payload))
    }
}
