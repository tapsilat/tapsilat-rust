use crate::client::{path_escape, Query};
use crate::error::Result;
use crate::types::{
    Submerchant, SubmerchantCreateRequest, SubmerchantListResponse, SubmerchantMutationResponse,
    SubmerchantPaymentAction, SubmerchantPaymentActionResponse, SubmerchantPaymentItemUpdate,
    SubmerchantPaymentItemUpdateResponse, SubmerchantPayoutEventRequest,
    SubmerchantPayoutEventResponse, SubmerchantSuborganizationMapping, SubmerchantUpdateRequest,
};
use std::sync::Arc;

/// Submerchant operations (API v1). Marketplace sellers are managed through
/// [`crate::modules::MarketplaceModule`].
pub struct SubmerchantModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl SubmerchantModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Creates a submerchant. The response carries only a code and message; read the
    /// submerchant for its key and status.
    pub fn create(&self, request: SubmerchantCreateRequest) -> Result<SubmerchantMutationResponse> {
        self.client.request("POST", "submerchants", Some(&request))
    }

    pub fn get(&self, id: &str) -> Result<Submerchant> {
        let endpoint = format!("submerchants/{}", path_escape(id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    pub fn list(&self, page: u32, per_page: u32) -> Result<SubmerchantListResponse> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .append_to("submerchants");
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Replaces a submerchant; every field is stored as sent.
    pub fn update(
        &self,
        id: &str,
        request: SubmerchantUpdateRequest,
    ) -> Result<SubmerchantMutationResponse> {
        let endpoint = format!("submerchants/{}", path_escape(id));
        self.client.request("PATCH", &endpoint, Some(&request))
    }

    pub fn delete(&self, id: &str) -> Result<SubmerchantMutationResponse> {
        let endpoint = format!("submerchants/{}", path_escape(id));
        self.client.request::<(), _>("DELETE", &endpoint, None)
    }

    /// Returns the sub-organization mapped to a submerchant.
    pub fn get_suborganization(&self, id: &str) -> Result<SubmerchantSuborganizationMapping> {
        let endpoint = format!("submerchants/{}/suborganization", path_escape(id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Records an item business event; time gates, approval and provider settlement
    /// remain separate. The server answers 202 Accepted.
    pub fn record_payout_event(
        &self,
        request: SubmerchantPayoutEventRequest,
    ) -> Result<SubmerchantPayoutEventResponse> {
        self.client
            .request("POST", "submerchant-payout-events", Some(&request))
    }

    /// Approves settlement of an item payment to its submerchant.
    pub fn approve_payment(
        &self,
        request: SubmerchantPaymentAction,
    ) -> Result<SubmerchantPaymentActionResponse> {
        self.client
            .request("POST", "submerchants/payment/approve", Some(&request))
    }

    /// Withdraws settlement approval of an item payment.
    pub fn disapprove_payment(
        &self,
        request: SubmerchantPaymentAction,
    ) -> Result<SubmerchantPaymentActionResponse> {
        self.client
            .request("POST", "submerchants/payment/disapprove", Some(&request))
    }

    /// Changes the seller or payout of a completed marketplace item.
    pub fn update_payment_item(
        &self,
        request: SubmerchantPaymentItemUpdate,
    ) -> Result<SubmerchantPaymentItemUpdateResponse> {
        self.client
            .request("PUT", "submerchants/payment/item", Some(&request))
    }
}
