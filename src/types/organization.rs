use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSettings {
    pub name: String,
    pub email: String,
    pub logo: Option<String>,
    pub callback_url: Option<String>,
    pub currencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackURLDTO {
    pub callback_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgCreateBusinessRequest {
    pub address: String,
    pub business_name: String,
    pub business_type: u32,
    pub email: String,
    pub first_name: String,
    pub identity_number: String,
    pub last_name: String,
    pub phone: String,
    pub tax_number: String,
    pub tax_office: String,
    pub zip_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserLimitRequest {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLimitUserRequest {
    pub limit_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVposRequest {
    pub currency_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgCreateUserReq {
    pub conversation_id: String,
    pub email: String,
    pub first_name: String,
    pub identity_number: String,
    pub is_mail_verified: bool,
    pub last_name: String,
    pub phone: String,
    pub reference_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUserVerifyReq {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUserMobileVerifyReq {
    pub user_id: String,
}
