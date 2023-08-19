use std::collections::HashMap;
use std::time::Duration;
use crate::builder::build_mod::build::Build;

#[derive(Debug, Clone)]
pub struct BuildSearchResult<'a> {
    pub eval: i64,
    pub build: Build<'a>,
    pub builds_evaluated: i64,
    pub search_time: Duration,
    pub best_build_position: i64,
    pub additional_data: HashMap<String, String>,
}

impl<'a> BuildSearchResult<'a> {
    pub fn new(eval: i64, build: Build<'a>, builds_evaluated: i64, search_time: Duration, best_build_position: i64, additional_data: HashMap<String, String>) -> Self {
        BuildSearchResult {
            eval,
            build,
            builds_evaluated,
            search_time,
            best_build_position,
            additional_data,
        }
    }

    pub fn empty() -> Self {
        Self::new(-1, Build::new(), 0, Duration::new(0, 0), -1, HashMap::new())
    }
}