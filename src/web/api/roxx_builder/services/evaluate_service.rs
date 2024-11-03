use actix_web::{post, web, Responder, HttpResponse};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::models::post_attack_data::PostAttackData;

#[post("/api/evaluate")]
pub async fn post_evaluate(app_data: web::Data<AppState>, attack_data: web::Json<PostAttackData>) -> impl Responder {
    println!("< New evaluation --");
    let build = app_data.find_build(attack_data.attack().clone());
    println!("                   -- evaluation accomplished >");
    HttpResponse::Ok().json(build)
}