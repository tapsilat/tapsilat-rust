use crate::error::Result;
use crate::types::{ApiResponse, CreatePaymentRequest, Payment, PaymentResponse, PaginatedResponse, PaginationParams};
use crate::modules::validators::Validators;
use std::sync::Arc;

pub struct PaymentModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl PaymentModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    pub fn create(&self, request: CreatePaymentRequest) -> Result<PaymentResponse> {
        // Validate request
        Validators::validate_amount(request.amount)?;

        let response = self.client.make_request("POST", "payments", Some(&request))?;
        let api_response: ApiResponse<PaymentResponse> = serde_json::from_value(response)?;

        match api_response.data {
            Some(payment_response) => Ok(payment_response),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No payment data in response".to_string())
            ))
        }
    }

    pub fn get(&self, payment_id: &str) -> Result<Payment> {
        if payment_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Payment ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("payments/{}", payment_id);
        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<Payment> = serde_json::from_value(response)?;

        match api_response.data {
            Some(payment) => Ok(payment),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No payment data in response".to_string())
            ))
        }
    }

    pub fn list(&self, pagination: Option<PaginationParams>) -> Result<PaginatedResponse<Payment>> {
        let mut endpoint = "payments".to_string();

        // Add pagination parameters
        if let Some(params) = pagination {
            let mut query_params = Vec::new();
            
            if let Some(page) = params.page {
                query_params.push(format!("page={}", page));
            }
            
            if let Some(per_page) = params.per_page {
                query_params.push(format!("per_page={}", per_page));
            }

            if !query_params.is_empty() {
                endpoint.push('?');
                endpoint.push_str(&query_params.join("&"));
            }
        }

        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<PaginatedResponse<Payment>> = serde_json::from_value(response)?;

        match api_response.data {
            Some(paginated_payments) => Ok(paginated_payments),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No payments data in response".to_string())
            ))
        }
    }

    pub fn cancel(&self, payment_id: &str) -> Result<Payment> {
        if payment_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Payment ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("payments/{}/cancel", payment_id);
        let response = self.client.make_request::<()>("POST", &endpoint, None)?;
        let api_response: ApiResponse<Payment> = serde_json::from_value(response)?;

        match api_response.data {
            Some(payment) => Ok(payment),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No payment data in response".to_string())
            ))
        }
    }
}
