use crate::client::{path_escape, Query};
use crate::error::Result;
use crate::types::{
    CardTokenizeRequest, CardTokenizeResponse, DeleteSavedCardResponse, ListSavedCardsResponse,
};
use std::sync::Arc;

/// Saved card (tokenization) operations.
pub struct TokenizationModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl TokenizationModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    /// Tokenizes a card and returns 3D secure form details when required.
    pub fn tokenize_card(&self, request: CardTokenizeRequest) -> Result<CardTokenizeResponse> {
        self.client
            .request("POST", "tokenization/card/tokenize", Some(&request))
    }

    pub fn list_saved_cards(&self, page: u32, per_page: u32) -> Result<ListSavedCardsResponse> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .append_to("tokenization/card/list");
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    pub fn delete_saved_card(&self, id: &str) -> Result<DeleteSavedCardResponse> {
        let endpoint = format!("tokenization/card/{}", path_escape(id));
        self.client.request::<(), _>("DELETE", &endpoint, None)
    }
}
