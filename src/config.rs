//! Configuration management for the Tapsilat SDK.
//!
//! This module handles SDK configuration including API keys, base URLs,
//! and request timeouts.

use crate::error::{Result, TapsilatError};

/// Configuration for the Tapsilat SDK client.
///
/// Contains all necessary configuration options for connecting to the Tapsilat API,
/// including authentication credentials and request settings.
///
/// # Example
///
/// ```rust
/// use tapsilat::Config;
///
/// let config = Config::new("your-api-key")
///     .with_base_url("https://api.tapsilat.com/v1")
///     .with_timeout(30);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// API key for authenticating with the Tapsilat API.
    pub api_key: String,
    /// Base URL for the Tapsilat API (default: <https://panel.tapsilat.dev/api/v1>).
    pub base_url: String,
    /// Request timeout in seconds (default: 30).
    pub timeout: u64,
}

impl Config {
    /// Creates a new configuration with the given API key.
    ///
    /// Uses default values for base URL and timeout.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authenticating with Tapsilat
    ///
    /// # Example
    ///
    /// ```rust
    /// use tapsilat::Config;
    ///
    /// let config = Config::new("your-api-key");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://panel.tapsilat.dev/api/v1".to_string(),
            timeout: 30,
        }
    }

    /// Sets a custom base URL for the API.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL to use for API requests
    ///
    /// # Example
    ///
    /// ```rust
    /// use tapsilat::Config;
    ///
    /// let config = Config::new("api-key")
    ///     .with_base_url("https://api.staging.tapsilat.com/v1");
    /// ```
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets a custom timeout for API requests.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Timeout in seconds for API requests
    ///
    /// # Example
    ///
    /// ```rust
    /// use tapsilat::Config;
    ///
    /// let config = Config::new("api-key")
    ///     .with_timeout(60); // 60 second timeout
    /// ```
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Validates the configuration.
    ///
    /// Ensures that required fields are present and valid.
    ///
    /// # Errors
    ///
    /// Returns [`TapsilatError::ConfigError`] if:
    /// - API key is empty
    /// - Base URL is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use tapsilat::Config;
    ///
    /// let config = Config::new("api-key");
    /// config.validate().expect("Configuration should be valid");
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.api_key.is_empty() {
            return Err(TapsilatError::ConfigError(
                "API key cannot be empty".to_string(),
            ));
        }

        if self.base_url.is_empty() {
            return Err(TapsilatError::ConfigError(
                "Base URL cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}
