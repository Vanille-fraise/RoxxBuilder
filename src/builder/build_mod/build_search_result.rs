use std::time::Duration;
use crate::builder::build_mod::build::Build;
use crate::builder::item_mod::item::Item;

#[derive(Debug, Clone)]
pub struct BuildSearchResult<'a> {
    pub eval: i64,
    pub build: Build<'a>,
    pub last_item_tested: Option<&'a Item<'a>>,
    pub build_evaluated: i64,
    pub search_time: Duration,
    pub spares: i64,
    pub best_build_position: i64,
}

impl<'a> BuildSearchResult<'a> {
    pub fn new(eval: i64, build: Build<'a>, build_evaluated: i64, search_time: Duration, spares: i64, last_item_tested: Option<&'a Item<'a>>, best_build_position: i64) -> Self {
        BuildSearchResult {
            eval,
            build,
            build_evaluated,
            search_time,
            spares,
            last_item_tested,
            best_build_position,
        }
    }
}