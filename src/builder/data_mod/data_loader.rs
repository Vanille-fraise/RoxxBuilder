use std::error::Error;
use std::fs::{File};
use std::io::{Write};
use crate::builder::data_mod::data_container::DataContainer;

pub struct DataLoader;

impl DataLoader {
    pub async fn create_files_from_dofus_db_api(path: String) -> Result<(), Box<dyn Error>> {
        let client_builder = reqwest::Client::builder();
        let client = client_builder.build()?;
        let response = client.get("https://api.dofusdb.fr/items?$limit=50&$skip=0").send().await?;
        let mut my_file = File::create(path)?;
        let body_text = response.text().await?;
        my_file.write(body_text.as_bytes())?;
        let whole_json_answer: serde_json::Value = serde_json::from_str(body_text.as_str())?;
        let _data = &whole_json_answer["data"];
        Ok(())
    }

    pub fn from_resources_file(_path: String) -> Result<DataContainer, std::io::Error> {
        todo!()
    }

    pub fn from_api_dofus_db() -> Result<DataContainer, reqwest::Error> {
        todo!()
    }


    #[allow(dead_code)]
    fn parse_items_data(data_container: DataContainer, json_str: String) -> DataContainer {
        let mut _whole_data: serde_json::Value;
        if let Ok(dt) = serde_json::from_str(json_str.as_str()) {
            _whole_data = dt;
        } else { return data_container; }

        data_container
    }
}