use std::time::Duration;
use crate::builder::build_mod::build::Build;
use crate::builder::item_mod::item::Item;

#[derive(Debug)]
pub struct DamageEval<'a> {
    pub min: i64,
    pub average: i64,
    pub max: i64,
    pub build: Build<'a>,
    pub last_item_tested: &'a Item<'a>,
    pub build_evaluated: i64,
    pub search_time: Duration,
    pub spares: i64,
}

impl<'a> DamageEval<'a> {
    pub fn new(eval: (i64, i64, i64), build: Build<'a>, build_evaluated: i64, search_time: Duration, spares: i64, last_item_tested: &'a Item<'a>) -> Self {
        DamageEval {
            min: eval.0,
            average: eval.1,
            max: eval.2,
            build,
            build_evaluated,
            search_time,
            spares,
            last_item_tested,
        }
    }
}