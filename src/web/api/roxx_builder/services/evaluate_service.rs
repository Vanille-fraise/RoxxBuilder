use actix_web::{post, web, Responder, HttpResponse};
use crate::web::api::app_state::AppState;
use crate::web::api::roxx_builder::models::post_attack_data::PostAttackData;
use crate::web::api::roxx_builder::services::response_wrapper;

#[post("/api/evaluate-attack")]
pub async fn post_evaluate_attack(app_data: web::Data<AppState>, attack_data: web::Json<PostAttackData>) -> impl Responder {
    println!("< New evaluation --");
    let mut attack = attack_data.into_inner().attack;
    attack.fix_damage_lines_and_crit_lines();
    let build = app_data.find_build(attack);
    println!("                   -- evaluation accomplished >");
    let response = response_wrapper::ResponseWrapper { status: "success".to_string(), data: build };
    HttpResponse::Ok().json(response)
}
