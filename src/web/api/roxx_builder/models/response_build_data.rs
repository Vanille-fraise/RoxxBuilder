use std::time::Duration;
use serde::Serialize;
use crate::builder::build_mod::build_search_result::BuildSearchResult;

#[derive(Serialize, Clone)]
pub struct ResponseBuildData {
    items_name: [String; 16],
    eval: i64,
    build_evaluated: i64,
    search_time: Duration,
    spares: i64,
    best_build_position: i64,
}

impl ResponseBuildData {
    pub fn new(items_name: [String; 16], eval: i64, build_evaluated: i64, search_time: Duration, spares: i64, best_build_position: i64) -> Self {
        ResponseBuildData {
            items_name,
            eval,
            build_evaluated,
            search_time,
            spares,
            best_build_position,
        }
    }
    
    pub fn from_build_search_result(search_result: &BuildSearchResult) -> Self {
        Self::new(search_result.build.items.map(|item| item.name.clone()),
                  search_result.eval,
                  search_result.build_evaluated,
                  search_result.search_time,
                  search_result.spares,
                  search_result.best_build_position,
        )
    }
}