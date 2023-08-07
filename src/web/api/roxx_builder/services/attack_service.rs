use std::thread;
use std::time::{Duration, Instant};
use actix_web::{post, web, Responder, HttpResponse};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::models::post_attack_data::PostAttackData;

#[post("/roxx-builder/attack")]
pub async fn post_attack<'a>(app_data: web::Data<AppState<'a>>, attack_data: web::Json<PostAttackData>) -> impl Responder {
    let build = app_data.find_build(attack_data.attack().clone());
    HttpResponse::Ok().json(build)
}