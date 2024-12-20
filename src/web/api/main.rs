use crate::builder::data_mod::data_manager::DataManager;
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::services::attack_list_service::get_attacks;
use crate::web::api::roxx_builder::services::breed_list_service::get_breeds;
use crate::web::api::roxx_builder::services::evaluate_service::post_evaluate_attack;
use crate::web::api::roxx_builder::services::health_check_service::{empty_check, health_check};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

pub async fn api_main() -> std::io::Result<()> {
    let base_data_container = DataManager::retrieve_data().await;
    let app_data = web::Data::new(AppState::new(base_data_container));
    let port = 8008;
    let url = "localhost";
    println!("Data loaded, starting server on {}, port {}", url, port);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(empty_check).wrap(Cors::permissive())
            .service(health_check).wrap(Cors::permissive())
            .service(post_evaluate_attack)
            .service(get_attacks)
            .service(get_breeds)
    })
    .bind((url, port))?
    .run()
    .await
}
