pub mod client;
pub mod config;
pub mod error;
pub mod modules;
pub mod types;

pub use client::TapsilatClient;
pub use config::Config;
pub use error::{Result, TapsilatError};
pub use modules::{InstallmentModule, OrderModule, PaymentModule, Validators, WebhookModule};
pub use types::*;

// Re-export installment types for convenience
pub use modules::installments::{
    CreateInstallmentPlanRequest, Installment, InstallmentPlan, InstallmentStatus,
    RefundInstallmentRequest, UpdateInstallmentRequest,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = Config::new("test-api-key");
        let client = TapsilatClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_config_validation() {
        let config = Config::new("");
        assert!(config.validate().is_err());
    }
}
