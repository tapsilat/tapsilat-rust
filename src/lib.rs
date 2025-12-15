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
//!     currency: "TRY".to_string(),
//!     locale: "tr".to_string(),
//!     conversation_id: Some("order-123".to_string()),
//!     buyer: tapsilat::types::CreateBuyerRequest {
//!         name: "John".to_string(),
//!         surname: "Doe".to_string(),
//!         email: Some("john@example.com".to_string()),
//!         gsm_number: None, identity_number: None, registration_address: None, ip: None, city: None, country: None, zip_code: None
//!     },
//!     basket_items: None,
//!     billing_address: None,
//!     shipping_address: None,
//!     checkout_design: None,
//!     enabled_installments: None,
//!     external_reference_id: None,
//!     order_cards: None,
//!     paid_amount: None,
//!     partial_payment: None,
//!     payment_failure_url: None,
//!     payment_methods: None,
//!     payment_mode: None,
//!     payment_options: None,
//!     payment_success_url: None,
//!     payment_terms: None,
//!     pf_sub_merchant: None,
//!     redirect_failure_url: None,
//!     redirect_success_url: None,
//!     sub_organization: None,
//!     submerchants: None,
//!     tax_amount: None,
//!     three_d_force: None,
//!     metadata: None,
//! };
//!
//! let order_response = client.create_order(order_request)?;
//! println!("Order created: {:?}", order_response.order_id);
//! # Ok(())
//! # }
//! ```
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
