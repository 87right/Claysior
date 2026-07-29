use bevy::prelude::*;

use crate::{
    consumable::common::*,
    grid::{component::*, resource::*},
};

#[derive(Component)]
pub struct Channel<T>
where
    T: Consumable,
{
    pub input: Vec<Port<T>>,
    pub output: Vec<Port<T>>,
    pub gather: Vec<Port<T>>,
}
impl<T> Channel<T>
where
    T: Consumable,
{
    pub fn insert(
        &mut self,
        to_inventory: &mut Inventory<T>,
        buff: &mut MaterialSlotBuff<T>,
        _from_entity: Entity,
        from_pos: GridPos,
        to_pos: GridPos,
        _grid: &Res<GridEntityMap>
    ) -> bool {
        let mut result = false;
        for input in self.input.iter_mut() {
            if input.grid.check(from_pos, to_pos)
            && input.insert(to_inventory, &mut buff.content) {
                result = true;
            }
        }
        result
    }
    pub fn add_port(mut self, port_type: PortType, port: Port<T>) -> Self {
        match port_type {
            PortType::Input => self.input.push(port),
            PortType::Output => self.output.push(port),
            PortType::Gather => self.gather.push(port),
        }
        self
    }
}

impl<T> Default for Channel<T>
where 
    T: Consumable,
{
    fn default() -> Self {
        Self {
            input: vec![],
            output: vec![],
            gather: vec![],
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Port<T>
where
    T: Consumable,
{
    pub filter: Filter<T>,
    pub slot: TargetSlot,
    pub grid: TargetGrid,
    pub active: bool,
}
impl<T> Port<T>
where
    T: Consumable,
{
    pub fn get_first<'a>(
        &self,
        inventory: &'a Inventory<T>,
    ) -> Option<(SlotID, &'a MaterialSlot<T>)> {
        if !self.active {return None;}
        for id in self.slot.get_slot_ids(inventory.size) {
            if let Some(slot) = inventory.get(id)
                && let Some(val) = slot.val
                && self.filter.check(val)
            {
                return Some((id, slot));
            }
        }
        None
    }
    pub fn get_buff(&self, inventory: &Inventory<T>) -> Option<MaterialSlotBuff<T>> {
        if let Some((id, slot)) = self.get_first(inventory) {
            Some(MaterialSlotBuff::<T> {
                content: *slot,
                index: id,
            })
        } else {
            None
        }
    }
    pub fn insert(&self, inventory: &mut Inventory<T>, from: &mut MaterialSlot<T>) -> bool {
        if !self.active {return false;}
        let mut inserted = false;
        for id in self.slot.get_slot_ids(inventory.size) {
            if let Some(to) = inventory.get_mut(id)
                && to.insert(from)
            {
                inserted = true;
            }
        }
        inserted
    }
    pub fn get_target_entity(&self, pos: GridPos, grid: &Res<GridEntityMap>) -> Vec<Entity> {
        self.grid.entity_vec(pos, grid)
    }
    pub fn set_filter(mut self, filter: Filter<T>) -> Self {
        self.filter = filter;
        self
    }
    pub fn set_target_slot(mut self, target_slot: TargetSlot) -> Self {
        self.slot = target_slot;
        self
    }
    pub fn set_target_grid(mut self, target_grid: TargetGrid) -> Self {
        self.grid = target_grid;
        self
    }
    pub fn activate(mut self) -> Self {
        self.active = true;
        self
    }
    pub fn deactivate(mut self) -> Self {
        self.active = false;
        self
    }
}

impl<T> Default for Port<T>
where 
    T: Consumable,
{
    fn default() -> Self {
        Self {
            filter: Filter::<T>::default(),
            slot: TargetSlot::default(),
            grid: TargetGrid::default(),
            active: true,
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub enum Filter<T>
where
    T: Consumable,
{
    #[default]
    Any,
    Specific { val: T },
    Custom(fn(val: T) -> bool),
}
impl<T: Consumable> Filter<T> {
    fn check(&self, item: T) -> bool {
        match self {
            Self::Any => true,
            Self::Specific { val } => *val == item,
            Self::Custom(f) => f(item),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub enum TargetSlot {
    #[default]
    Any,
    Specific(SlotID),
    Range { from: SlotID, to: SlotID },
    Custom(fn(id: SlotID) -> bool),
}
impl TargetSlot {
    fn get_slot_ids(&self, size: usize) -> Vec<SlotID> {
        match self {
            Self::Any => (0..size).map(SlotID).collect(),

            Self::Specific(id) => vec![*id],

            Self::Range {
                from: SlotID(from),
                to: SlotID(to),
            } => (*from..=*to).map(SlotID).collect(),

            Self::Custom(f) => (0..size).filter(|&x| f(SlotID(x))).map(SlotID).collect(),
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub enum TargetGrid {
    #[default]
    Any,
    Specific(GridPos),
}
impl TargetGrid {
    pub fn entity_vec(&self, pos: GridPos, grid: &Res<GridEntityMap>) -> Vec<Entity> {
        match self {
            Self::Any => vec![],
            Self::Specific(diff) => {
                if let Some(e) = grid.get(&(pos + *diff)) {
                    vec![e]
                } else {
                    vec![]
                }
            }
        }
    }
    pub fn check(&self, from_pos: GridPos, to_pos: GridPos) -> bool {
        match self {
            Self::Any => true,
            Self::Specific(pos) => from_pos == to_pos + *pos,
        }
    }
}

#[derive(Component, Debug)]
pub struct Inventory<T>
where
    T: Consumable,
{
    pub content: Vec<MaterialSlot<T>>,
    pub size: usize,
}
impl<T> Inventory<T>
where
    T: Consumable,
{
    pub fn get(&self, id: SlotID) -> Option<&MaterialSlot<T>> {
        self.content.get(id.0)
    }
    pub fn get_mut(&mut self, id: SlotID) -> Option<&mut MaterialSlot<T>> {
        self.content.get_mut(id.0)
    }
    pub fn insert(&mut self, id: SlotID, val: &mut MaterialSlot<T>) -> bool {
        if let Some(slot) = self.content.get_mut(id.0) {
            slot.insert(val)
        } else {
            false
        }
    }
    pub fn apply_buff(&mut self, buff: MaterialSlotBuff<T>) {
        if let Some(to) = self.content.get_mut(buff.index.0) {
            to.copy_from(buff.content);
        }
    }
}
impl<T> Inventory<T> 
where 
    T: Consumable
{
    pub fn new(size: usize) -> Self {
        Self {
            content: vec![MaterialSlot::<T>{val: None::<T>, vol: 0}; size],
            size: size,
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct MaterialSlot<T>
where
    T: Consumable,
{
    pub val: Option<T>,
    pub vol: u64,
}
impl<T> MaterialSlot<T>
where
    T: Consumable,
{
    pub fn insert(&mut self, slot: &mut Self) -> bool {
        if let Some(val) = self.val {
            if self.vol == val.get_max_size() {
                return false;
            }
            if let Some(r_val) = slot.val
                && val == r_val
            {
                if val.get_max_size() - self.vol < slot.vol {
                    slot.vol -= val.get_max_size() - self.vol;
                    self.vol = val.get_max_size();

                    return true;
                } else {
                    self.vol += slot.vol;
                    slot.val = None;
                    slot.vol = 0;

                    return true;
                }
            }
        } else {
            self.val = slot.val;
            self.vol = slot.vol;
            slot.val = None;
            slot.vol = 0;

            return true;
        }
        false
    }
    fn copy_from(&mut self, from: Self) {
        self.val = from.val;
        self.vol = from.vol;
    }
}

#[derive(Component, Clone, Copy)]
pub struct SlotID(pub usize);
