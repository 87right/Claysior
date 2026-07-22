use bevy::prelude::*;

#[derive(Component)]
pub struct Item {
    pub item: ItemType,
    pub size: ItemSize,
}


#[derive(Component)]
pub enum ItemType {
    Clay,
}
impl ItemType {
    fn get_max_size(&self) -> ItemSize {
        match self {
            ItemType::Clay => ItemSize(9999)
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct ItemSize(pub u32);
impl std::ops::Add for ItemSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self (
            self.0 + rhs.0
        )
    }
}

#[derive(Component)]
pub struct Pickupable;

#[derive(Component)]
pub struct Age(pub u32);

#[derive(Component)]
pub struct Inventory(pub Vec<InventorySlot>);

#[derive(Component)]
pub struct InventorySlot(pub Option<Item>);

#[derive(Component)]
pub struct InventorySlotID(pub usize);
