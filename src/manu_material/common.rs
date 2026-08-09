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

pub enum PortType {
    Input ,
    Output,
    Open  ,
    Pull  ,
}

pub struct LogisticsOrder<T>
where 
    T: ManuMaterial
{
    pub from: GridPos,
    pub to: GridPos,
    pub slot: MaterialSlotBuff<T>,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Clay,
}
impl ManuMaterial for Item {}
