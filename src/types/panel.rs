//! Request and response types for card installments, organization currencies and users,
//! submerchants, suborganizations, virtual POS and saved cards.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Card installments

/// Selects installment options for a card BIN.
#[derive(Debug, Clone, Default)]
pub struct CardInstallmentsRequest {
    /// The card's first digits (at least 8).
    pub bin: String,
    pub amount: f64,
    /// A currency unit or name, for example "TRY".
    pub currency: String,
    pub is_three_d: bool,
}

/// One installment option with its commission.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CardInstallment {
    pub installment: u64,
    pub amount: String,
    pub total_amount: String,
    pub commission_rate: f64,
    pub monthly_payment: String,
}

/// The installment options available for a card.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CardInstallmentsResponse {
    pub is_installment_available: bool,
    pub installments: Vec<CardInstallment>,
}

/// Selects the installment table for an amount.
#[derive(Debug, Clone, Default)]
pub struct InstallmentTableRequest {
    pub amount: f64,
    /// A currency unit or name, for example "TRY".
    pub currency: String,
    pub is_three_d: bool,
}

/// One installment count within a card program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InstallmentTableRow {
    pub installment: u64,
    pub installment_amount: String,
    pub total_amount: String,
    pub commission_rate: f64,
    pub is_interest_free: bool,
}

/// Installment rows grouped by bank card program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InstallmentTableGroup {
    pub bank: String,
    pub program: String,
    pub program_code: String,
    pub rows: Vec<InstallmentTableRow>,
}

/// The installment table for every card program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct InstallmentTableResponse {
    pub groups: Vec<InstallmentTableGroup>,
}

// Organization

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrganizationCurrency {
    pub id: String,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub currency_unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrganizationCurrenciesResponse {
    pub currencies: Vec<OrganizationCurrency>,
}

/// A currency the organization can add.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrganizationCurrencyPreset {
    pub currency_code: String,
    pub currency_unit: String,
    pub name: String,
    pub minor_unit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrganizationCurrencyPresetsResponse {
    pub items: Vec<OrganizationCurrencyPreset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CreateOrganizationCurrencyResponse {
    pub code: u64,
    pub message: String,
    pub created: bool,
    pub currency: OrganizationCurrency,
}

/// Creates an access token for an organization user. `expire` is given in minutes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrgUserTokenCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<u32>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub invalidate_old_tokens: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrgUserTokenCreateResponse {
    pub code: u64,
    pub message: String,
    pub token: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrgCreateUserResponse {
    pub code: u64,
    pub message: String,
    pub user_id: String,
}

// Suborganizations

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SuborganizationListItem {
    pub id: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub phone: String,
    pub currency: String,
    pub logo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SuborganizationListResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub rows: Vec<SuborganizationListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SuborganizationDetail {
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub public_status: i64,
    pub availability_status: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubmerchantSuborganizationMapping {
    pub submerchant_id: String,
    pub suborganization_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SuborganizationSubmerchantMapping {
    pub suborganization_id: String,
    pub submerchant_id: String,
}

// Submerchants

/// Creates a submerchant. Required: `email`, `name`, `locale`, `conversation_id` and
/// `gsm_number`. `sub_merchant_type` is PERSONAL (requires `identity_number`),
/// PRIVATE_COMPANY or LIMITED_OR_JOINT_STOCK_COMPANY (both require `tax_number`).
/// `currency_id` accepts a currency UUID or unit such as "TRY".
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubmerchantCreateRequest {
    pub locale: String,
    pub conversation_id: String,
    pub name: String,
    pub email: String,
    pub gsm_number: String,
    pub address: String,
    pub iban: String,
    pub tax_office: String,
    pub legal_company_title: String,
    pub currency_id: String,
    pub sub_merchant_external_id: String,
    pub identity_number: String,
    pub sub_merchant_type: String,
    pub tax_number: String,
    pub sub_merchant_key: String,
    pub organization_id: String,
    pub status: String,
    pub system_time: i64,
    pub contact_name: String,
    pub contact_surname: String,
}

/// Replaces a submerchant; every field is stored as sent.
pub type SubmerchantUpdateRequest = SubmerchantCreateRequest;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Submerchant {
    pub id: String,
    pub locale: String,
    pub conversation_id: String,
    pub name: String,
    pub email: String,
    pub gsm_number: String,
    pub address: String,
    pub iban: String,
    pub tax_office: String,
    pub legal_company_title: String,
    pub currency_id: String,
    pub sub_merchant_external_id: String,
    pub identity_number: String,
    pub sub_merchant_type: String,
    pub tax_number: String,
    pub sub_merchant_key: String,
    pub organization_id: String,
    pub labels: String,
    pub status: String,
    pub system_time: i64,
    pub acquirer: String,
    pub contact_name: String,
    pub contact_surname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubmerchantListItem {
    pub id: String,
    pub name: String,
    pub email: String,
    pub submerchant_type: String,
    pub submerchant_key: String,
    pub labels: String,
    pub status: String,
    pub acquirer: String,
}

/// A page of submerchants; the rows come in the `row` key.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubmerchantListResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    #[serde(rename = "row")]
    pub rows: Vec<SubmerchantListItem>,
}

/// Returned by submerchant create, update and delete. Read the submerchant for its key
/// and status.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubmerchantMutationResponse {
    pub code: u64,
    pub message: String,
}

/// Records an item business event for marketplace payouts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubmerchantPayoutEventRequest {
    pub idempotency_key: String,
    pub order_reference: String,
    pub item_id: String,
    pub event_type: String,
    /// RFC3339 timestamp; the server uses the current time when it is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubmerchantPayoutEventResponse {
    pub id: String,
    pub idempotency_key: String,
    pub order_reference: String,
    pub item_id: String,
    pub event_type: String,
    pub occurred_at: String,
    pub payload: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Requests settlement approval or disapproval for an item payment. Every field is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubmerchantPaymentAction {
    pub locale: String,
    pub conversation_id: String,
    pub payment_transaction_id: String,
    pub acquirer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubmerchantPaymentActionResponse {
    pub operation_id: String,
    pub status: String,
    pub locale: String,
    pub system_time: i64,
    pub conversation_id: String,
    pub payment_transaction_id: String,
    pub acquirer: String,
}

/// Changes the seller or payout of a completed marketplace item. `sub_merchant_price` is
/// a decimal string greater than zero.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubmerchantPaymentItemUpdate {
    pub locale: String,
    pub conversation_id: String,
    pub payment_transaction_id: String,
    pub sub_merchant_key: String,
    pub sub_merchant_price: String,
    pub acquirer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubmerchantPaymentItemUpdateResponse {
    pub operation_id: String,
    pub status: String,
    pub locale: String,
    pub system_time: i64,
    pub conversation_id: String,
    pub payment_transaction_id: String,
}

// Virtual POS

/// Creates a VPOS, or replaces one on update: fields left at their default are stored as
/// such (for example `failover` and `tls` become false), so read the VPOS first and send
/// every field back. `currencies` accepts currency UUIDs or units such as "TRY".
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VposCreateRequest {
    pub name: String,
    pub bank_name: String,
    pub env_mode: String,
    pub merchant: String,
    pub merchant_code: String,
    pub merchant_key: String,
    pub auth_key: String,
    pub terminal: String,
    pub company: String,
    pub username: String,
    pub password: String,
    pub store_key: String,
    pub api_key: String,
    pub api_secret: String,
    pub guid: String,
    pub pid: String,
    pub client_id: String,
    pub client_code: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub block_date: u64,
    pub priority: i64,
    pub main: bool,
    pub force_three_d: bool,
    pub marketplace: bool,
    pub pf: bool,
    pub payment_mode: String,
    pub prefix: String,
    pub acquirer_id: String,
    pub card_schemes: Vec<String>,
    pub currencies: Vec<String>,
    pub failover: bool,
    pub pf_id: String,
    pub redirect_uri: String,
    pub tls: bool,
    pub tls_key: String,
    pub tls_cert: String,
    pub root_ca: String,
    pub main_installment: bool,
    pub main_without_installment: bool,
}

/// Replaces a VPOS configuration; see [`VposCreateRequest`].
pub type VposUpdateRequest = VposCreateRequest;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Vpos {
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub env_mode: String,
    pub provider: String,
    pub payment_mode: String,
    pub merchant: String,
    pub merchant_code: String,
    pub merchant_key: String,
    pub auth_key: String,
    pub terminal: String,
    pub company: String,
    pub username: String,
    pub password: String,
    pub store_key: String,
    pub api_key: String,
    pub api_secret: String,
    pub guid: String,
    pub pid: String,
    pub client_id: String,
    pub client_code: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub block_date: u64,
    pub priority: i64,
    pub main: bool,
    pub force_three_d: bool,
    pub marketplace: bool,
    pub pf: bool,
    pub prefix: String,
    pub acquirer_id: String,
    pub card_schemes: Vec<String>,
    pub currencies: Vec<String>,
    pub failover: bool,
    pub pf_id: String,
    pub redirect_uri: String,
    pub tls: bool,
    pub tls_key: String,
    pub tls_cert: String,
    pub root_ca: String,
    pub main_installment: bool,
    pub main_without_installment: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposListItem {
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub env_mode: String,
    pub provider: String,
    pub payment_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposListResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub rows: Vec<VposListItem>,
}

#[derive(Debug, Clone, Default)]
pub struct VposListFilter {
    pub suborganization_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposMutationResponse {
    pub code: u64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposAcquirer {
    pub id: String,
    pub name: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposAcquirerListResponse {
    pub items: Vec<VposAcquirer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CardScheme {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CardSchemeListResponse {
    pub items: Vec<CardScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposAcquirerTemplateDefaults {
    pub payment_mode: String,
    pub main: bool,
    pub marketplace: bool,
    pub force_three_d: bool,
    pub credentials: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposAcquirerTemplate {
    pub acquirer_id: String,
    pub name: String,
    pub prefix: String,
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub defaults: VposAcquirerTemplateDefaults,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposAcquirerTemplateListResponse {
    pub items: Vec<VposAcquirerTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VposSubmerchantCreateRequest {
    pub external_reference_id: String,
    pub submerchant_id: String,
    pub terminal_no: String,
    pub vpos_id: String,
    pub mcc: String,
    pub tax_id: String,
    pub national_id: String,
    pub title: String,
    pub switch_id: String,
    pub city: String,
    pub country: String,
    #[serde(rename = "country_isocode")]
    pub country_iso_code: String,
    pub postal_code: String,
    pub address: String,
    pub submerchant_url: String,
    pub submerchant_nin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VposSubmerchantUpdateRequest {
    pub external_reference_id: String,
    pub submerchant_id: String,
    pub terminal_no: String,
    pub mcc: String,
    pub tax_id: String,
    pub national_id: String,
    pub title: String,
    pub switch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposSubmerchant {
    pub id: String,
    pub external_reference_id: String,
    pub submerchant_id: String,
    pub terminal_no: String,
    pub vpos_id: String,
    pub mcc: String,
    pub tax_id: String,
    pub national_id: String,
    pub title: String,
    pub switch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposSubmerchantListItem {
    pub id: String,
    pub external_reference_id: String,
    pub submerchant_id: String,
    pub terminal_no: String,
    pub vpos_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VposSubmerchantListResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub rows: Vec<VposSubmerchantListItem>,
}

#[derive(Debug, Clone, Default)]
pub struct VposSubmerchantListFilter {
    pub vpos_id: Option<String>,
    pub external_reference_id: Option<String>,
}

// Saved cards (tokenization)

/// Tokenizes a card.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardTokenizeRequest {
    pub card_number: String,
    pub holder_name: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvv: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_failure_url: Option<String>,
}

/// The tokenization result, with 3D secure form details when required.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CardTokenizeResponse {
    pub threedform_url: String,
    pub threedform_html: String,
    pub is_success: bool,
    pub message: String,
    pub status_code: String,
    pub error_code: String,
    pub order_reference_id: String,
    pub card_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SavedCard {
    pub id: String,
    pub name: String,
    pub holder_name: String,
    pub masked_number: String,
    pub last_four: String,
    pub brand: String,
    pub bin: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListSavedCardsResponse {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub rows: Vec<SavedCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeleteSavedCardResponse {
    pub success: bool,
    pub message: String,
}
