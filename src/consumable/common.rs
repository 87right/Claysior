use bevy::prelude::*;

use crate::consumable::component::*;

pub trait Consumable: Component + Clone + Copy + PartialEq + Eq {
    fn get_max_size(&self) -> u64 {
        u64::MAX
    }
    fn get_id(&self) -> String;
}

#[derive(Clone)]
pub struct MaterialSlotBuff<T>
where
    T: Consumable,
{
    pub content: MaterialSlot<T>,
    pub index: SlotID,
}

pub enum PortType {
    Input,
    Output,
    Open,
    Pull,
    Gather,
}
