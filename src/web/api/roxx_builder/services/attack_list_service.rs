use actix_web::{get, web, Responder, HttpResponse};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::services::response_wrapper::ResponseWrapper;

#[get("/api/attacks")]
pub async fn get_attacks(app_data: web::Data<AppState>) -> impl Responder {
    println!("<Providing attacks>");
    let result = &app_data.data_container.spell_variants;
    HttpResponse::Ok().json(ResponseWrapper {status: "succes".to_string(), data: &result})
}
