use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buyer {
    pub id: Option<String>,
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    #[serde(rename = "gsm_number")]
    pub gsm_number: Option<String>,
    #[serde(rename = "identity_number")]
    pub identity_number: Option<String>,
    #[serde(rename = "last_login_date")]
    pub last_login_date: Option<String>,
    #[serde(rename = "registration_date")]
    pub registration_date: Option<String>,
    #[serde(rename = "registration_address")]
    pub registration_address: Option<String>,
    pub ip: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub country: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<String>,
    #[serde(rename = "zip_code")]
    pub postal_code: Option<String>,
    // Python BillingAddressDTO/ShippingAddressDTO fields mapping
    #[serde(rename = "contact_name")]
    pub contact_name: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "tax_office")]
    pub tax_office: Option<String>,
    #[serde(rename = "vat_number")]
    pub vat_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBuyerRequest {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    #[serde(rename = "gsm_number")]
    pub gsm_number: Option<String>,
    #[serde(rename = "identity_number")]
    pub identity_number: Option<String>,
    #[serde(rename = "registration_address")]
    pub registration_address: Option<String>,
    pub ip: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddressRequest {
    pub country: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<String>,
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
    #[serde(rename = "contact_name")]
    pub contact_name: Option<String>,
}
