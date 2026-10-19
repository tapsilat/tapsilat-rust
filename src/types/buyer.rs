use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Buyer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub surname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "gsm_number", skip_serializing_if = "Option::is_none")]
    pub gsm_number: Option<String>,
    #[serde(rename = "identity_number", skip_serializing_if = "Option::is_none")]
    pub identity_number: Option<String>,
    #[serde(rename = "last_login_date", skip_serializing_if = "Option::is_none")]
    pub last_login_date: Option<String>,
    #[serde(rename = "registration_date", skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[serde(
        rename = "registration_address",
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Birth date in YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    // Python BillingAddressDTO/ShippingAddressDTO fields mapping
    #[serde(rename = "contact_name", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tax_office", skip_serializing_if = "Option::is_none")]
    pub tax_office: Option<String>,
    #[serde(rename = "vat_number", skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBuyerRequest {
    pub name: String,
    pub surname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "gsm_number", skip_serializing_if = "Option::is_none")]
    pub gsm_number: Option<String>,
    #[serde(rename = "identity_number", skip_serializing_if = "Option::is_none")]
    pub identity_number: Option<String>,
    #[serde(
        rename = "registration_address",
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Birth date in YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(rename = "contact_name", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
}
