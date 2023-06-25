use std::path::Path;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::data_mod::data_loader::DataLoader;

pub struct DataManager<'a> {
    reload: bool,
    path: &'a str,
}

static mut DATA_MANAGER: DataManager = DataManager { reload: true, path: "resource/data/" };


impl<'a> DataManager<'a> {
    pub async fn retrieve_data() -> DataContainer<'a> {
        let folder_api_call = "call_to_api";
        let folder_data_container = "data_container";
        unsafe {
            let res_dc = DataLoader::from_data_container_file(Path::new(DATA_MANAGER.path).join(folder_data_container).to_string_lossy().to_string());
            let mut dc = res_dc.unwrap_or(
                DataLoader::from_api_response_files(Some(Path::new(DATA_MANAGER.path).join(folder_api_call).join("items").to_string_lossy().to_string()),
                                                    Some(Path::new(DATA_MANAGER.path).join(folder_api_call).join("sets").to_string_lossy().to_string()))
                    .unwrap_or(DataContainer::new())
            );
            if DATA_MANAGER.reload {
                let tots = DataLoader::get_total_dofus_db_api(vec!["items".to_string(), "sets".to_string()]).await;
                match tots {
                    Ok(res) => {
                        let mut changed = false;
                        if res[0] != dc.items.len() as i64 {
                            let r = DataLoader::create_files_from_dofus_db_api_with_call_limit(Path::new(DATA_MANAGER.path).to_string_lossy().to_string(), -1, "items".to_string()).await;
                            if r.is_ok() {
                                changed = true;
                            }
                        } else if res[1] != dc.sets.len() as i64 {
                            let r = DataLoader::create_files_from_dofus_db_api_with_call_limit(Path::new(DATA_MANAGER.path).to_string_lossy().to_string(), -1, "sets".to_string()).await;
                            if r.is_ok() {
                                changed = true;
                            }
                        }
                        if changed {
                            dc = DataLoader::from_api_response_files(Some(Path::new(DATA_MANAGER.path).join(folder_api_call).join("items").to_string_lossy().to_string()),
                                                                     Some(Path::new(DATA_MANAGER.path).join(folder_api_call).join("sets").to_string_lossy().to_string()))
                                .unwrap_or(DataContainer::new())
                        }
                    }
                    Err(_) => {}
                }
                DATA_MANAGER.reload = false;
            }
            return dc;
        }
    }
}
