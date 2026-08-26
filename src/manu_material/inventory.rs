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
    cd_ticks: u64,
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
    Baked(Vec<SlotID>)
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
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
            cd_ticks: 0,
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
    pub fn get(&self, id: SlotID) -> Option<&MaterialSlot<T>> {
        self.content.get(id.0)
    }
    pub fn get_mut(&mut self, id: SlotID) -> Option<&mut MaterialSlot<T>> {
        self.content.get_mut(id.0)
    }
    pub fn apply_buff(&mut self, buff: &MaterialSlotBuff::<T>) {
        let id = &buff.id;
        let slot = &buff.slot;

        if let Some(inv_slot) = self.get_mut(*id) {
            *inv_slot = slot.clone();
        }
    }
    pub fn test_init(mut self, ind: usize, val: Option<T>, vol: u64) -> Self {
        self.content.get_mut(ind).and_then(|x| {
            x.value = val; x.volume = vol;
            None::<T>
        });
        self
    }
    pub fn test_constructor(size: usize, f: fn(Inventory::<T>) -> Self) -> Self {
        f(Self::new(size))
    }
}

impl InventorySlice {
    pub fn get_slot_id<'a, T>(&'a mut self, inventory: &Inventory<T>) -> &'a Vec<SlotID> 
    where 
        T: ManuMaterial
    {
        if let Self::Baked(v) = self {
            return v;
        }

        *self = Self::Baked(
            match self {
                Self::Any => (0..inventory.size).map(SlotID).collect(),
                Self::Continuity { begin, end } => (begin.0..end.0).filter(|x| *x < inventory.size).map(SlotID).collect(),
                _ => (0..0).map(SlotID).collect(),
            }
        );

        if let Self::Baked(v) = self {
            return v;
        } else {
            panic!("到達不可能です");
        }
    } 
    pub fn insert<T>(&mut self, inventory: &mut Inventory<T>, from_slot: &mut MaterialSlot<T>, cd_ticks: u64) -> bool
    where
        T: ManuMaterial
    {
        let mut result = false;
        for id in self.get_slot_id(inventory).iter() {
            if let Some(to_slot) = inventory.get_mut(*id) {
                result |= to_slot.insert(from_slot, cd_ticks);
            }
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
    pub fn get(&self) -> Option<(Option<T>, u64, &SlotSetting<T>)> {
        if self.cd_ticks > 0 {
            return None;
        }
        Some(self.get_raw())
    }
    pub fn get_raw(&self) -> (Option<T>, u64, &SlotSetting<T>) {
        (self.value, self.volume, &self.setting)
    }
    pub fn insert(&mut self, slot: &mut MaterialSlot<T>, cd_ticks: u64) -> bool {
        if self.cd_ticks > 0 {
            return false;
        }

        let record = slot.clone();
        
        if self.value.is_none() || self.value == slot.value {
            self.volume += slot.volume;
            self.value = slot.value;
            slot.set_content(self.clamp());
        }

        if !slot.equal(&record) {
            self.cd_ticks = cd_ticks;
            true
        } else {
            false
        }
    }
    fn clamp(&mut self) -> (Option<T>, u64) {
        if let Some(value) = self.value {
            if !self.setting.filter.check(value) {
                let result = (self.value, self.volume);
                self.value = None;
                self.volume = 0;
                return result;
            }

            let max_size = self.setting.max_volume.min(value.get_max_size());
            if self.volume > max_size {
                let over = self.volume - max_size;
                self.volume = max_size;
                (Some(value), over)
            } else {
                println!("clamp 関数 よし！ 最大: {max_size}, 現在: {}, slot: {}, item: {}", self.volume, self.setting.max_volume, value.get_max_size());
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
    pub fn update(&mut self) {
        if self.cd_ticks > 0 {
            self.cd_ticks -= 1;
        }
    }
    pub fn setting<'a>(&'a mut self) -> &'a mut SlotSetting<T> {
        &mut self.setting
    }
}

impl<T> SlotSetting<T>
where 
    T: ManuMaterial
{
    pub fn set_max_volume(&mut self, value: u64) {
        self.max_volume = value;
    }
}
