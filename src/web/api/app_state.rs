use std::sync::Mutex;
use crate::builder::algorithm_mod::roxx_build_finder::RoxxBuildFinder;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::data_mod::data_container::DataContainer;
use crate::web::api::roxx_builder::data::data_handler::DataHandler;
use crate::web::api::roxx_builder::models::response_build_data::ResponseBuildData;

pub struct AppState {
    pub roxx_build_finder: Mutex<RoxxBuildFinder>,
    data_handler: Mutex<DataHandler>,
}

impl AppState {
    pub fn find_build(&self, attack: Attack) -> ResponseBuildData {
        let mut dh = self.data_handler.lock().unwrap();
        let opt_search_res = dh.get_search_result(&attack);
        if let Some(res) = opt_search_res {
            ResponseBuildData::from_build_search_result(&res)
        } else {
            let mut build_finder = self.roxx_build_finder.lock().unwrap();
            build_finder.set_attack(attack.clone());
            let res = build_finder.find_build().clone();
            dh.add_or_update_search_result(&attack, &res);
            ResponseBuildData::from_build_search_result(&res)
        }
    }

    pub fn new(container: DataContainer) -> Self {
        let mut build_finder =  RoxxBuildFinder::new(container, Attack::default());
        build_finder.time_limit = 6 * 1_000_000_000;
        build_finder.track_data = false;
        AppState {
            roxx_build_finder: Mutex::new(build_finder),
            data_handler: Mutex::new(DataHandler::new()),
        }
    }
}