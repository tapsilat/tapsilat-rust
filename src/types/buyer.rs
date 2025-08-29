use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buyer {
    pub id: Option<String>,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub gsm: String,
    pub identity_number: Option<String>,
    pub address: Option<Address>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub country: String,
    pub city: String,
    pub district: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBuyerRequest {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub gsm: String,
    pub identity_number: Option<String>,
    pub address: Option<CreateAddressRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddressRequest {
    pub country: String,
    pub city: String,
    pub district: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub postal_code: Option<String>,
}