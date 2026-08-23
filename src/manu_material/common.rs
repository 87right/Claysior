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
    pub slot: MaterialSlot<T>,
    pub id: SlotID
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

#[derive(PartialEq, Eq)]
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
    pub record: Option<MaterialSlotBuff<T>>,
    pub cd_ticks: u64
}

impl<T> LogisticsOrder<T>
where 
    T: ManuMaterial
{
    pub fn is_done(&self) -> bool {
        if let Some(slot) = &self.slot
        && let Some(reco) = &self.record {
            slot.slot.get_raw().0 != reco.slot.get_raw().0 ||
            slot.slot.get_raw().1 != reco.slot.get_raw().1
        } else {
            self.slot.is_some() && self.record.is_none()
        }
    }
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
            client_id,
            record: None,
            cd_ticks: 1
        }
    }
    pub fn write(&mut self, buff: Option<MaterialSlotBuff<T>>, cd_ticks: u64) {
        self.slot = buff.clone();
        self.record = buff;
        self.cd_ticks = cd_ticks;
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
impl DisplayableManuMaterial for Item {
    fn insert_texture<'a>(&self, mut commands: EntityCommands<'a>) -> EntityCommands<'a> {
        commands.insert(
            match self {
                Self::Clay => texture_material_buff::FromImage("textures/item/clay.png")
            }
        );
        commands
    }
}

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
