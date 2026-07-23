use bevy::prelude::*;

use crate::consumable::common::*;

#[derive(Component)]
pub struct Channel<T>
where
    T: Consumable,
{
    pub input: Vec<Port<T>>,
    pub output: Vec<Port<T>>,
    pub gather: Vec<Port<T>>,
}

#[derive(Component)]
pub struct Port<T>
where
    T: Consumable,
{
    pub filter: Filter<T>,
    pub slot: TargetSlot,
}

#[derive(Component)]
pub enum Filter<T>
where
    T: Consumable,
{
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

#[derive(Component)]
pub enum TargetSlot {
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

#[derive(Component)]
pub struct Inventory<T>
where
    T: Consumable,
{
    pub content: Vec<MaterialSlot<T>>,
    pub size: usize,
}

#[derive(Component)]
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
    fn insert(&mut self, slot: &mut Self) -> bool {
        if let Some(val) = self.val {
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
            slot.val = None;
            slot.vol = 0;

            return true;
        }
        false
    }
}

#[derive(Component, Clone, Copy)]
pub struct SlotID(pub usize);
