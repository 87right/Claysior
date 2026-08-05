use crate::prelude::*;

pub trait ManuMaterial: Component + Clone + Copy + PartialEq + Eq {}

pub struct MaterialSlotBuff<T>
where 
    T: ManuMaterial
{
    slot: MaterialSlot<T>,
    id: SlotID
}

#[derive(Component, Default, Clone, Copy)]
pub enum MaterialFilter<T>
where 
    T: ManuMaterial
{
    #[default]
    Any,
    Specific(T)
}
