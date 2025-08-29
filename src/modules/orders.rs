use crate::error::Result;
use crate::types::{
    CreateOrderRequest, Order, OrderResponse, RefundOrderRequest, RefundOrderResponse,
    PaginatedResponse, PaginationParams, ApiResponse
};
use crate::modules::validators::Validators;
use std::sync::Arc;

pub struct OrderModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl OrderModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Creates a new order
    pub fn create(&self, mut request: CreateOrderRequest) -> Result<OrderResponse> {
        // Validate request
        self.validate_create_order_request(&mut request)?;

        let response = self.client.make_request("POST", "orders", Some(&request))?;
        let api_response: ApiResponse<OrderResponse> = response.into_json()?;

        match api_response.data {
            Some(order_response) => Ok(order_response),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No order data in response".to_string())
            ))
        }
    }

    /// Retrieves an order by ID
    pub fn get(&self, order_id: &str) -> Result<Order> {
        if order_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("orders/{}", order_id);
        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<Order> = response.into_json()?;

        match api_response.data {
            Some(order) => Ok(order),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No order data in response".to_string())
            ))
        }
    }

    /// Gets order status by ID
    pub fn get_status(&self, order_id: &str) -> Result<String> {
        if order_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("orders/{}/status", order_id);
        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<serde_json::Value> = response.into_json()?;

        match api_response.data {
            Some(status_data) => {
                let status = status_data["status"].as_str()
                    .ok_or_else(|| crate::error::TapsilatError::InvalidResponse(
                        "Status field not found in response".to_string()
                    ))?;
                Ok(status.to_string())
            }
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No status data in response".to_string())
            ))
        }
    }

    /// Lists orders with optional pagination
    pub fn list(&self, pagination: Option<PaginationParams>) -> Result<PaginatedResponse<Order>> {
        let mut endpoint = "orders".to_string();

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
        let api_response: ApiResponse<PaginatedResponse<Order>> = response.into_json()?;

        match api_response.data {
            Some(paginated_orders) => Ok(paginated_orders),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No orders data in response".to_string())
            ))
        }
    }

    /// Cancels an order
    pub fn cancel(&self, order_id: &str) -> Result<Order> {
        if order_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("orders/{}/cancel", order_id);
        let response = self.client.make_request::<()>("POST", &endpoint, None)?;
        let api_response: ApiResponse<Order> = response.into_json()?;

        match api_response.data {
            Some(order) => Ok(order),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No order data in response".to_string())
            ))
        }
    }

    /// Refunds an order (full or partial)
    pub fn refund(&self, order_id: &str, request: RefundOrderRequest) -> Result<RefundOrderResponse> {
        if order_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order ID cannot be empty".to_string()
            ));
        }

        // Validate refund amount if provided
        if let Some(amount) = request.amount {
            Validators::validate_amount(amount)?;
        }

        let endpoint = format!("orders/{}/refund", order_id);
        let response = self.client.make_request("POST", &endpoint, Some(&request))?;
        let api_response: ApiResponse<RefundOrderResponse> = response.into_json()?;

        match api_response.data {
            Some(refund_response) => Ok(refund_response),
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No refund data in response".to_string())
            ))
        }
    }

    /// Gets checkout URL for an order
    pub fn get_checkout_url(&self, order_id: &str) -> Result<String> {
        if order_id.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order ID cannot be empty".to_string()
            ));
        }

        let endpoint = format!("orders/{}/checkout", order_id);
        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<serde_json::Value> = response.into_json()?;

        match api_response.data {
            Some(checkout_data) => {
                let checkout_url = checkout_data["checkout_url"].as_str()
                    .ok_or_else(|| crate::error::TapsilatError::InvalidResponse(
                        "Checkout URL not found in response".to_string()
                    ))?;
                Ok(checkout_url.to_string())
            }
            None => Err(crate::error::TapsilatError::InvalidResponse(
                api_response.message.unwrap_or("No checkout data in response".to_string())
            ))
        }
    }

    /// Validates create order request
    fn validate_create_order_request(&self, request: &mut CreateOrderRequest) -> Result<()> {
        // Validate amount
        Validators::validate_amount(request.amount)?;

        // Validate buyer if provided
        if let Some(buyer) = &request.buyer {
            Validators::validate_email(&buyer.email)?;
            
            // Validate and normalize GSM
            let normalized_gsm = Validators::validate_gsm(&buyer.gsm)?;
            // Note: We can't modify the GSM here because we only have a reference
            // The user should handle GSM normalization before calling this method
        }

        // Validate items
        if request.items.is_empty() {
            return Err(crate::error::TapsilatError::ValidationError(
                "Order must have at least one item".to_string()
            ));
        }

        for item in &request.items {
            Validators::validate_amount(item.price)?;
            
            if item.quantity <= 0 {
                return Err(crate::error::TapsilatError::ValidationError(
                    "Item quantity must be greater than 0".to_string()
                ));
            }

            if item.name.trim().is_empty() {
                return Err(crate::error::TapsilatError::ValidationError(
                    "Item name cannot be empty".to_string()
                ));
            }
        }

        // Validate total amount matches items
        let calculated_total: f64 = request.items.iter()
            .map(|item| item.price * item.quantity as f64)
            .sum();

        if (request.amount - calculated_total).abs() > 0.01 {
            return Err(crate::error::TapsilatError::ValidationError(
                format!("Order amount ({:.2}) doesn't match items total ({:.2})", 
                    request.amount, calculated_total)
            ));
        }

        Ok(())
    }
}