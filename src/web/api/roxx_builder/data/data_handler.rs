use crate::builder::attack_mod::attack::Attack;
use crate::builder::build_mod::build_search_result::BuildSearchResult;

pub struct DataHandler {
    _db_connexion: String, // todo put real db connexion
}

impl DataHandler {
    pub fn get_search_result(&self, _attack: &Attack) -> Option<BuildSearchResult> {
        None
    }

    /// Return true if the insert worked, false otherwise
    pub fn add_or_update_search_result(&mut self, _attack: &Attack, _search_result: &BuildSearchResult) -> bool {
        false
    }

    pub fn new() -> Self {
        DataHandler {
            _db_connexion: "I'm definitely working".to_string(),
        }
    }
}