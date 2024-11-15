use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::data_mod::data_loader::DataLoader;
use std::path::Path;
use std::time::Instant;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct DataManager<'a> {
    reload: bool,
    path: &'a str,
    clear_unknown_items: bool,
    clear_not_breeds_attacks: bool,
}

#[derive(Debug, EnumIter)]
pub enum DataType {
    Item,
    ItemSet,
    Breed,
    SpellVariant,
    // Spell level is internally attack
    SpellLevel,
    Version,
}

impl DataType {
    fn str_rep(&self) -> &'static str {
        match *self {
            DataType::Item => "items",
            DataType::ItemSet => "item-sets",
            DataType::Breed => "breeds",
            DataType::SpellLevel => "spell-levels",
            DataType::SpellVariant => "spell-variants",
            DataType::Version => "version",
        }
    }

    pub fn call_to_api_folder_path(&self) -> String {
        Path::new(DATA_MANAGER.path)
            .join("call_to_api")
            .join(self.str_rep())
            .to_string_lossy()
            .to_string()
    }

    pub fn dofus_db_api_argument(&self) -> String {
        self.str_rep().to_string()
    }

    pub fn data_types_to_load() -> Vec<DataType> {
        DataType::iter().collect()
    }

    pub fn is_raw_data(&self) -> bool {
        match self {
            DataType::Item
            | DataType::ItemSet
            | DataType::Breed
            | DataType::SpellVariant
            | DataType::SpellLevel => false,
            DataType::Version => true,
        }
    }
}

static DATA_MANAGER: DataManager = DataManager {
    reload: true,
    path: "resource/data/",
    clear_unknown_items: true,
    clear_not_breeds_attacks: true,
};

impl<'a> DataManager<'a> {
    pub async fn retrieve_data() -> DataContainer {
        let folder_data_container = Path::new(DATA_MANAGER.path)
            .join("data_container")
            .join("container")
            .to_string_lossy()
            .to_string();

        for data_type in DataType::data_types_to_load() {
            let _res_items =
                std::fs::create_dir_all(Path::new(&data_type.call_to_api_folder_path()));
        }
        let _res_container =
            std::fs::create_dir_all(Path::new(&folder_data_container).parent().unwrap());

        let start = Instant::now();
        println!("Building data container.");
        let mut res_dc = DataLoader::from_data_container_file(folder_data_container.clone());
        if res_dc.is_err() {
            res_dc = res_dc.inspect_err(|e| {
                println!("Error while building data container from dc file: {}", e)
            });
        }
        let from_data_container_file = res_dc.is_ok();
        // Note: If the container is buit from API response file, the version will not be saved and everything will be fetched again.
        let mut dc = res_dc.unwrap_or(
            DataLoader::from_api_response_files(DataType::data_types_to_load())
                .unwrap_or(DataContainer::new()),
        );
        println!("Data container built in {:?}.", start.elapsed());
        let mut reloaded_from_api = false;
        if DATA_MANAGER.reload {
            let tots = DataLoader::get_version_dofus_db_api().await;
            match tots {
                Ok(version) => {
                    if dc.version != version {
                        println!("Every data needs to be fetched from DofusDB for the new version {} => {}!", dc.version, version);
                        reloaded_from_api = true;
                        for data_type in DataType::data_types_to_load() {
                            let fetch_start = Instant::now();
                            let _r_files =
                                DataLoader::create_files_from_dofus_db_api_with_call_limit(
                                    data_type.call_to_api_folder_path(),
                                    if data_type.is_raw_data() { 1 } else { -1 },
                                    data_type.dofus_db_api_argument().to_string(),
                                    data_type.is_raw_data()
                                )
                                .await;
                            println!("Fetched {:?} in {:?}.", data_type, fetch_start.elapsed())
                        }
                        println!("Building data container with new data.");
                        dc = DataLoader::from_api_response_files(DataType::data_types_to_load())
                            .unwrap_or(DataContainer::new());
                        dc.version = version;
                    }
                }
                Err(_) => {}
            }
        }
        if !from_data_container_file || reloaded_from_api {
            if DATA_MANAGER.clear_unknown_items {
                dc.clear_unknown_type_items();
            }
            if DATA_MANAGER.clear_not_breeds_attacks {
                dc.clear_not_breeds_attacks();
            }
            let save_res = DataLoader::save_data_container(folder_data_container.clone(), &dc);
            if save_res.is_err() {
                println!("Could not save the new loaded files");
            } else {
                println!("New data container built.");
            }
        }
        dc.link_sub_data();
        println!("Data container version: {}", dc.version);
        return dc;
    }
}
