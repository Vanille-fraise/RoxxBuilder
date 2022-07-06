use crate::builder::attack_mod::damage_element::DamageElement;

pub struct DamageLine {
    pub damage_element: DamageElement,
    pub min_value: i32,
    pub max_value: i32,
}

impl DamageLine {
    pub fn new(damage_element:DamageElement, min_value:i32, max_value: i32) -> Self {
        DamageLine {
            damage_element,
            min_value,
            max_value,
        }
    }

    pub fn new_fix(damage_element:DamageElement, value:i32) -> Self{
        DamageLine {
            damage_element,
            min_value: value,
            max_value: value
        }
    }
}