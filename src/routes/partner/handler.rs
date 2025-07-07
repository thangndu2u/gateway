use actix_web::{HttpResponse, get, web};
use serde_json::json;

use crate::{
    bootstrap::SharedState, model::partner::PartnerConfigCondition,
    services::partner_service::PartnerService,
};

#[get("/sla/filter-provider")]
pub async fn partner_sla_filter(
    query: web::Query<PartnerConfigCondition>,
    app_state: web::Data<SharedState>,
) -> HttpResponse {
    let result = app_state
        .partner_svc
        .filter_partners_info(query.into_inner())
        .await;

    HttpResponse::Ok().json(json!(result))
}
