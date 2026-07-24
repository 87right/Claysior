use bevy::prelude::*;

use crate::consumable::component::*;

pub trait Consumable: Component + Clone + Copy + PartialEq + Eq {
    fn get_max_size(&self) -> u64 {
        u64::MAX
    }
}

pub struct MaterialSlotBuff<T>
where 
    T: Consumable
{
    pub content: MaterialSlot<T>,
    pub index: SlotID,
}
