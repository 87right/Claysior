use bevy::prelude::*;

pub trait Consumable: Component + Clone + Copy + PartialEq + Eq {
    fn get_max_size(&self) -> u64 {
        u64::MAX
    }
}
