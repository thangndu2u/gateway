use std::sync::Arc;

use crate::services::partner_service::PartnerServiceImpl;

pub struct AppState {
    pub partner_svc: Arc<PartnerServiceImpl>,
}

impl AppState {
    pub fn new(partner_svc: Arc<PartnerServiceImpl>) -> Self {
        Self { partner_svc }
    }
}

pub type SharedState = AppState;
