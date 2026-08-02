use bevy::prelude::*;

use crate::{
    consumable::{common::*, resource::RegisteredSlot}, grid::{component::*, resource::*},
};

#[derive(Component)]
pub struct Channel<T>
where
    T: Consumable,
{
    pub input: Vec<Port<T>>,
    pub output: Vec<Port<T>>,
    pub open: Vec<Port<T>>,
    pub pull: Vec<Port<T>>,
    pub gather: Vec<Port<T>>,
    pub time_cost: u64,
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
        to_entity: Entity,
        to_pos: GridPos,
        _grid: &Res<GridEntityMap>,
        registered_slot: &mut RegisteredSlot,
        time_cost: u64,
    ) -> bool {
        let mut result = false;
        for input in self.input.iter_mut() {
            if input.grid.check(from_pos, to_pos)
            && input.insert(to_inventory, &mut buff.content, time_cost, registered_slot, to_entity) {
                result = true;
            }
        }
        result
    }
    pub fn add_port(mut self, port_type: PortType, port: Port<T>) -> Self {
        match port_type {
            PortType::Input => self.input.push(port),
            PortType::Output => self.output.push(port),
            PortType::Open => self.open.push(port),
            PortType::Pull => self.pull.push(port),
            PortType::Gather => self.gather.push(port),
        }
        self
    }
    pub fn inserted(&mut self, port_type: PortType, index: usize) {
        self.get_port(port_type, index).and_then(|port| {
            port.mode.reset();
            None::<&mut Port<T>>
        });
    }
    pub fn get_port(&mut self, port_type: PortType, index: usize) -> Option<&mut Port<T>> {
        match port_type {
            PortType::Input => &mut self.input,
            PortType::Output => &mut self.output,
            PortType::Open => &mut self.open,
            PortType::Pull => &mut self.pull,
            PortType::Gather => &mut self.gather,
        }.get_mut(index)
    }
    pub fn configure_time_cost(mut self, value: u64) -> Self {
        self.time_cost = value;
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
            open: vec![],
            pull: vec![],
            gather: vec![],
            time_cost: 0,
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
    pub mode: PortMode,
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
                && let Some(val) = slot.get_val()
                && self.filter.check(val)
            {
                return Some((id, slot));
            }
        }
        None
    }
    pub fn get_buff(&self, inventory: &Inventory<T>) -> Option<MaterialSlotBuff<T>> {
        if self.mode.is_valid()
        && let Some((id, slot)) = self.get_first(inventory) {
            Some(MaterialSlotBuff::<T> {
                content: *slot,
                index: id,
            })
        } else {
            None
        }
    }
    pub fn insert(&mut self, inventory: &mut Inventory<T>, from: &mut MaterialSlot<T>, time_cost: u64, registered_slot: &mut RegisteredSlot, e: Entity) -> bool {
        if !self.active || !self.mode.is_valid() {return false;}
        let mut inserted = false;
        for id in self.slot.get_slot_ids(inventory.size) {
            if let Some(to) = inventory.get_mut(id)
                && to.insert(from, time_cost)
            {
                inserted = true;
                self.mode.reset();
                registered_slot.push(e, id);
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
    pub fn set_mode(mut self, mode: PortMode) -> Self {
        self.mode = mode;
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
            mode: PortMode::default(),
            active: true,
        }
    }
}

#[derive(Component, Clone, Copy, Default)]
pub enum PortMode {
    #[default]
    Always,
    WithCD(u32, u32),
}
impl PortMode {
    pub fn with_cool_down(ticks: u32) -> Self {
        Self::WithCD(ticks, 0)
    }
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Always => true,
            Self::WithCD(val, pro) => val <= pro,
        }
    }
    pub fn update(&mut self) {
        match self {
            Self::WithCD(val, pro) => {
                if pro < val {
                    *pro += 1;
                }
            },
            _ => {}
        }
    }
    fn reset(&mut self) {
        match self {
            Self::WithCD(_, pro) => {
                *pro = 0;
            },
            _ => {}
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
    pub fn configure_slot(mut self, id: SlotID, f: fn(slot: &mut MaterialSlot<T>)) -> Self {
        self.get_mut(id).and_then(|slot| {
            f(slot);
            None::<&mut MaterialSlot<T>>
        });
        self
    }
    pub fn configure_all_slots(mut self, f: fn(slot: &mut MaterialSlot<T>)) -> Self {
        for slot in &mut self.content {
            f(slot);
        };
        self
    }
    pub fn insert(&mut self, id: SlotID, val: &mut MaterialSlot<T>, time_cost: u64) -> bool {
        if let Some(slot) = self.content.get_mut(id.0) {
            slot.insert(val, time_cost)
        } else {
            false
        }
    }
    pub fn apply_buff(&mut self, buff: MaterialSlotBuff<T>) {
        if let Some(to) = self.content.get_mut(buff.index.0) {
            to.copy_from(buff.content);
        }
    }
    pub fn new(size: usize) -> Self {
        Self {
            content: vec![MaterialSlot::<T>::new(); size],
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
    pub max: u64,
    pub reserved: u64,
}
impl<T> MaterialSlot<T>
where
    T: Consumable,
{
    pub fn new() -> Self {
        Self {
            val: None,
            vol: 0,
            max: u64::MAX,
            reserved: 0,
        }
    }
    pub fn get_val(&self) -> Option<T> {
        if self.reserved == 0 {
            self.val
        } else {
            None::<T>
        }
    }

    pub fn update_and_is_valid(&mut self) -> bool {
        if self.reserved == 0 {
            true
        } else {
            self.reserved -= 1;
            self.reserved == 0
        }
    }

    pub fn configure_value(mut self, value: Option<T>) -> Self{
        self.val = value;
        self
    }
    pub fn configure_volume(mut self, volume: u64) -> Self{
        self.vol = volume;
        self
    }
    pub fn configure_max_volume(mut self, volume: u64) -> Self{
        self.max = volume;
        self
    }
    pub fn set_value(&mut self, value: Option<T>) {
        self.val = value;
    }
    pub fn set_volume(&mut self, volume: u64) {
        self.vol = volume;
    }
    pub fn set_max_volume(&mut self, volume: u64) {
        self.max = volume;
    }

    pub fn insert(&mut self, slot: &mut Self, time_cost: u64) -> bool {
        if self.reserved != 0 {
            return false;
        }
        if let Some(val) = self.val {
            if self.vol == val.get_max_size() {
                return false;
            }
            if self.vol < self.max
            && self.vol < val.get_max_size()
            && let Some(r_val) = slot.val
            && val == r_val
            {
                let item_cap = val.get_max_size() < slot.vol + self.vol;
                let slot_cap = self.max < slot.vol + self.vol;
                if item_cap || slot_cap {
                    let mut take_item_size = slot.vol;
                    if self.vol + take_item_size > val.get_max_size() {
                        take_item_size = val.get_max_size() - self.vol;
                    }
                    if self.vol + take_item_size > self.max {
                        take_item_size = self.max - self.vol;
                    }
                    slot.vol -= take_item_size;
                    self.vol += take_item_size;

                    if slot.vol == 0 {
                        slot.val = None::<T>;
                    }

                    self.reserved = time_cost;

                    return true;
                } else {
                    self.vol += slot.vol;
                    slot.val = None;
                    slot.vol = 0;

                    self.reserved = time_cost;

                    return true;
                }
            }
        } else {
            if slot.vol > self.max {
                self.val = slot.val;
                self.vol = self.max;
                slot.vol -= self.max;
            } else if let Some(item) = slot.val
            && slot.vol > item.get_max_size() {
                self.val = slot.val;
                self.vol = item.get_max_size();
                slot.vol -= item.get_max_size();
            } else {
                self.val = slot.val;
                self.vol = slot.vol;
                slot.val = None;
                slot.vol = 0;
            }

            self.reserved = time_cost;

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
