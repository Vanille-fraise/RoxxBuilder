use std::collections::HashMap;
use std::time::Duration;
use crate::builder::build_mod::build::Build;

#[derive(Debug, Clone)]
pub struct BuildSearchResult<'a> {
    pub eval: i64,
    pub build: Build<'a>,
    pub build_evaluated: i64,
    pub search_time: Duration,
    pub spares: i64,
    pub best_build_position: i64,
    pub additional_data: HashMap<String, String>,
}

impl<'a> BuildSearchResult<'a> {
    pub fn new(eval: i64, build: Build<'a>, build_evaluated: i64, search_time: Duration, spares: i64, best_build_position: i64) -> Self {
        BuildSearchResult {
            eval,
            build,
            build_evaluated,
            search_time,
            spares,
            best_build_position,
            additional_data: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self::new(-1, Build::new(), 0, Duration::new(0, 0), -1, -1)
    }
}