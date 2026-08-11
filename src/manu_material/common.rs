use crate::prelude::*;

pub trait ManuMaterial: Component + Clone + Copy + PartialEq + Eq {
    fn get_max_size(&self) -> u64 {
        9999
    }
}

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

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Clay,
}
impl ManuMaterial for Item {}

impl<T> MaterialFilter<T>
where 
    T: ManuMaterial
{
    pub fn check(&self, value: T) -> bool {
        match self {
            Self::Any => true,
            Self::Specific(filter) => *filter == value,
        }
    }
}
