/* 
use actix_web::{get, web, Responder, HttpResponse};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::models::post_attack_data::PostAttackData;

#[get("/api/attacks")]
pub async fn post_attack(app_data: web::Data<AppState>) -> impl Responder {

    HttpResponse::Ok().json("{}")
}

*/