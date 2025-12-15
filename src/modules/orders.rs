use crate::error::Result;
use crate::types::{
    ApiResponse, CreateOrderRequest, CreateOrderResponse, Order, RefundOrderRequest,
};
use std::sync::Arc;

pub struct OrderModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl OrderModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Creates a new order
    pub fn create(&self, request: CreateOrderRequest) -> Result<CreateOrderResponse> {
        // Validation logic removed to simplify synchronization; rely on API or add later if needed.
        let response = self
            .client
            .make_request("POST", "order/create", Some(&request))?;
        serde_json::from_value(response).map_err(|e| {
            crate::error::TapsilatError::ConfigError(format!(
                "Failed to parse create order response: {}",
                e
            ))
        })
    }

    /// Retrieves an order by ID
    pub fn get(&self, reference_id: &str) -> Result<Order> {
        let endpoint = format!("order/{}", reference_id);
        let response = self.client.make_request::<()>("GET", &endpoint, None)?;
        let api_response: ApiResponse<Order> = serde_json::from_value(response).map_err(|e| {
             crate::error::TapsilatError::ConfigError(format!("Failed to parse order response: {}", e))
        })?;
        
        match api_response.data {
             Some(order) => Ok(order),
             None => Err(crate::error::TapsilatError::InvalidResponse(
                 api_response.message.unwrap_or("No data".to_string())
             ))
        }
    }

    /// Gets order status by ID
    pub fn get_status(&self, reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = format!("order/{}/status", reference_id);
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Lists orders with optional pagination
    pub fn list(&self, page: u32, per_page: u32, buyer_id: Option<String>) -> Result<serde_json::Value> {
        let mut endpoint = "order/list".to_string();
        let mut params = Vec::new();
        params.push(format!("page={}", page));
        params.push(format!("per_page={}", per_page));
        
        if let Some(bid) = buyer_id {
            params.push(format!("buyer_id={}", bid));
        }

        if !params.is_empty() {
             endpoint = format!("{}?{}", endpoint, params.join("&"));
        }

        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Cancels an order
    pub fn cancel(&self, reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = "order/cancel";
        let payload = serde_json::json!({ "reference_id": reference_id });
        self.client.make_request("POST", endpoint, Some(&payload))
    }

    /// Refunds an order (full or partial)
    pub fn refund(&self, request: RefundOrderRequest) -> Result<serde_json::Value> {
        let endpoint = "order/refund";
        let response = self.client.make_request("POST", endpoint, Some(&request))?;
        let api_response: ApiResponse<serde_json::Value> = serde_json::from_value(response).map_err(|e| {
             crate::error::TapsilatError::ConfigError(format!("Failed to parse refund response: {}", e))
        })?;

        match api_response.data {
             Some(v) => Ok(v),
             None => Ok(serde_json::Value::Null)
        }
    }
    
    /// Refunds all items in an order
    pub fn refund_all(&self, reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = "order/refund-all";
        let payload = serde_json::json!({ "reference_id": reference_id });
        self.client.make_request("POST", endpoint, Some(&payload))
    }

    /// Gets checkout URL for an order via get_order
    pub fn get_checkout_url(&self, reference_id: &str) -> Result<String> {
        let order = self.get(reference_id)?;
        order.checkout_url.ok_or_else(|| {
             crate::error::TapsilatError::InvalidResponse("Checkout URL not found".to_string())
        })
    }
    
    pub fn create_term(&self, request: crate::types::OrderPaymentTermCreateDTO) -> Result<serde_json::Value> {
         let endpoint = "order/term";
         self.client.make_request("POST", endpoint, Some(&request))
    }
    
    pub fn update_term(&self, request: crate::types::OrderPaymentTermUpdateDTO) -> Result<serde_json::Value> {
         let endpoint = "order/term/update";
         self.client.make_request("POST", endpoint, Some(&request))
    }
    
    pub fn delete_term(&self, order_id: &str, term_reference_id: &str) -> Result<serde_json::Value> {
         let endpoint = "order/term/delete";
         let payload = serde_json::json!({ "order_id": order_id, "term_reference_id": term_reference_id });
         self.client.make_request("POST", endpoint, Some(&payload))
    }
    
    pub fn refund_term(&self, request: crate::types::OrderTermRefundRequest) -> Result<serde_json::Value> {
         let endpoint = "order/term/refund";
         self.client.make_request("POST", endpoint, Some(&request))
    }

    pub fn terminate_term(&self, term_reference_id: &str, reason: Option<String>) -> Result<serde_json::Value> {
         let endpoint = "order/term/terminate";
         let mut payload = serde_json::Map::new();
         payload.insert("term_reference_id".to_string(), serde_json::Value::String(term_reference_id.to_string()));
         if let Some(r) = reason {
             payload.insert("reason".to_string(), serde_json::Value::String(r));
         }
         self.client.make_request("POST", endpoint, Some(&payload))
    }

    pub fn terminate(&self, reference_id: &str) -> Result<serde_json::Value> {
         let endpoint = "order/terminate";
         let payload = serde_json::json!({ "reference_id": reference_id });
         self.client.make_request("POST", endpoint, Some(&payload))
    }
    
    pub fn manual_callback(&self, reference_id: &str, conversation_id: Option<String>) -> Result<serde_json::Value> {
         let endpoint = "order/manual-callback";
         let mut payload = serde_json::Map::new();
         payload.insert("reference_id".to_string(), serde_json::Value::String(reference_id.to_string()));
         if let Some(cid) = conversation_id {
             payload.insert("conversation_id".to_string(), serde_json::Value::String(cid));
         }
         self.client.make_request("POST", endpoint, Some(&payload))
    }
    
    pub fn related_update(&self, reference_id: &str, related_reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = "order/related-update";
        let payload = serde_json::json!({
            "reference_id": reference_id,
            "related_reference_id": related_reference_id
        });
        self.client.make_request("POST", endpoint, Some(&payload))
    }
    
    pub fn accounting(&self, request: crate::types::OrderAccountingRequest) -> Result<serde_json::Value> {
        let endpoint = "order/accounting";
        self.client.make_request("POST", endpoint, Some(&request))
    }
    
    pub fn postauth(&self, request: crate::types::OrderPostAuthRequest) -> Result<serde_json::Value> {
         let endpoint = "order/postauth";
         self.client.make_request("POST", endpoint, Some(&request))
    }
}
