use crate::builder::algorithm_mod::build_generator::BuildGenerator;
use crate::builder::algorithm_mod::build_iter::BuildIterator;
use crate::builder::algorithm_mod::item_estimator::ItemEstimator;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::data_mod::data_container::DataContainer;

pub struct BuildIteratorFactory {}

impl BuildIteratorFactory {
    pub fn new_iterative_factory() -> Self {
        BuildIteratorFactory {}
    }

    pub fn create<'a>(&'a self, container: &'a DataContainer, attack: &'a Attack) -> Box<dyn BuildIterator + '_> {
        let items = ItemEstimator::roxx_based_estimation(container, attack);
        let sets = container.sets.iter().collect();
        let build_finder = BuildGenerator::new(items, sets);
        Box::new(build_finder)
    }
}