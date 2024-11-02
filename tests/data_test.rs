use std::error::Error;
use serde::{Deserialize, Serialize};
use roxx_builder::builder::data_mod::data_loader::DataLoader;
use roxx_builder::builder::data_mod::data_manager::DataManager;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_type::ItemType;


const RELOAD: bool = true;
const MAKE_API_CALL: bool = false;

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
struct TestJson {
    id: usize,
    name: String,
}

#[test]
fn small_json_test() {
    let json_str = "{\"id\":10,\"name\":\"testwork\",\"notpresent\":false}";
    let parsed: TestJson = serde_json::from_str(json_str).unwrap();
    assert_eq!("TestJson { id: 10, name: \"testwork\" }", format!("{:?}", parsed));
}

#[test]
fn one_item_creation_from_json_test() -> Result<(), Box<dyn Error>> {
    let value: serde_json::Value = serde_json::from_str(std::fs::read_to_string("tests/test_files/first")?.as_str())?;
    let item_val = &value["data"][0];
    let created_item = Item::from_serde_value(item_val);
    assert_eq!(created_item.item_type, ItemType::Epee);
    assert_eq!(created_item.name, "Epée de Boisaille");
    Ok(())
}

#[test]
fn small_file_load_test() -> Result<(), Box<dyn Error>> {
    let container = DataLoader::from_api_response_files(Some("tests/test_files/small_convert_test".to_string()), None)?;
    assert_eq!(container.items.len(), 150);
    Ok(())
}

#[test]
fn save_data_container() -> Result<(), Box<dyn Error>> {
    let container = DataLoader::from_api_response_files(Some("tests/test_files/small_convert_test".to_string()), None)?;
    assert!(DataLoader::save_data_container("tests/test_files/containers/data_container_small".to_string(), &container).is_ok());
    Ok(())
}

#[test]
fn load_small_data_container() -> Result<(), Box<dyn Error>> {
    if RELOAD {
        let container = DataLoader::from_api_response_files(Some("tests/test_files/small_convert_test".to_string()), None)?;
        assert!(DataLoader::save_data_container("tests/test_files/containers/data_container_small_to_read".to_string(), &container).is_ok());
    }
    let container = DataLoader::from_data_container_file("tests/test_files/containers/data_container_small_to_read".to_string());
    assert!(container.is_ok());
    assert_eq!(container.unwrap().items.len(), 150);
    Ok(())
}

#[actix_rt::test]
async fn load_whole_data_container() -> Result<(), Box<dyn Error>> {
    if MAKE_API_CALL {
        let api_container = DataManager::retrieve_data().await;
        assert!(DataLoader::save_data_container("tests/test_files/containers/whole_data_container_to_read".to_string(), &api_container).is_ok());
    }
    if RELOAD {
        let brut_file_container = DataLoader::from_api_response_files(Some("tests/test_files/full_api_calls".to_string()), None)?;
        assert!(DataLoader::save_data_container("tests/test_files/containers/whole_data_container_to_read".to_string(), &brut_file_container).is_ok());
    }
    let container = DataLoader::from_data_container_file("tests/test_files/containers/whole_data_container_to_read".to_string());
    assert!(container.is_ok());
    assert_eq!(container.unwrap().items.len(), 18401);
    Ok(())
}

#[actix_rt::test]
async fn small_set_api_call() {
    let res = DataLoader::create_files_from_dofus_db_api_with_call_limit("tests/test_files/sets/small_api_call".to_string(), 20, "item-sets".to_string()).await;
    assert!(res.is_ok())
}

#[test]
fn small_set_gen_from_files() -> Result<(), Box<dyn Error>> {
    if RELOAD {
        let container = DataLoader::from_api_response_files(None, Some("tests/test_files/sets/small_api_call_to_read".to_string()))?;
        assert!(DataLoader::save_data_container("tests/test_files/containers/small_sets_data_container".to_string(), &container).is_ok());
    }
    let container = DataLoader::from_data_container_file("tests/test_files/containers/small_sets_data_container".to_string());
    assert!(container.is_ok());
    assert_eq!(container.unwrap().sets.len(), 150);
    Ok(())
}


