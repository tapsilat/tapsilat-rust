//! Request and response types for the marketplace (API v2): sellers, their provider
//! accounts, payouts, payout operations and approval and release policies.
//!
//! Revisions are RFC3339 timestamps with nanosecond precision that the server compares
//! exactly. Keep them as the strings you received and send them back unchanged.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Creates a marketplace seller. Without `vpos_id` or `provider_accounts` only the seller
/// is created.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceSubmerchantCreateRequest {
    /// Explicitly selects accounts in one environment; an empty list creates only the
    /// seller. It cannot be combined with `vpos_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_accounts: Option<Vec<MarketplaceAccountTarget>>,
    pub locale: String,
    pub conversation_id: String,
    pub name: String,
    pub email: String,
    pub gsm_number: String,
    pub address: String,
    pub iban: String,
    pub tax_office: String,
    pub legal_company_title: String,
    /// A currency UUID or unit such as "TRY".
    pub currency_id: String,
    pub sub_merchant_external_id: String,
    pub identity_number: String,
    pub sub_merchant_type: String,
    pub tax_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    pub status: String,
    pub system_time: i64,
    pub contact_name: String,
    pub contact_surname: String,
    /// Targets only this account, using its configured environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpos_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceAccountTarget {
    pub vpos_id: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSubmerchantProvisioning {
    pub vpos_id: String,
    pub vpos_submerchant_id: String,
    pub provider: String,
    pub status: String,
    pub provider_submerchant_key: String,
    pub retryable: bool,
    pub error_message: String,
}

/// The created seller; inspect `provisionings` for each VPOS account's readiness.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSubmerchantCreateResponse {
    pub id: String,
    pub suborganization_id: String,
    pub vpos_submerchant_id: String,
    pub routing_reference: String,
    pub provisionings: Vec<MarketplaceSubmerchantProvisioning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSeller {
    pub id: String,
    pub routing_reference: String,
    /// Opaque revision; send it back unchanged when updating the seller.
    pub revision: String,
    pub name: String,
    pub email: String,
    pub gsm_number: String,
    pub address: String,
    pub iban: String,
    pub currency_id: String,
    pub sub_merchant_type: String,
    pub sub_merchant_external_id: String,
    pub identity_number: String,
    pub tax_number: String,
    pub tax_office: String,
    pub legal_company_title: String,
    pub contact_name: String,
    pub contact_surname: String,
    pub locale: String,
    pub conversation_id: String,
    pub status: String,
    pub labels: Vec<String>,
    pub approval_mode: Option<String>,
    pub release_policy_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSellerList {
    pub rows: Vec<MarketplaceSeller>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

/// Replaces the saved seller profile at `revision`. Leave `vpos_id` and
/// `provider_accounts` unset.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceProfileUpdate {
    #[serde(flatten)]
    pub profile: MarketplaceSubmerchantCreateRequest,
    pub revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceProfileUpdateResponse {
    pub approval_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSellerContract {
    pub supported: bool,
    pub remote: bool,
    pub fields: HashMap<String, bool>,
    pub immutable_fields: Vec<String>,
    pub currencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceContracts {
    pub contracts: HashMap<String, MarketplaceSellerContract>,
}

/// Selects seller contracts; `currency_id` accepts a currency UUID or unit such as "TRY".
#[derive(Debug, Clone, Default)]
pub struct MarketplaceContractFilter {
    pub currency_id: String,
    pub seller_type: String,
    pub operation: String,
    pub vpos_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSynchronization {
    pub status: String,
    pub operation: String,
    pub error_message: String,
    pub needs_update: bool,
    pub attempts: u64,
    pub verified_at: Option<String>,
    pub missing_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSellerAccount {
    pub vpos_id: String,
    pub name: String,
    pub provider: String,
    pub environment: String,
    pub eligible: bool,
    pub mapped: bool,
    pub status: String,
    pub attempts: u64,
    pub updated_at: Option<String>,
    pub error_message: String,
    pub retryable: bool,
    pub revision: String,
    pub seller_revision: String,
    pub account_revision: String,
    pub actions: Vec<String>,
    pub contracts: HashMap<String, MarketplaceSellerContract>,
    pub contract_error: String,
    pub sync: Option<MarketplaceSynchronization>,
    pub registration_profile_changed: bool,
    pub registration_missing_fields: Vec<String>,
    pub provider_submerchant_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceSellerAccounts {
    pub rows: Vec<MarketplaceSellerAccount>,
}

/// Confirms a listed account action using the account's latest revision and environment.
/// Retry reuses the original snapshot; resubmit registers the current saved profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceAccountAction {
    pub action: String,
    pub environment: String,
    pub revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplacePayout {
    /// Opaque revision; send it back unchanged in payout operations.
    pub revision: String,
    pub active_operation_id: String,
    pub id: String,
    pub provider: String,
    pub organization_id: String,
    pub submerchant_id: String,
    pub submerchant_name: String,
    pub order_id: String,
    pub order_reference_id: String,
    pub order_display: String,
    pub amount: String,
    pub currency: String,
    pub approval_mode: String,
    pub approval_rule_id: String,
    pub approval_rule_name: String,
    pub release_policy_id: String,
    pub release_policy_name: String,
    pub eligibility_status: String,
    pub required_event: String,
    pub item_id: String,
    pub business_eligible_at: Option<String>,
    pub eligible_at: Option<String>,
    pub release_status: String,
    pub status: String,
    pub provider_reference: String,
    pub attempts: u64,
    pub released_at: Option<String>,
    pub created_at: String,
    pub error_message: String,
}

/// Filters payouts. Dates use YYYY-MM-DD and `currency` takes a unit such as "TRY".
#[derive(Debug, Clone, Default)]
pub struct MarketplacePayoutFilter {
    pub order_id: Option<String>,
    pub submerchant_id: Option<String>,
    pub vpos_id: Option<String>,
    pub provider: Option<String>,
    pub status: Option<String>,
    pub release_status: Option<String>,
    pub approval_mode: Option<String>,
    pub currency: Option<String>,
    pub eligibility: Option<String>,
    pub search: Option<String>,
    pub min_amount: Option<String>,
    pub max_amount: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplacePayoutList {
    pub rows: Vec<MarketplacePayout>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplacePayoutApproval {
    pub ids: Vec<String>,
}

/// Acknowledges local approval; payout completion is reported by the payout status.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplacePayoutAcceptance {
    pub status: String,
}

/// Changes a payout's allocation or withdraws its approval.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplacePayoutOperationInput {
    pub kind: String,
    pub idempotency_key: String,
    /// The payout's revision, exactly as received.
    pub revision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplacePayoutOperation {
    pub id: String,
    pub payout_id: String,
    pub kind: String,
    pub status: String,
    pub attempts: u64,
    pub error_message: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceApprovalPolicy {
    pub id: String,
    pub name: String,
    pub priority: u64,
    pub enabled: bool,
    pub provider: String,
    pub min_amount: Option<String>,
    pub max_amount: Option<String>,
    pub currency_id: Option<String>,
    pub submerchant_ids: Vec<String>,
    pub labels_any: Vec<String>,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceApprovalPolicies {
    pub rows: Vec<MarketplaceApprovalPolicy>,
}

/// Creates or replaces an approval rule. `currency_id` must be a currency UUID on the
/// wire, so the SDK resolves units such as "TRY" before sending.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceApprovalPolicyInput {
    pub name: String,
    pub priority: u64,
    pub enabled: Option<bool>,
    pub provider: String,
    pub min_amount: Option<String>,
    pub max_amount: Option<String>,
    pub currency_id: String,
    pub submerchant_ids: Vec<String>,
    pub labels_any: Vec<String>,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceReleasePolicy {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub default: bool,
    pub minimum_delay_seconds: i64,
    pub explicit_not_before: String,
    pub required_external_event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceReleasePolicies {
    pub rows: Vec<MarketplaceReleasePolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceReleasePolicyInput {
    pub name: String,
    pub enabled: bool,
    pub default: bool,
    pub minimum_delay_seconds: i64,
    pub explicit_not_before: String,
    pub required_external_event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MarketplaceApprovalSettings {
    pub default_mode: String,
}
