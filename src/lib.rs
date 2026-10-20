//! # Tapsilat Rust SDK
//!
//! A comprehensive Rust SDK for the Tapsilat payment processing platform.
//!
//! ## Overview
//!
//! The Tapsilat SDK provides a type-safe, ergonomic interface for integrating with the
//! Tapsilat API. It covers orders and payment terms, card installments, organization
//! settings, submerchants, virtual POS, saved cards, subscriptions, the marketplace
//! (API v2), webhook verification and validation utilities.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tapsilat::{Config, TapsilatClient, CreateOrderRequest, Currency};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize the client
//! let config = Config::new("your-api-key")
//!     .with_base_url("https://panel.tapsilat.dev/api/v1")
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
//!         ..Default::default()
//!     },
//!     ..Default::default()
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
//! - **Webhook Support**: `Tapsilat-Signature` verification, callback parsing and order statuses
//! - **Installments**: Card installment options and the installment table
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
pub use modules::{
    InstallmentModule, MarketplaceModule, OrderModule, OrganizationModule, PaymentModule,
    SubmerchantModule, SubscriptionModule, TokenizationModule, Validators, VposModule,
    WebhookModule, WebhookVerifyOptions, DEFAULT_WEBHOOK_TOLERANCE, WEBHOOK_SIGNATURE_HEADER,
};
pub use types::*;

// Re-export the deprecated installment plan types for compatibility
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
