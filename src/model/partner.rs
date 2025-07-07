use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct PartnerConfigCondition {
    pub throughput_from: Option<f64>,
    pub throughput_to: Option<f64>,

    pub packet_loss_from: Option<f64>,
    pub packet_loss_to: Option<f64>,

    pub jitter_from: Option<u128>,
    pub jitter_to: Option<u128>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PartnerConfig {
    pub id: String,
    pub partner_name: String,
    pub conditions: Option<PartnerConfigCondition>,
}

#[derive(Serialize)]
pub struct PartnerSLAFilterPath {
    pub partner_name: String,
}
