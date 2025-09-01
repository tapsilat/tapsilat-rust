//! # Tapsilat Rust SDK
//!
//! A comprehensive Rust SDK for the Tapsilat payment processing platform.
//!
//! ## Overview
//!
//! The Tapsilat SDK provides a type-safe, ergonomic interface for integrating with the
//! Tapsilat API. It supports order creation, payment processing, installment plans,
//! webhook verification, and comprehensive validation utilities.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tapsilat::{Config, TapsilatClient, CreateOrderRequest, Currency};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize the client
//! let config = Config::new("your-api-key")
//!     .with_base_url("https://api.tapsilat.com/v1")
//!     .with_timeout(30);
//! let client = TapsilatClient::new(config)?;
//!
//! // Create an order
//! let order_request = CreateOrderRequest {
//!     amount: 100.0,
//!     currency: Currency::TRY,
//!     locale: Some("tr".to_string()),
//!     conversation_id: Some("order-123".to_string()),
//!     description: Some("Test order".to_string()),
//!     buyer: None,
//!     items: vec![],
//!     callback_url: Some("https://example.com/callback".to_string()),
//!     metadata: None,
//! };
//!
//! let order_response = client.create_order(order_request)?;
//! println!("Order created: {}", order_response.order.id);
//! # Ok(())
//! # }
//!
//! ## Features
//!
//! - **Type Safety**: Comprehensive type definitions with validation
//! - **Error Handling**: Robust error types with detailed context
//! - **Flexible API**: Both direct client methods and modular interfaces
//! - **Validation**: Built-in validators for Turkish phone numbers, emails, and identity numbers
//! - **Webhook Support**: Cryptographic webhook verification
//! - **Installments**: Support for installment plan creation and management
//!
//! ## Module Organization
//!
//! - [`client`] - Core HTTP client and API methods
//! - [`config`] - Configuration management
//! - [`error`] - Error types and handling
//! - [`types`] - Data types for API requests and responses
//! - [`modules`] - Modular API interfaces (orders, payments, webhooks, etc.)

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
