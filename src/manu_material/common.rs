use crate::prelude::*;

pub trait ManuMaterial: Component + Clone + Copy + PartialEq + Eq {
    fn get_max_size(&self) -> u64 {
        9999
    }
}

#[derive(Clone)]
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

pub enum LogisticsType {
    InputOutput,
    OpenPull
}

pub struct LogisticsOrder<T>
where 
    T: ManuMaterial
{
    pub from: GridPos,
    pub to: GridPos,
    pub slot: Option<MaterialSlotBuff<T>>,
    pub logistics_type: LogisticsType,
    pub client_id: usize,

}

impl<T> MaterialSlotBuff<T>
where 
    T: ManuMaterial
{
    pub fn new(slot: MaterialSlot<T>, id: SlotID) -> Self {
        Self {
            slot,
            id,
        }
    }
}

impl<T> LogisticsOrder<T>
where 
    T: ManuMaterial
{
    pub fn new(from: GridPos, to: GridPos, logistics_type: LogisticsType, client_id: usize) -> Self {
        Self {
            from, 
            to, 
            slot: None,
            logistics_type,
            client_id
        }
    }
    pub fn write(&mut self, buff: MaterialSlotBuff<T>) {
        self.slot = Some(buff.clone());
    }
    pub fn get_buff_mut(&mut self) -> &mut Option<MaterialSlotBuff<T>> {
        &mut self.slot
    }
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
