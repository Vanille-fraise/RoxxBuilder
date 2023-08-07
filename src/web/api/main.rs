use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::services::attack_service::post_attack;
use crate::web::api::roxx_builder::services::health_check_service::{empty_check, health_check};

#[actix_web::main]
#[test]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState::new());
    HttpServer::new(move || {
        App::new().app_data(app_data.clone()).
            service(health_check).service(empty_check).service(post_attack)
    }).bind(("localhost", 8008))?.run().await
}