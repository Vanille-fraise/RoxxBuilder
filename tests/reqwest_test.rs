use reqwest;
use std::fs::File;
use std::io::prelude::*;
use roxx_builder::builder::data_mod::data_loader::DataLoader;

async fn simple_request(url: String) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(url).await?.text().await?)
}

#[actix_rt::test]
async fn google_request_ok() {
    let res = simple_request("https://www.google.fr/".to_string()).await;
    assert!(res.is_ok());
}

#[actix_rt::test]
async fn google_request_wrong() {
    let res = simple_request("https://www.googlefalsturlidontextists.fr/".to_string()).await;
    assert!(res.is_err())
}

#[actix_rt::test]
async fn request_image_and_save_it() {
    let res = reqwest::get("https://api.dofusdb.fr/img/monsters/6981.png").await;
    assert!(res.is_ok());
    let unwrapped_res = res.unwrap().bytes().await;
    assert!(unwrapped_res.is_ok());
    let unwrapped2_res = unwrapped_res.unwrap();
    let file = File::create("tests/test_files/turtle_image.png");
    assert!(file.is_ok());
    assert!(file.unwrap().write(&*unwrapped2_res).is_ok());
}

#[actix_rt::test]
async fn first_items_api_test() {
    let res = DataLoader::create_files_from_dofus_db_api_with_call_limit("tests/test_files/small_call_test/".to_string(), 120).await;
    assert!(res.is_ok());
}
