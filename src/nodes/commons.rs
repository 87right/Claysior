//! # Path: src/nodes/commons.rs

use bevy::prelude::*;
use crate::movables::item::Item;

#[derive(Component)]
pub struct Inventory (pub Vec<InventorySlot>);
impl Inventory {
    pub fn create_empty(size: InventorySize) -> Self{
        Self (vec![InventorySlot(None); size.0])
    }
    pub fn check_item(&self, slot_id: InventorySlotID) -> Option<&Item> {
        self.0.get(slot_id.0)?.0.as_ref()
    }
    pub fn take_item(&mut self, slot_id: InventorySlotID) -> Option<Item> {
        self.0.get_mut(slot_id.0)?.0.take()
    }
    pub fn write_item(&mut self, slot_id: InventorySlotID, item: InventorySlot) {
        if let Some(slot) = self.0.get_mut(slot_id.0) {
            *slot = item;
        }
    }
}

#[derive(Clone)]
pub struct InventorySlot (pub Option<Item>);

pub struct InventorySlotID (pub usize);
pub struct InventorySize (pub usize);

#[derive(Message)]
pub struct ItenSendReq {
    pub from : Entity,
    pub index: InventorySlotID,
}
