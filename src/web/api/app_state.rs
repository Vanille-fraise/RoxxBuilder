use crate::builder::algorithm_mod::roxx_build_finder::RoxxBuildFinder;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::data_mod::data_container::DataContainer;
use crate::web::api::roxx_builder::models::response_build_data::ResponseBuildData;

pub struct AppState<'a> {
    roxx_build_finder: RoxxBuildFinder<'a>,
    db_connexion: String, //todo update for real db connexion
}

impl<'a> AppState<'a> {
    pub fn find_build(&self, attack: Attack) -> ResponseBuildData {
        // todo check into DB and return the found value
        // if not present compute the value
        ResponseBuildData::from_build_search_result(&self.roxx_build_finder.find_build())
    }

    pub fn new() -> Self {
        // todo put the right data
        AppState {
            roxx_build_finder: RoxxBuildFinder::new(DataContainer::new(), &Attack::default()),
            db_connexion: "Yeah i'm working, sure".to_string(),
        }
    }
}