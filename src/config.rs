use crate::error::{Result, TapsilatError};

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub timeout: u64,
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.tapsilat.com/v1".to_string(),
            timeout: 30,
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

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
