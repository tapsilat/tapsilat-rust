use crate::config::Config;
use crate::error::{Result, TapsilatError};
use crate::modules::payments::PaymentModule;
use std::time::Duration;

pub struct TapsilatClient {
    config: Config,
    http_client: ureq::Agent,
    pub payments: PaymentModule,
}

impl TapsilatClient {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let http_client = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .build();

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
    ) -> Result<ureq::Response>
    where
        T: serde::Serialize,
    {
        let url = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        );

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            "PUT" => self.http_client.put(&url),
            "DELETE" => self.http_client.delete(&url),
            _ => {
                return Err(TapsilatError::ConfigError(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        request = request
            .set("Authorization", &format!("Bearer {}", self.config.api_key))
            .set("Content-Type", "application/json")
            .set(
                "User-Agent",
                &format!("tapsilat-rust/{}", env!("CARGO_PKG_VERSION")),
            );

        let response = match body {
            Some(data) => request.send_json(data)?,
            None => request.call()?,
        };

        if response.status() >= 400 {
            let status_code = response.status();
            let error_body: serde_json::Value = response.into_json().unwrap_or_default();
            let message = error_body["message"]
                .as_str()
                .unwrap_or("Unknown API error")
                .to_string();

            return Err(TapsilatError::ApiError {
                status_code,
                message,
            });
        }

        Ok(response)
    }
}
