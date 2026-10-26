use crate::client::path_escape;
use crate::error::Result;
use crate::types::{ElementsSession, ElementsSessionCreateRequest, ElementsSessionWithSecret};
use std::sync::Arc;

/// Tapsilat Elements session operations.
///
/// Your backend creates a session with its secret API key and hands only the session's
/// `client_secret` to the app, which opens the native card sheet with it. Errors on these
/// routes carry a stable `code` such as `elements_not_enabled` or `session_not_found`, and
/// `retryable`, on [`TapsilatError::ApiError`](crate::TapsilatError::ApiError).
///
/// ```rust,no_run
/// use tapsilat::{ElementsSession, ElementsSessionCreateRequest, TapsilatClient};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = TapsilatClient::from_api_key("your-secret-api-key")?;
/// let created = client.elements().create_session(ElementsSessionCreateRequest {
///     purpose: ElementsSession::PURPOSE_PAY.to_string(),
///     order_reference_id: Some("reference-id".to_string()),
///     save_card: true,
///     ..Default::default()
/// })?;
/// let session_id = created.session.id.clone().unwrap_or_default();
/// // Send session_id and created.client_secret to your app. Do not log or store the secret.
///
/// // Later, read the outcome on your backend instead of trusting the app
/// let outcome = client.elements().get_session(&session_id)?;
/// if outcome.status.as_deref() == Some(ElementsSession::STATUS_SUCCEEDED) {
///     // fulfil the order; outcome.card holds the brand, last4 and expiry
/// }
/// # Ok(())
/// # }
/// ```
pub struct ElementsModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl ElementsModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Creates a session with `POST /elements/sessions`. A `pay` session needs
    /// `order_reference_id`; a `tokenize` session saves a card for `user_id`.
    ///
    /// Hand the returned `client_secret` to your app; it is returned only once. Do not log or
    /// store it.
    pub fn create_session(
        &self,
        request: ElementsSessionCreateRequest,
    ) -> Result<ElementsSessionWithSecret> {
        self.client
            .request("POST", "elements/sessions", Some(&request))
    }

    /// Returns the session's authoritative state and outcome with
    /// `GET /elements/sessions/{id}`. Read it on your backend before fulfilling, rather than
    /// trusting what the app reports.
    pub fn get_session(&self, id: &str) -> Result<ElementsSession> {
        let endpoint = format!("elements/sessions/{}", path_escape(id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Cancels a session that is still waiting for a card with
    /// `POST /elements/sessions/{id}/cancel`. A session that is confirming or has completed
    /// comes back unchanged.
    pub fn cancel_session(&self, id: &str) -> Result<ElementsSession> {
        let endpoint = format!("elements/sessions/{}/cancel", path_escape(id));
        self.client.request::<(), _>("POST", &endpoint, None)
    }
}
