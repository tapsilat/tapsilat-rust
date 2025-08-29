use crate::error::Result;
use crate::types::{ApiResponse, PaginatedResponse, PaginationParams};
use crate::modules::validators::Validators;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallmentPlan {
    pub id: String,
    pub order_id: String,
    pub total_installments: u8,
    pub installment_amount: f64,
    pub currency: String,
    pub status: InstallmentStatus,
    pub installments: Vec<Installment>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Installment {
    pub id: String,
    pub installment_number: u8,
    pub amount: f64,
    pub due_date: String,
    pub paid_at: Option<String>,
    pub status: InstallmentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallmentStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "refunded")]
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstallmentPlanRequest {
    pub order_id: String,
    pub installment_count: u8,
    pub first_installment_date: String, // ISO 8601 date
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstallmentRequest {
    pub due_date: Option<String>,
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundInstallmentRequest {
    pub amount: Option<f64>, // None for full refund
    pub reason: Option<String>,
}

pub struct InstallmentModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl InstallmentModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Creates an installment plan for an order
    pub fn create_plan(&self, request: CreateInstallmentPlanRequest) -> Result<InstallmentPlan> {
        // Validate request
        self.validate_create_request(&request)?;

        let response = self.client.make_request("POST", "installments/plans", Some(&request))?;
        let api_response: ApiResponse<InstallmentPlan> = response.into_json()?;

        match api_response.data {
            Some(plan) => Ok(plan),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No installment plan data in response".to_string())
            ))
        }
    }

    /// Gets an installment plan by ID
    pub fn get_plan(&self, plan_id: &str) -> Result<InstallmentPlan> {
        if plan_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Plan ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("installments/plans/{}", plan_id);
        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<InstallmentPlan> = response.into_json()?;

        match api_response.data {
            Some(plan) => Ok(plan),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No installment plan data in response".to_string())
            ))
        }
    }

    /// Gets installment plans for an order
    pub fn get_plans_by_order(&self, order_id: &str) -> Result<Vec<InstallmentPlan>> {
        if order_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("orders/{}/installments/plans", order_id);
        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<Vec<InstallmentPlan>> = response.into_json()?;

        match api_response.data {
            Some(plans) => Ok(plans),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No installment plans data in response".to_string())
            ))
        }
    }

    /// Updates an installment
    pub fn update_installment(&self, installment_id: &str, request: UpdateInstallmentRequest) -> Result<Installment> {
        if installment_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Installment ID cannot be empty".to_string()
            ));
        }

        // Validate amount if provided
        if let Some(amount) = request.amount {
            Validators::validate_amount(amount)?;
        }

        let endpoint = format!("installments/{}", installment_id);
        let response = self.client.make_request("PUT", &endpoint, Some(&request))?;
        let api_response: ApiResponse<Installment> = response.into_json()?;

        match api_response.data {
            Some(installment) => Ok(installment),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No installment data in response".to_string())
            ))
        }
    }

    /// Cancels an installment plan
    pub fn cancel_plan(&self, plan_id: &str) -> Result<InstallmentPlan> {
        if plan_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Plan ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("installments/plans/{}/cancel", plan_id);
        let response = self.client.make_request::<()>("POST", &endpoint, None)?;
        let api_response: ApiResponse<InstallmentPlan> = response.into_json()?;

        match api_response.data {
            Some(plan) => Ok(plan),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No installment plan data in response".to_string())
            ))
        }
    }

    /// Refunds an installment
    pub fn refund_installment(&self, installment_id: &str, request: RefundInstallmentRequest) -> Result<Installment> {
        if installment_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Installment ID cannot be empty".to_string()
            ));
        }

        // Validate refund amount if provided
        if let Some(amount) = request.amount {
            Validators::validate_amount(amount)?;
        }

        let endpoint = format!("installments/{}/refund", installment_id);
        let response = self.client.make_request("POST", &endpoint, Some(&request))?;
        let api_response: ApiResponse<Installment> = response.into_json()?;

        match api_response.data {
            Some(installment) => Ok(installment),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No installment data in response".to_string())
            ))
        }
    }

    /// Lists all installment plans with pagination
    pub fn list_plans(&self, pagination: Option<PaginationParams>) -> Result<PaginatedResponse<InstallmentPlan>> {
        let mut endpoint = "installments/plans".to_string();

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
        let api_response: ApiResponse<PaginatedResponse<InstallmentPlan>> = response.into_json()?;

        match api_response.data {
            Some(paginated_plans) => Ok(paginated_plans),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No installment plans data in response".to_string())
            ))
        }
    }

    /// Validates create installment plan request
    fn validate_create_request(&self, request: &CreateInstallmentPlanRequest) -> Result<()> {
        if request.order_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order ID cannot be empty".to_string()
            ));
        }

        // Validate installment count
        Validators::validate_installments(request.installment_count)?;

        // Basic date format validation (should be more robust in production)
        if request.first_installment_date.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "First installment date cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}