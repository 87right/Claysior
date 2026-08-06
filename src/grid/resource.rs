use std::collections::HashMap;

use crate::prelude::*;

#[derive(Resource, Default)]
pub struct GridEntityMap(HashMap<GridPos, Entity>);
impl GridEntityMap {
    pub fn insert(&mut self, key: GridPos, value: Entity) -> Option<Entity> {
        self.0.insert(key, value)
    }
    pub fn get(&self, key: GridPos) -> Option<Entity> {
        self.0.get(&key).and_then(|x| Some(*x))
    }
}

#[derive(Resource)]
pub struct WorldGeneratingSetting {
    pub width: u64,
    pub height: u64,
    pub background: &'static str,
}
impl Default for WorldGeneratingSetting {
    fn default() -> Self {
        Self {
            width: 16,
            height: 16,
            background: "basic_tile"
        }
    }
}
