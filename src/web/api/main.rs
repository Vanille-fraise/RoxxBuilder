use actix_web::{get, web, App, Responder, HttpResponse};
use crate::web::api::app_state::AppState;

#[actix_web::main]
async fn main() {
    let app_data = web::Data::new(AppState::new());
}