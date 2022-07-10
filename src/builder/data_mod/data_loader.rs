use crate::builder::data_mod::data_container::DataContainer;

pub struct DataLoader;

impl DataLoader {
    pub fn from_resources_files() -> Result<DataContainer,  std::io::Error> {
        Ok(DataContainer::new())
    }

    pub fn from_api_dofus_db() -> Result<DataContainer, reqwest::Error> {
        Ok(DataContainer::new())
    }
}