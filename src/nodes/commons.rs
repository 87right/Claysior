//! # Path: src/nodes/commons.rs

use bevy::prelude::*;
use crate::{grid::messages::Placed, movables::item::Item};

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
    pub fn insert(&mut self, item: Item) -> bool {
        for slot in &mut self.0 {
            if slot.insert(item) {
                return true;
            }
        }
        false
    }
    pub fn take_1(&mut self, slot_id: InventorySlotID) -> bool {
        if let Some(slot) = self.0.get_mut(slot_id.0) {
            slot.take_1()
        } else {
            false
        }
    }
    pub fn can_insert(&self, input: InputPort, item: Item) -> bool {
        for slot_id in input.port.get_target_slot_id() {
            if let Some(slot) = self.0.get(slot_id.0) {
                if slot.can_insert(item) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone)]
pub struct InventorySlot (pub Option<Item>);
impl InventorySlot {
    pub fn insert(&mut self, item: Item) -> bool {
        if let Some(content) = &mut self.0 {
            if content.id == item.id {
                content.size += item.size;
                true
            } else {
                false
            }
        } else {
            self.0 = Some(item);
            true
        }
    }
    pub fn take_1(&mut self) -> bool {
        let mut is_empty = false;
        let mut result = false;
        if let Some(item) = &mut self.0 {
            item.size -= 1;
            is_empty = item.size == 0;
            result = true;
        }
        if is_empty {
            self.0 = None;
        }
        result
    }
    pub fn can_insert(&self, item: Item) -> bool {
        if let Some(self_item) = self.0 {
            item.id == self_item.id
        } else {
            true
        }
    }
}

#[derive(Copy, Clone)]
pub struct InventorySlotID (pub usize);
pub struct InventorySize (pub usize);

#[derive(Message)]
pub struct ItemSendReq {
    pub from  : Entity,
    pub to    : Entity,
    pub index : InventorySlotID,
    pub e_item: Option<Entity>
}

pub trait Spawnable {
    fn get_bundle() -> impl Bundle;
}

/*
    TODO: From から適切な要素を削除。
    たとえば
    (
        MachineComponent,
        Inventory
    )
    があった場合、これでは Inventory が残ってしまう。

    Input/Output の挙動を統一した場合に好ましくない挙動をする可能性あり。
*/
pub fn replace<From: Bundle, To: Spawnable>(
    command: &mut Commands,
    writer: &mut MessageWriter<Placed>,
    entity: Entity,
) {
    command.entity(entity).remove::<From>().insert(To::get_bundle());
    writer.write(Placed(entity));
}

#[derive(Component)]
pub struct Shake {
    pub base_x: f32,
    pub scale: f32,
    pub pace: f32,
    pub timer: Timer
}

#[derive(Component, Clone, Copy)]
pub struct InputPort{
    pub port: Port,
    pub is_active: bool,
    pub recieved: bool,
    pub display_item: Option<Entity>,
}
impl InputPort {
    pub fn new(port: Port) -> Self {
        Self {
            port: port,
            is_active: true,
            recieved: false,
            display_item: None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Port{
    Single(InventorySlotID),
    Range(InventorySlotID, InventorySlotID)
}
impl Port {
    fn get_target_slot_id(&self) -> Vec<InventorySlotID>{
        match *self {
            Port::Single (id) => vec![id],    
            Port::Range (begin, end) => ((begin.0)..=(end.0)).map(InventorySlotID).collect()
        }
    }
}