use actix_web::{post, web, Responder, HttpResponse};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::models::post_attack_data::PostAttackData;

#[post("/roxx-builder/attack")]
async fn post_attack(app_data: web::Data<AppState<'_>>, attack_data: web::Json<PostAttackData>) -> impl Responder {
    HttpResponse::Ok().json(app_data.find_build(attack_data.into_inner().attack().clone()))
}