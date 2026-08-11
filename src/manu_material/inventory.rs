use std::slice::{Iter, IterMut};

use crate::prelude::*;

#[derive(Component)]
pub struct Inventory<T>
where 
    T: ManuMaterial
{
    content: Vec<MaterialSlot<T>>,
    size: usize,
}

#[derive(Component, Clone)]
pub struct MaterialSlot<T>
where 
    T: ManuMaterial
{
    value: Option<T>,
    volume: u64,
    setting: SlotSetting<T>,
}

#[derive(Component, Clone)]
pub struct SlotSetting<T>
where 
    T: ManuMaterial
{
    max_volume: u64,
    filter: MaterialFilter<T>,
}

#[derive(Component, Default, Clone)]
pub enum InventorySlice {
    #[default]
    Any,
    Continuity {
        begin: SlotID,
        end: SlotID,
    },
}

pub enum InventoryIterator<'a, T> 
where
    T: ManuMaterial
{
    Raw(core::slice::Iter<'a, MaterialSlot<T>>),
    Continuity(std::iter::Take<std::iter::Skip<std::slice::Iter<'a, MaterialSlot<T>>>>),
}

pub enum InventoryIteratorMut<'a, T> 
where
    T: ManuMaterial
{
    Raw(core::slice::IterMut<'a, MaterialSlot<T>>),
    Continuity(std::iter::Take<std::iter::Skip<std::slice::IterMut<'a, MaterialSlot<T>>>>),
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct SlotID(pub usize);

impl<T> Default for MaterialSlot<T>
where 
    T: ManuMaterial
{
    fn default() -> Self {
        Self {
            value: None::<T>,
            volume: 0,
            setting: SlotSetting::<T>::default(),
        }
    }
}

impl<T> Default for SlotSetting<T>
where 
    T: ManuMaterial
{
    fn default() -> Self {
        Self {
            max_volume: u64::MAX,
            filter: MaterialFilter::<T>::default(),
        }
    }
}

impl<T> Inventory<T>
where 
    T: ManuMaterial
{
    pub fn new(size: usize) -> Self {
        Self {
            content: vec![MaterialSlot::<T>::default(); size],
            size: size,
        }
    }
    pub fn iter(& self) -> Iter<'_, MaterialSlot<T>> {
        self.content.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, MaterialSlot<T>> {
        self.content.iter_mut()
    }
    pub fn get(&self, id: SlotID) -> Option<MaterialSlot<T>> {
        self.content.get(id.0).and_then(|x| Some(x.clone()))
    }
}

impl<'a, T> Iterator for InventoryIterator<'a, T> 
where 
    T: ManuMaterial
{
    type Item = &'a MaterialSlot<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Raw(iter) => iter.next(),
            Self::Continuity(iter) => iter.next(),
        }
    }
}

impl<'a, T> Iterator for InventoryIteratorMut<'a, T> 
where 
    T: ManuMaterial
{
    type Item = &'a mut MaterialSlot<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Raw(iter) => iter.next(),
            Self::Continuity(iter) => iter.next(),
        }
    }
}

impl InventorySlice {
    fn get_slice<'a, T>(&self, inventory: &'a Inventory<T>) -> InventoryIterator<'a, T>
    where 
        T: ManuMaterial
    {
        match self {
            Self::Any => InventoryIterator::<'a, T>::Raw(inventory.iter()),
            Self::Continuity { begin, end } => {
                let SlotID(begin) = *begin;
                let SlotID(end) = *end;
                InventoryIterator::<'a, T>::Continuity(inventory.iter().skip(begin).take(end - begin + 1))
            }
        }
    }
    fn get_slice_mut<'a, T>(&self, inventory: &'a mut Inventory<T>) -> InventoryIteratorMut<'a, T>
    where 
        T: ManuMaterial
    {
        match self {
            Self::Any => InventoryIteratorMut::<'a, T>::Raw(inventory.iter_mut()),
            Self::Continuity { begin, end } => {
                let SlotID(begin) = *begin;
                let SlotID(end) = *end;
                InventoryIteratorMut::<'a, T>::Continuity(inventory.iter_mut().skip(begin).take(end - begin + 1))
            }
        }
    }
    pub fn insert<T>(&self, inventory: &mut Inventory<T>, from_slot: &mut MaterialSlot<T>) -> bool
    where
        T: ManuMaterial
    {
        let mut result = false;
        for to_slot in self.get_slice_mut(inventory) {
            result |= to_slot.insert(from_slot);
        }
        result
    }
}

impl<T> MaterialSlot<T>
where 
    T: ManuMaterial
{
    pub fn set(&mut self, value: Option<T>, volume: u64) {
        self.value = value;
        self.volume = volume;
        self.clamp();
    }
    pub fn get(&self) -> (Option<T>, u64, &SlotSetting<T>) {
        (self.value, self.volume, &self.setting)
    }
    pub fn insert(&mut self, slot: &mut MaterialSlot<T>) -> bool {
        let record = slot.clone();
        
        if self.value.is_none() || self.value == slot.value {
            self.volume += slot.volume;
            self.value = slot.value;
            slot.set_content(self.clamp());
        }

        !slot.equal(&record)
    }
    fn clamp(&mut self) -> (Option<T>, u64) {
        if let Some(value) = self.value {
            if !self.setting.filter.check(value) {
                let result = (self.value, self.volume);
                self.value = None;
                self.volume = 0;
                return result;
            }

            let max_size = if self.setting.max_volume < value.get_max_size() {
                self.setting.max_volume
            } else {
                value.get_max_size()
            };
            if self.volume > max_size {
                let over = self.volume - max_size;
                self.volume = max_size;
                (Some(value), over)
            } else {
                (None, 0)
            }
        } else {
            self.volume = 0;
            (None, 0)
        }
    }
    fn set_content(&mut self, value: (Option<T>, u64)) {
        self.value = value.0;
        self.volume = value.1;
    }
    fn equal(&self, rhs: &Self) -> bool {
        self.value == rhs.value && self.volume == rhs.volume
    }
}

