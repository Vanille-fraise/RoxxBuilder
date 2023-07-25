use actix_web::{get, web, Responder, HttpResponse};

#[get("/roxx-builder")]
async fn health_test() -> impl Responder {
    HttpResponse::Ok().json("{message: 'I am healthy'}")
}