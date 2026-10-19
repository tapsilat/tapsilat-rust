use crate::client::{path_escape, Query};
use crate::error::Result;
use crate::modules::validators::Validators;
use crate::types::{
    CreateOrderRequest, CreateOrderResponse, GetOrderPaymentsRequest, GetOrderPaymentsResponse,
    Order, OrderChargeRequest, OrderChargeResponse, OrderDetail, OrderListFilter,
    OrderSubmerchantListResponse, RefundOrderRequest,
};
use std::sync::Arc;

pub struct OrderModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl OrderModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Creates an order. The buyer's GSM number is cleaned and validated first, and the
    /// checkout URL is filled from the created order when it can be read.
    pub fn create(&self, mut request: CreateOrderRequest) -> Result<CreateOrderResponse> {
        if let Some(gsm) = request.buyer.gsm_number.as_deref() {
            if !gsm.is_empty() {
                request.buyer.gsm_number = Some(Validators::validate_gsm_number(gsm)?);
            }
        }

        let mut response: CreateOrderResponse =
            self.client
                .request("POST", "order/create", Some(&request))?;

        if let Some(reference_id) = response.reference_id.as_deref() {
            if !reference_id.is_empty() {
                // The order is created even when its checkout URL cannot be read
                if let Ok(url) = self.get_checkout_url(reference_id) {
                    response.checkout_url = Some(url);
                }
            }
        }
        Ok(response)
    }

    /// Retrieves an order by its reference ID
    pub fn get(&self, reference_id: &str) -> Result<Order> {
        let endpoint = format!("order/{}", path_escape(reference_id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Retrieves an order by its conversation ID. The server fills a subset of the order
    /// detail fields for this lookup.
    pub fn get_by_conversation_id(&self, conversation_id: &str) -> Result<OrderDetail> {
        let endpoint = format!("order/conversation/{}", path_escape(conversation_id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Gets order status by reference ID; the response is `{"status": ...}`
    pub fn get_status(&self, reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = format!("order/{}/status", path_escape(reference_id));
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Lists orders with pagination and an optional buyer filter
    pub fn list(
        &self,
        page: u32,
        per_page: u32,
        buyer_id: Option<String>,
    ) -> Result<serde_json::Value> {
        self.list_with_filter(&OrderListFilter {
            page,
            per_page,
            buyer_id,
            ..Default::default()
        })
    }

    /// Lists orders with every filter the order list endpoint accepts. The response is
    /// `{rows, total, page, per_page, total_pages}`.
    pub fn list_with_filter(&self, filter: &OrderListFilter) -> Result<serde_json::Value> {
        let endpoint = Query::new()
            .set("page", filter.page)
            .set("per_page", filter.per_page)
            .opt("start_date", filter.start_date.as_deref())
            .opt("end_date", filter.end_date.as_deref())
            .opt("organization_id", filter.organization_id.as_deref())
            .opt("conversation_id", filter.conversation_id.as_deref())
            .opt(
                "related_reference_id",
                filter.related_reference_id.as_deref(),
            )
            .opt("buyer_id", filter.buyer_id.as_deref())
            .opt("status", filter.status)
            .append_to("order/list");
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Lists the submerchants available for orders.
    pub fn list_submerchants(
        &self,
        page: u32,
        per_page: u32,
    ) -> Result<OrderSubmerchantListResponse> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .append_to("order/submerchants");
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Returns the payment details of an order.
    pub fn get_payment_details(&self, reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = format!("order/{}/payment-details", path_escape(reference_id));
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    /// Returns the payment details of the order with the given conversation ID.
    pub fn get_payment_details_by_conversation_id(
        &self,
        conversation_id: &str,
    ) -> Result<serde_json::Value> {
        let payload = serde_json::json!({ "conversation_id": conversation_id });
        self.client
            .make_request("POST", "order/payment-details", Some(&payload))
    }

    /// Lists the payments made against an order, with commission totals.
    pub fn get_payments(
        &self,
        request: GetOrderPaymentsRequest,
    ) -> Result<GetOrderPaymentsResponse> {
        self.client
            .request("POST", "order/payments", Some(&request))
    }

    /// Charges an order, for example with its saved card.
    pub fn charge(&self, request: OrderChargeRequest) -> Result<OrderChargeResponse> {
        self.client.request("POST", "order/charge", Some(&request))
    }

    /// Cancels an order
    pub fn cancel(&self, reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = "order/cancel";
        let payload = serde_json::json!({ "reference_id": reference_id });
        self.client.make_request("POST", endpoint, Some(&payload))
    }

    /// Refunds an order (full or partial); the response is `{status, message, is_success}`
    pub fn refund(&self, request: RefundOrderRequest) -> Result<serde_json::Value> {
        let endpoint = "order/refund";
        self.client.make_request("POST", endpoint, Some(&request))
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
        order
            .checkout_url
            .filter(|url| !url.is_empty())
            .ok_or_else(|| {
                crate::error::TapsilatError::InvalidResponse("Checkout URL not found".to_string())
            })
    }

    /// Adds a payment term to an existing order; `order_id` is the order's ID.
    pub fn create_term(
        &self,
        request: crate::types::OrderPaymentTermCreateDTO,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/term";
        self.client.make_request("POST", endpoint, Some(&request))
    }

    /// Replaces a payment term with the supplied values.
    pub fn update_term(
        &self,
        request: crate::types::OrderPaymentTermUpdateDTO,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/term";
        self.client.make_request("PATCH", endpoint, Some(&request))
    }

    /// Removes a payment term; `order_id` is the order's ID, not its reference ID.
    pub fn delete_term(
        &self,
        order_id: &str,
        term_reference_id: &str,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/term";
        let payload =
            serde_json::json!({ "order_id": order_id, "term_reference_id": term_reference_id });
        self.client.make_request("DELETE", endpoint, Some(&payload))
    }

    /// Refunds a payment made against a payment term.
    pub fn refund_term(
        &self,
        request: crate::types::OrderTermRefundRequest,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/term/refund";
        self.client.make_request("POST", endpoint, Some(&request))
    }

    #[deprecated(
        note = "the API has no term terminate route; this request always fails. Use terminate or delete_term"
    )]
    pub fn terminate_term(
        &self,
        term_reference_id: &str,
        reason: Option<String>,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/term/terminate";
        let mut payload = serde_json::Map::new();
        payload.insert(
            "term_reference_id".to_string(),
            serde_json::Value::String(term_reference_id.to_string()),
        );
        if let Some(r) = reason {
            payload.insert("reason".to_string(), serde_json::Value::String(r));
        }
        self.client.make_request("POST", endpoint, Some(&payload))
    }

    /// Returns a payment term by its term reference ID.
    pub fn get_term(&self, term_reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = Query::new()
            .set("term_reference_id", term_reference_id)
            .append_to("order/term");
        self.client.make_request::<()>("GET", &endpoint, None)
    }

    pub fn terminate(&self, reference_id: &str) -> Result<serde_json::Value> {
        let endpoint = "order/terminate";
        let payload = serde_json::json!({ "reference_id": reference_id });
        self.client.make_request("POST", endpoint, Some(&payload))
    }

    /// Asks the server to resend the order's callback.
    pub fn manual_callback(
        &self,
        reference_id: &str,
        conversation_id: Option<String>,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/callback";
        let mut payload = serde_json::Map::new();
        payload.insert(
            "reference_id".to_string(),
            serde_json::Value::String(reference_id.to_string()),
        );
        if let Some(cid) = conversation_id {
            payload.insert(
                "conversation_id".to_string(),
                serde_json::Value::String(cid),
            );
        }
        self.client.make_request("POST", endpoint, Some(&payload))
    }

    /// Links an order to a related reference ID.
    pub fn related_update(
        &self,
        reference_id: &str,
        related_reference_id: &str,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/releated";
        let payload = serde_json::json!({
            "reference_id": reference_id,
            "related_reference_id": related_reference_id
        });
        self.client.make_request("PATCH", endpoint, Some(&payload))
    }

    pub fn add_basket_item(&self, request: serde_json::Value) -> Result<serde_json::Value> {
        let endpoint = "order/basket-item";
        self.client.make_request("POST", endpoint, Some(&request))
    }

    pub fn remove_basket_item(
        &self,
        order_id: &str,
        basket_item_id: &str,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/basket-item";
        let payload = serde_json::json!({
            "order_reference_id": order_id,
            "basket_item_id": basket_item_id
        });
        self.client.make_request("DELETE", endpoint, Some(&payload))
    }

    pub fn update_basket_item(&self, request: serde_json::Value) -> Result<serde_json::Value> {
        let endpoint = "order/basket-item";
        self.client.make_request("PATCH", endpoint, Some(&request))
    }

    pub fn accounting(
        &self,
        request: crate::types::OrderAccountingRequest,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/accounting";
        self.client.make_request("POST", endpoint, Some(&request))
    }

    pub fn postauth(
        &self,
        request: crate::types::OrderPostAuthRequest,
    ) -> Result<serde_json::Value> {
        let endpoint = "order/postauth";
        self.client.make_request("POST", endpoint, Some(&request))
    }
}
