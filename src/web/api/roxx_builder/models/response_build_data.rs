use std::io::empty;
use serde::Serialize;
use crate::builder::build_mod::build_search_result::BuildSearchResult;

#[derive(Serialize, Clone)]
pub struct ResponseBuildData {}

impl ResponseBuildData {
    pub fn empty() -> Self {
        ResponseBuildData {}
    }

    pub fn from_build_search_result(search_result: &BuildSearchResult) -> Self{
        // todo do the mapping
        return Self::empty()
    }
}