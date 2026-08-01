use bevy::prelude::*;

use crate::consumable::component::SlotID;

#[derive(Resource, Default)]
pub struct RegisteredSlot(pub Vec<(Entity, SlotID)>);
impl RegisteredSlot {
    pub fn get(&self) -> &Vec<(Entity, SlotID)> {
        &self.0
    }

    pub fn push(&mut self, entity: Entity, slot: SlotID) {
        self.0.push((entity, slot));
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.0.len() {return;}
        self.0[index] = self.0[self.0.len() - 1];
        self.0.pop();
    }
}
