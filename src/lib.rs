pub mod client;
pub mod config;
pub mod error;
pub mod modules;
pub mod types;

pub use client::TapsilatClient;
pub use config::Config;
pub use error::{Result, TapsilatError};
pub use types::*;
pub use modules::{PaymentModule, OrderModule, InstallmentModule, WebhookModule, Validators};

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
