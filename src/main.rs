#[actix_web::main]
async fn main() -> std::io::Result<()> {
    roxx_builder::web::api::main::api_main().await
}
