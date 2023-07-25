use crate::builder::attack_mod::damage_element::DamageElement;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DamageLine {
    pub damage_element: DamageElement,
    pub min_value: i64,
    pub max_value: i64,
}

impl DamageLine {
    pub fn new(damage_element:DamageElement, min_value:i64, max_value: i64) -> Self {
        DamageLine {
            damage_element,
            min_value,
            max_value,
        }
    }

    pub fn new_fix(damage_element:DamageElement, value:i64) -> Self{
        DamageLine {
            damage_element,
            min_value: value,
            max_value: value
        }
    }
}