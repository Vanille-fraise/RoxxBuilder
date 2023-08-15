use std::thread;
use std::time::Duration;
use actix_web::{get, Responder, HttpResponse};

#[get("/roxx-builder")]
pub async fn health_check() -> impl Responder {
    thread::sleep(Duration::from_secs(3));
    HttpResponse::Ok().json("{message: 'I am healthy'}")
}

#[get("/")]
pub async fn empty_check() -> impl Responder {
    HttpResponse::Ok().json("Empty")
}