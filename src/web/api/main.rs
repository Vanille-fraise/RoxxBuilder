use actix_web::{web, App, HttpServer};
use crate::builder::data_mod::data_manager::DataManager;
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::services::attack_service::post_attack;
use crate::web::api::roxx_builder::services::health_check_service::{empty_check, health_check};

pub async fn api_main() -> std::io::Result<()> {
    let base_data_container = DataManager::retrieve_data().await;
    let app_data = web::Data::new(AppState::new(base_data_container));
    let port = 8008;
    let url = "0.0.0.0";
    println!("Data loaded, starting server on {}, port {}", url, port);
    HttpServer::new(move || {
        App::new().app_data(app_data.clone()).
            service(health_check).service(empty_check).service(post_attack)
    }).bind((url, port))?.run().await
}
