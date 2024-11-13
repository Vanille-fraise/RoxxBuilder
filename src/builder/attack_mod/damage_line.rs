use std::cmp::max;

use crate::builder::attack_mod::damage_element::DamageElement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DamageLine {
    #[serde(rename = "effectElement")]
    pub damage_element: DamageElement,

    // Idk what to do with it yet, but it might be usefull later for heal, do pou, best or worst element
    pub effect_id: i64,
    #[serde(rename = "diceNum")]
    pub min_value: i64,
    #[serde(rename = "diceSide")]
    pub max_value: i64,
    pub for_client_only: bool,
}

impl DamageLine {
    pub fn new(damage_element: DamageElement, min_value: i64, max_value: i64) -> Self {
        DamageLine {
            damage_element,
            min_value,
            max_value,
            for_client_only: false,
            effect_id: -1,
        }
    }

    pub fn new_fix(damage_element: DamageElement, value: i64) -> Self {
        DamageLine {
            damage_element,
            min_value: value,
            max_value: value,
            for_client_only: false,
            effect_id: -1,
        }
    }

    pub fn has_no_damage(&self) -> bool {
        return self.min_value == 0 && self.max_value == 0 || self.damage_element == DamageElement::None || self.for_client_only;
    }

    pub fn fix_values(&mut self) {
        self.min_value = max(1, self.min_value);
        self.max_value = max(1, self.max_value);

        if self.min_value > self.max_value {
            let temp = self.min_value;
            self.min_value = self.max_value;
            self.max_value = temp;
        }
    }
}
