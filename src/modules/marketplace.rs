use crate::client::{path_escape, Query};
use crate::error::Result;
use crate::types::{
    MarketplaceAccountAction, MarketplaceApprovalPolicies, MarketplaceApprovalPolicy,
    MarketplaceApprovalPolicyInput, MarketplaceApprovalSettings, MarketplaceContractFilter,
    MarketplaceContracts, MarketplacePayout, MarketplacePayoutAcceptance,
    MarketplacePayoutApproval, MarketplacePayoutFilter, MarketplacePayoutList,
    MarketplacePayoutOperation, MarketplacePayoutOperationInput, MarketplaceProfileUpdate,
    MarketplaceProfileUpdateResponse, MarketplaceReleasePolicies, MarketplaceReleasePolicy,
    MarketplaceReleasePolicyInput, MarketplaceSeller, MarketplaceSellerAccounts,
    MarketplaceSellerList, MarketplaceSubmerchantCreateRequest,
    MarketplaceSubmerchantCreateResponse, MarketplaceSubmerchantProvisioning,
};
use serde_json::Value;
use std::sync::Arc;

/// Marketplace sellers, payouts and policies. These routes are served by API v2, derived
/// from the configured base URL.
pub struct MarketplaceModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl MarketplaceModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    // Sellers

    /// Creates a logical seller; inspect `provisionings` for each VPOS account's readiness.
    pub fn create_submerchant(
        &self,
        request: MarketplaceSubmerchantCreateRequest,
    ) -> Result<MarketplaceSubmerchantCreateResponse> {
        self.client
            .request_v2("POST", "submerchants", Some(&request))
    }

    /// Returns a page of the seller directory scoped to the API token.
    pub fn list_submerchants(&self, page: u32, per_page: u32) -> Result<MarketplaceSellerList> {
        self.list_submerchants_by_external_id(page, per_page, None)
    }

    /// Returns an exact external ID match when `external_id` is set.
    pub fn list_submerchants_by_external_id(
        &self,
        page: u32,
        per_page: u32,
        external_id: Option<&str>,
    ) -> Result<MarketplaceSellerList> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .opt("external_id", external_id)
            .append_to("submerchants");
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Returns the current seller profile and its revision.
    pub fn get_submerchant(&self, id: &str) -> Result<MarketplaceSeller> {
        let endpoint = format!("submerchants/{}", path_escape(id));
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Replaces the profile at the supplied revision and reports whether review is required.
    pub fn update_submerchant(
        &self,
        id: &str,
        request: MarketplaceProfileUpdate,
    ) -> Result<MarketplaceProfileUpdateResponse> {
        let endpoint = format!("submerchants/{}", path_escape(id));
        self.client.request_v2("PATCH", &endpoint, Some(&request))
    }

    /// Returns the selected accounts' requirements for a seller operation.
    pub fn get_seller_contracts(
        &self,
        filter: &MarketplaceContractFilter,
    ) -> Result<MarketplaceContracts> {
        let mut query = Query::new()
            .set("currency_id", &filter.currency_id)
            .set("seller_type", &filter.seller_type)
            .set("operation", &filter.operation);
        for vpos_id in &filter.vpos_ids {
            query = query.set("vpos_id", vpos_id);
        }
        let endpoint = query.append_to("marketplace/seller-contracts");
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Returns account registration, synchronization and available actions.
    pub fn get_seller_accounts(&self, id: &str) -> Result<MarketplaceSellerAccounts> {
        let endpoint = format!("submerchants/{}/providers", path_escape(id));
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Executes an available account action at the supplied revision and environment.
    pub fn run_seller_account_action(
        &self,
        id: &str,
        vpos_id: &str,
        request: MarketplaceAccountAction,
    ) -> Result<MarketplaceSubmerchantProvisioning> {
        let endpoint = format!(
            "submerchants/{}/providers/{}/actions",
            path_escape(id),
            path_escape(vpos_id)
        );
        self.client.request_v2("POST", &endpoint, Some(&request))
    }

    // Payouts

    /// Returns filtered organization-scoped payouts and their release state.
    pub fn list_payouts(&self, filter: &MarketplacePayoutFilter) -> Result<MarketplacePayoutList> {
        let endpoint = Query::new()
            .opt("order_id", filter.order_id.as_deref())
            .opt("submerchant_id", filter.submerchant_id.as_deref())
            .opt("vpos_id", filter.vpos_id.as_deref())
            .opt("provider", filter.provider.as_deref())
            .opt("status", filter.status.as_deref())
            .opt("release_status", filter.release_status.as_deref())
            .opt("approval_mode", filter.approval_mode.as_deref())
            .opt("currency", filter.currency.as_deref())
            .opt("eligibility", filter.eligibility.as_deref())
            .opt("search", filter.search.as_deref())
            .opt("min_amount", filter.min_amount.as_deref())
            .opt("max_amount", filter.max_amount.as_deref())
            .opt("start_date", filter.start_date.as_deref())
            .opt("end_date", filter.end_date.as_deref())
            .set("page", filter.page)
            .set("per_page", filter.per_page)
            .append_to("submerchant-payouts");
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Returns a payout's eligibility, approval and settlement state.
    pub fn get_payout(&self, id: &str) -> Result<MarketplacePayout> {
        let endpoint = format!("submerchant-payouts/{}", path_escape(id));
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Submits eligible payouts to the approval workflow without bypassing release gates.
    /// The server answers 202 Accepted.
    pub fn approve_payouts(
        &self,
        request: MarketplacePayoutApproval,
    ) -> Result<MarketplacePayoutAcceptance> {
        self.client
            .request_v2("POST", "submerchant-payouts/approve", Some(&request))
    }

    /// Changes allocation or withdraws approval; inspect `status` before treating it as complete.
    pub fn run_payout_operation(
        &self,
        payout_id: &str,
        request: MarketplacePayoutOperationInput,
    ) -> Result<MarketplacePayoutOperation> {
        let endpoint = format!("submerchant-payouts/{}/operations", path_escape(payout_id));
        self.client.request_v2("POST", &endpoint, Some(&request))
    }

    /// Returns the persisted operation receipt scoped to the API token.
    pub fn get_payout_operation(&self, id: &str) -> Result<MarketplacePayoutOperation> {
        let endpoint = format!("marketplace/payout-operations/{}", path_escape(id));
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Checks the provider result without repeating a mutation.
    pub fn reconcile_payout_operation(&self, id: &str) -> Result<MarketplacePayoutOperation> {
        let endpoint = format!(
            "marketplace/payout-operations/{}/reconcile",
            path_escape(id)
        );
        self.client.request_v2::<(), _>("POST", &endpoint, None)
    }

    /// Repeats an unresolved change only after verifying the previous provider state.
    pub fn retry_payout_operation(&self, id: &str) -> Result<MarketplacePayoutOperation> {
        let endpoint = format!("marketplace/payout-operations/{}/retry", path_escape(id));
        self.client.request_v2::<(), _>("POST", &endpoint, None)
    }

    // Approval settings and policies

    pub fn get_approval_settings(&self) -> Result<MarketplaceApprovalSettings> {
        self.client
            .request_v2::<(), _>("GET", "marketplace/approval-settings", None)
    }

    pub fn update_approval_settings(
        &self,
        request: MarketplaceApprovalSettings,
    ) -> Result<MarketplaceApprovalSettings> {
        self.client
            .request_v2("PATCH", "marketplace/approval-settings", Some(&request))
    }

    pub fn list_approval_policies(&self) -> Result<MarketplaceApprovalPolicies> {
        self.client
            .request_v2::<(), _>("GET", "marketplace/approval-policies", None)
    }

    pub fn get_approval_policy(&self, id: &str) -> Result<MarketplaceApprovalPolicy> {
        let endpoint = format!("marketplace/approval-policies/{}", path_escape(id));
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    /// Creates an approval rule. A `currency_id` given as a unit such as "TRY" is resolved
    /// to its UUID.
    pub fn create_approval_policy(
        &self,
        mut request: MarketplaceApprovalPolicyInput,
    ) -> Result<()> {
        request.currency_id = self
            .client
            .resolve_optional_currency_id(&request.currency_id)?;
        self.client.make_versioned_request(
            "v2",
            "POST",
            "marketplace/approval-policies",
            Some(&request),
        )?;
        Ok(())
    }

    /// Replaces an approval rule. A `currency_id` given as a unit such as "TRY" is resolved
    /// to its UUID.
    pub fn update_approval_policy(
        &self,
        id: &str,
        mut request: MarketplaceApprovalPolicyInput,
    ) -> Result<()> {
        request.currency_id = self
            .client
            .resolve_optional_currency_id(&request.currency_id)?;
        let endpoint = format!("marketplace/approval-policies/{}", path_escape(id));
        self.client
            .make_versioned_request("v2", "PATCH", &endpoint, Some(&request))?;
        Ok(())
    }

    /// Deletes an approval rule; the server answers 204 No Content.
    pub fn delete_approval_policy(&self, id: &str) -> Result<()> {
        let endpoint = format!("marketplace/approval-policies/{}", path_escape(id));
        self.client
            .make_versioned_request::<Value>("v2", "DELETE", &endpoint, None)?;
        Ok(())
    }

    pub fn list_release_policies(&self) -> Result<MarketplaceReleasePolicies> {
        self.client
            .request_v2::<(), _>("GET", "marketplace/release-policies", None)
    }

    pub fn get_release_policy(&self, id: &str) -> Result<MarketplaceReleasePolicy> {
        let endpoint = format!("marketplace/release-policies/{}", path_escape(id));
        self.client.request_v2::<(), _>("GET", &endpoint, None)
    }

    pub fn create_release_policy(
        &self,
        request: MarketplaceReleasePolicyInput,
    ) -> Result<MarketplaceReleasePolicy> {
        self.client
            .request_v2("POST", "marketplace/release-policies", Some(&request))
    }

    pub fn update_release_policy(
        &self,
        id: &str,
        request: MarketplaceReleasePolicyInput,
    ) -> Result<MarketplaceReleasePolicy> {
        let endpoint = format!("marketplace/release-policies/{}", path_escape(id));
        self.client.request_v2("PATCH", &endpoint, Some(&request))
    }

    /// Deletes a release policy; the server answers 204 No Content.
    pub fn delete_release_policy(&self, id: &str) -> Result<()> {
        let endpoint = format!("marketplace/release-policies/{}", path_escape(id));
        self.client
            .make_versioned_request::<Value>("v2", "DELETE", &endpoint, None)?;
        Ok(())
    }
}
