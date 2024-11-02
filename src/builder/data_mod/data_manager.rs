use std::path::Path;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::data_mod::data_loader::DataLoader;

pub struct DataManager<'a> {
    reload: bool,
    path: &'a str,
    clear_unknown: bool,
}

static DATA_MANAGER: DataManager = DataManager { reload: true, path: "resource/data/", clear_unknown: true };


impl<'a> DataManager<'a> {
    pub async fn retrieve_data() -> DataContainer {
        let folder_api_call = Path::new(DATA_MANAGER.path).join("call_to_api");
        let folder_api_items = folder_api_call.clone().join("items").to_string_lossy().to_string();
        let folder_api_sets = folder_api_call.clone().join("item-sets").to_string_lossy().to_string();
        let folder_data_container = Path::new(DATA_MANAGER.path).join("data_container").join("container").to_string_lossy().to_string();
        let _res_items = std::fs::create_dir_all(Path::new(&folder_api_items));
        let _res_sets = std::fs::create_dir_all(Path::new(&folder_api_sets));
        let _res_container = std::fs::create_dir_all(Path::new(&folder_data_container).parent().unwrap());

        let res_dc = DataLoader::from_data_container_file(folder_data_container.clone());
        let mut dc = res_dc.unwrap_or(
            DataLoader::from_api_response_files(Some(folder_api_items.clone()),
                                                Some(folder_api_sets.clone()))
                .unwrap_or(DataContainer::new())
        );
        if DATA_MANAGER.reload {
            let tots = DataLoader::get_total_dofus_db_api(vec!["items".to_string(), "item-sets".to_string()]).await;
            match tots {
                Ok(res) => {
                    let mut changed = false;
                    if res[0] + res[1] != dc.reference_size as i64 {
                        let r_items = DataLoader::create_files_from_dofus_db_api_with_call_limit(folder_api_items.clone(), -1, "items".to_string()).await;
                        changed = r_items.is_ok();
                        let r_sets = DataLoader::create_files_from_dofus_db_api_with_call_limit(folder_api_sets.clone(), -1, "item-sets".to_string()).await;
                        changed = changed && r_sets.is_ok();
                    }
                    if changed {
                        println!("Must reload data from DofusDB API");
                        dc = DataLoader::from_api_response_files(Some(folder_api_items.clone().to_string()),
                                                                 Some(folder_api_sets.clone()))
                            .unwrap_or(DataContainer::new());
                        if DATA_MANAGER.clear_unknown {
                            dc.clear_unknown_type();
                        }
                        let save_res = DataLoader::save_data_container(folder_data_container.clone(), &dc);
                        if save_res.is_err() {
                            println!("Could not save the new loaded files");
                        }
                    }
                }
                Err(_) => {}
            }
        }
        return dc;
    }
}
