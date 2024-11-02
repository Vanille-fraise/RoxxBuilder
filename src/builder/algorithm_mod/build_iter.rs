use crate::builder::build_mod::build::Build;

pub trait BuildIterator {
    fn next_build(&mut self) -> Option<&Build>;
    fn last_item_id(&self) -> Option<i64> {
        return None;
    }
}