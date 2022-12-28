use std::cmp::min;
use std::error::Error;
use std::fs::{File};
use std::io::{Write};
use serde_json::{Value};
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::item::Item;
use std::path::Path;
use crate::builder::item_mod::set::Set;

pub struct DataLoader;

impl DataLoader {
    pub async fn create_files_from_dofus_db_api_with_call_limit(path: String, limit: i64, api_section: String) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(&path)?;

        let client_builder = reqwest::Client::builder();
        let client = client_builder.build()?;
        let response = client.get(format!("https://api.dofusdb.fr/{}?$limit=0&$skip=0", api_section)).send().await?;
        let body_text = response.text().await?;
        let val: serde_json::Value = serde_json::from_str(body_text.as_str())?;
        let mut total: i64 = val["total"].as_i64().unwrap_or(-1);
        total = if limit < 0 { total } else { min(total, limit) };
        let mut i = 0;
        let step = 50;
        while i * step < total {
            let cur_response = client.get(format!("https://api.dofusdb.fr/{}?$limit={}&$skip={}", api_section, step, step *
                i)).send().await?;
            let cur_body_text = cur_response.text().await?;
            let mut cur_file = File::create(Path::new(&path).join(i.to_string()))?;
            cur_file.write(cur_body_text.as_bytes())?;
            i += 1;
        }
        Ok(())
    }

    pub async fn get_total_dofus_db_api(api_sections: Vec<String>) -> Result<Vec<i64>, Box<dyn Error>> {
        let mut res = vec![];
        let client_builder = reqwest::Client::builder();
        let client = client_builder.build()?;
        for api_section in api_sections {
            let response = client.get(format!("https://api.dofusdb.fr/{}?$limit=0&$skip=0", api_section)).send().await?;
            let body_text = response.text().await?;
            let val: serde_json::Value = serde_json::from_str(body_text.as_str())?;
            let total: i64 = val["total"].as_i64().unwrap_or(-1);
            res.push(total);
        };
        Ok(res)
    }

    pub fn from_api_response_files<'a>(files_path: Option<String>, sets_path: Option<String>) -> Result<DataContainer<'a>, std::io::Error> {
        let mut container = DataContainer::new();
        for opt_cur_path in vec![(files_path, true), (sets_path, false)] {
            if let (Some(cur_path), is_itm) = opt_cur_path {
                let dir = std::fs::read_dir(cur_path)?;
                for entry in dir {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        Self::add_data(&mut container, std::fs::read_to_string(path)?, is_itm);
                    }
                }
            }
        }
        Ok(container)
    }

    pub fn save_data_container(path: String, data_container: DataContainer) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write(serde_json::to_string(&data_container)?.as_bytes())?;
        Ok(())
    }

    pub fn from_data_container_file<'a>(path: String) -> Result<DataContainer<'a>, std::io::Error> {
        let container: DataContainer = serde_json::from_str(std::fs::read_to_string(path.as_str())?.as_str())?;

        return Ok(container);
    }

    fn add_data(data_container: &mut DataContainer, json_str: String, item: bool) {
        let dt = serde_json::from_str(json_str.as_str());
        if dt.is_ok() {
            let item_list_json: Value = dt.unwrap();
            let item_list = &item_list_json["data"].as_array();
            if let Some(itm_lst) = item_list {
                for itm in *itm_lst {
                    if item {
                        data_container.items.push(Item::from_serde_value(itm))
                    } else {
                        data_container.sets.push(Set::from_serde_value(itm))
                    }
                }
            }
        }
    }
}