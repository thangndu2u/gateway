use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, middleware, web};
use std::sync::Arc;
pub use tracing::*;

use crate::{
    bootstrap::AppState, services::partner_service::PartnerServiceImpl, tasks::start_cronjob,
    utils::config::APP_CONFIG,
};

pub mod bootstrap;
pub mod model;
pub mod repository;
pub mod routes;
pub mod services;
pub mod tasks;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!(
        "starting HTTP server at http://localhost:{}",
        APP_CONFIG.app_port
    );

    let partner_svc = Arc::new(PartnerServiceImpl::new());

    let arc_state = Arc::new(AppState::new(partner_svc.clone()));

    tokio::spawn(async move {
        start_cronjob(partner_svc).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(arc_state.clone()))
            .configure(routes::partner::router_config)
            .wrap(
                Cors::default()
                    //TODO: remove allow_any_origin when deployment
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    ])
                    // .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
    })
    .workers(8)
    .bind(("127.0.0.1", APP_CONFIG.app_port as u16))?
    .run()
    .await
}
