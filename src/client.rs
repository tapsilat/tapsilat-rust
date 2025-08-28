use crate::config::Config;
use crate::error::{Result, TapsilatError};
use crate::modules::payments::PaymentModule;

pub struct TapsilatClient {
    config: Config,
    http_client: ureq::Agent,
    pub payments: PaymentModule,
}

impl TapsilatClient {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let http_client = ureq::Agent::new_with_defaults();

        let payments = PaymentModule::new(&config);

        Ok(Self {
            config,
            http_client,
            payments,
        })
    }

    pub fn from_api_key(api_key: impl Into<String>) -> Result<Self> {
        let config = Config::new(api_key);
        Self::new(config)
    }

    pub(crate) fn make_request<T>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<serde_json::Value>
    where
        T: serde::Serialize,
    {
        let url = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        );

        let mut response = match method.to_uppercase().as_str() {
            "GET" => self
                .http_client
                .get(&url)
                .header("Authorization", &format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .header(
                    "User-Agent",
                    &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                )
                .call()?,
            "POST" => match body {
                Some(data) => self
                    .http_client
                    .post(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send_json(data)?,
                None => self
                    .http_client
                    .post(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send("")?,
            },
            "PUT" => match body {
                Some(data) => self
                    .http_client
                    .put(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send_json(data)?,
                None => self
                    .http_client
                    .put(&url)
                    .header("Authorization", &format!("Bearer {}", self.config.api_key))
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .send("")?,
            },
            "DELETE" => self
                .http_client
                .delete(&url)
                .header("Authorization", &format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .header(
                    "User-Agent",
                    &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
                )
                .call()?,
            _ => {
                return Err(TapsilatError::ConfigError(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        if response.status().as_u16() >= 400 {
            let status_code = response.status().as_u16();
            let body_text = response.body_mut().read_to_string().unwrap_or_default();
            let error_body: serde_json::Value =
                serde_json::from_str(&body_text).unwrap_or_default();
            let message = error_body["message"]
                .as_str()
                .unwrap_or("Unknown API error")
                .to_string();

            return Err(TapsilatError::ApiError {
                status_code,
                message,
            });
        }

        let body_text = response.body_mut().read_to_string().map_err(|e| {
            TapsilatError::ConfigError(format!("Failed to read response body: {}", e))
        })?;

        let json_response: serde_json::Value = serde_json::from_str(&body_text).map_err(|e| {
            TapsilatError::ConfigError(format!("Failed to parse response JSON: {}", e))
        })?;

        Ok(json_response)
    }
}
