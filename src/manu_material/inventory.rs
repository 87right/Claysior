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
    Any
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct SlotID(usize);

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
}

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
