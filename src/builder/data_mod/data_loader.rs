use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::set::Set;
use serde_json::Value;
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::data_manager::DataType;

pub struct DataLoader;

impl DataLoader {
    pub async fn create_files_from_dofus_db_api_with_call_limit(
        path: String,
        limit: i64,
        api_section: String,
        is_raw: bool,
    ) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(&path)?;

        let client_builder = reqwest::Client::builder();
        let client = client_builder.build()?;
        let mut total: i64 = limit;
        if !is_raw {
            let response = client
                .get(format!(
                    "https://api.dofusdb.fr/{}?$limit=0&$skip=0",
                    api_section
                ))
                .send()
                .await?;
            let body_text = response.text().await?;
            let val: Value = serde_json::from_str(body_text.as_str())?;
            total = val["total"].as_i64().unwrap_or(-1);
            total = if limit < 0 { total } else { min(total, limit) };
        }
        let mut i = 0;
        let step = 50;
        while i * step < total {
            let cur_response = client
                .get(format!(
                    "https://api.dofusdb.fr/{}?$limit={}&$skip={}",
                    api_section,
                    step,
                    step * i
                ))
                .send()
                .await?;
            let cur_body_text = cur_response.text().await?;
            let mut cur_file = File::create(Path::new(&path).join(i.to_string()))?;
            cur_file.write(cur_body_text.as_bytes())?;
            i += 1;
        }
        Ok(())
    }

    pub async fn get_total_dofus_db_api(
        api_sections: Vec<String>,
    ) -> Result<Vec<i64>, Box<dyn Error>> {
        let mut res = vec![];
        let client_builder = reqwest::Client::builder();
        let client = client_builder.build()?;
        for api_section in api_sections {
            let response = client
                .get(format!(
                    "https://api.dofusdb.fr/{}?$limit=0&$skip=0",
                    api_section
                ))
                .send()
                .await?;
            let body_text = response.text().await?;
            let val: Value = serde_json::from_str(body_text.as_str())?;
            let total: i64 = val["total"].as_i64().unwrap_or(-1);
            res.push(total);
        }
        Ok(res)
    }

    pub async fn get_version_dofus_db_api() -> Result<String, Box<dyn Error>> {
        let client_builder = reqwest::Client::builder();
        let client = client_builder.build()?;
        let response = client.get("https://api.dofusdb.fr/version").send().await?;
        let body_text = response.text().await?;
        Ok(body_text)
    }

    #[deprecated(note = "For testing purpose ONLY.")]
    pub fn from_api_response_files_item_and_sets_only<'a>(
        items_path: Option<String>,
        sets_path: Option<String>,
    ) -> Result<DataContainer, std::io::Error> {
        let mut container = DataContainer::new();
        for opt_cur_path in vec![(items_path, DataType::Item), (sets_path, DataType::ItemSet)] {
            if let (Some(cur_path), data_type) = opt_cur_path {
                let dir = std::fs::read_dir(cur_path)?;
                for entry in dir {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        Self::add_data(&mut container, std::fs::read_to_string(path)?, &data_type);
                    }
                }
            }
        }
        Ok(container)
    }

    pub fn from_api_response_files<'a>(
        data_type_to_load: Vec<DataType>,
    ) -> Result<DataContainer, std::io::Error> {
        let mut container = DataContainer::new();
        for data_type in data_type_to_load {
            let dir = std::fs::read_dir(data_type.call_to_api_folder_path())?;
            for entry in dir {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    Self::add_data(&mut container, std::fs::read_to_string(path)?, &data_type);
                }
            }
        }
        Ok(container)
    }

    pub fn save_data_container(
        path: String,
        data_container: &DataContainer,
    ) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write(serde_json::to_string(&data_container)?.as_bytes())?;
        Ok(())
    }

    pub fn from_data_container_file<'a>(path: String) -> Result<DataContainer, std::io::Error> {
        let container: DataContainer =
            serde_json::from_str(std::fs::read_to_string(path.as_str())?.as_str())?;
        return Ok(container);
    }

    fn add_data(data_container: &mut DataContainer, json_str: String, data_type: &DataType) {
        let mut obj_list =
        if data_type.is_raw_data() {
            vec![Value::String(json_str)]
        } else {
            let dt = serde_json::from_str(json_str.as_str());
            if dt.is_err() {
                return;
            }
            let json_object_list: Value = dt.unwrap();
            let opt_obejct_list = json_object_list["data"].as_array();
            if opt_obejct_list.is_none() {
                return;
            }
            opt_obejct_list.unwrap().to_owned()
        };

        while obj_list.len() > 0 {
            let obj = obj_list.pop().unwrap();
            match data_type {
                DataType::Item => data_container.add_item_from_value(obj),
                DataType::ItemSet => data_container.sets.push(Set::from_serde_value(&obj).into()),
                DataType::Breed => data_container.breeds.push(serde_json::from_value(obj).unwrap()),
                DataType::SpellVariant => data_container.spell_variants.push(serde_json::from_value(obj).unwrap()),
                DataType::SpellLevel => data_container.attacks.push(serde_json::from_value(obj).unwrap()),
                DataType::Version => data_container.version = obj.as_str().unwrap().to_string(),
            }
        }
    }
}
