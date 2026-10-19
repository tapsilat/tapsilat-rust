use crate::client::{path_escape, Query};
use crate::error::Result;
use crate::types::{
    CardSchemeListResponse, Vpos, VposAcquirerListResponse, VposAcquirerTemplateListResponse,
    VposCreateRequest, VposListFilter, VposListResponse, VposMutationResponse, VposSubmerchant,
    VposSubmerchantCreateRequest, VposSubmerchantListFilter, VposSubmerchantListResponse,
    VposSubmerchantUpdateRequest, VposUpdateRequest,
};
use std::sync::Arc;

/// Virtual POS operations.
pub struct VposModule {
    client: Arc<crate::client::TapsilatClient>,
}

impl VposModule {
    pub fn new(client: Arc<crate::client::TapsilatClient>) -> Self {
        Self { client }
    }

    pub fn list(&self, page: u32, per_page: u32) -> Result<VposListResponse> {
        self.list_with_filter(page, per_page, &VposListFilter::default())
    }

    pub fn list_with_filter(
        &self,
        page: u32,
        per_page: u32,
        filter: &VposListFilter,
    ) -> Result<VposListResponse> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .opt("suborganization_id", filter.suborganization_id.as_deref())
            .append_to("vpos");
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Creates a VPOS. `currencies` accepts currency UUIDs or units such as "TRY".
    pub fn create(&self, request: VposCreateRequest) -> Result<VposMutationResponse> {
        self.client.request("POST", "vpos", Some(&request))
    }

    pub fn get(&self, id: &str) -> Result<Vpos> {
        let endpoint = format!("vpos/{}", path_escape(id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    /// Replaces a VPOS configuration. Fields left at their default are stored as such, so
    /// read the VPOS with [`VposModule::get`] and send every field back.
    pub fn update(&self, id: &str, request: VposUpdateRequest) -> Result<VposMutationResponse> {
        let endpoint = format!("vpos/{}", path_escape(id));
        self.client.request("PATCH", &endpoint, Some(&request))
    }

    pub fn delete(&self, id: &str) -> Result<VposMutationResponse> {
        let endpoint = format!("vpos/{}", path_escape(id));
        self.client.request::<(), _>("DELETE", &endpoint, None)
    }

    pub fn list_acquirers(&self) -> Result<VposAcquirerListResponse> {
        self.client.request::<(), _>("GET", "vpos/acquirers", None)
    }

    pub fn list_card_schemes(&self) -> Result<CardSchemeListResponse> {
        self.client
            .request::<(), _>("GET", "vpos/card-schemes", None)
    }

    pub fn list_acquirer_templates(&self) -> Result<VposAcquirerTemplateListResponse> {
        self.client
            .request::<(), _>("GET", "vpos/acquirer-templates", None)
    }

    pub fn list_submerchants(
        &self,
        page: u32,
        per_page: u32,
        filter: &VposSubmerchantListFilter,
    ) -> Result<VposSubmerchantListResponse> {
        let endpoint = Query::new()
            .set("page", page)
            .set("per_page", per_page)
            .opt("vpos_id", filter.vpos_id.as_deref())
            .opt(
                "external_reference_id",
                filter.external_reference_id.as_deref(),
            )
            .append_to("vpos-submerchant");
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    pub fn create_submerchant(
        &self,
        request: VposSubmerchantCreateRequest,
    ) -> Result<VposMutationResponse> {
        self.client
            .request("POST", "vpos-submerchant", Some(&request))
    }

    /// Returns a VPOS submerchant; the server answers 404 when it does not exist.
    pub fn get_submerchant(&self, id: &str) -> Result<VposSubmerchant> {
        let endpoint = format!("vpos-submerchant/{}", path_escape(id));
        self.client.request::<(), _>("GET", &endpoint, None)
    }

    pub fn update_submerchant(
        &self,
        id: &str,
        request: VposSubmerchantUpdateRequest,
    ) -> Result<VposMutationResponse> {
        let endpoint = format!("vpos-submerchant/{}", path_escape(id));
        self.client.request("PATCH", &endpoint, Some(&request))
    }

    pub fn delete_submerchant(&self, id: &str) -> Result<VposMutationResponse> {
        let endpoint = format!("vpos-submerchant/{}", path_escape(id));
        self.client.request::<(), _>("DELETE", &endpoint, None)
    }
}
