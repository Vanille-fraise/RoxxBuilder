use actix_web::{post, web, Responder, HttpResponse};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::models::post_attack_data::PostAttackData;

#[post("/roxx-builder/attack")]
pub async fn post_attack(app_data: web::Data<AppState>, attack_data: web::Json<PostAttackData>) -> impl Responder {
    println!("New request --");
    let build = app_data.find_build(attack_data.attack().clone());
    println!(" -- Request finished");
    HttpResponse::Ok().json(build)
}