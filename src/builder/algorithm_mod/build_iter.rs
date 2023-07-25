use crate::builder::attack_mod::attack::Attack;
use crate::builder::build_mod::build::Build;
use crate::builder::data_mod::data_container::DataContainer;

pub trait BuildIterator<'a> {
    fn new(container: &'a DataContainer, attack: &'a Attack) -> Self;
    fn next_build(&mut self) -> Option<&Build>;
}