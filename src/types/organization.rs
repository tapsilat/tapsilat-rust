use serde::{Deserialize, Serialize};

/// Organization settings, as returned by `GET /organization/settings`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OrganizationSettings {
    pub ttl: u64,
    pub retry_count: u64,
    pub allow_payment: bool,
    pub session_ttl: u64,
    pub custom_checkout: bool,
    pub domain_address: String,
    pub checkout_domain: String,
    pub subscription_domain: String,
    pub has_kyc_kyx_template: bool,
    pub has_kyb_kyx_template: bool,
    pub is_money_transfer_partner: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CallbackURLDTO {
    pub callback_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserLimitRequest {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetLimitUserRequest {
    pub limit_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetVposRequest {
    pub currency_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrgUserVerifyReq {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrgUserMobileVerifyReq {
    pub user_id: String,
}
