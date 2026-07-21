use std::collections::HashMap;
use bevy::prelude::*;
use crate::{
    common::constant::{MAP_HEIGHT, MAP_WIDTH}, grid::component::*
};


#[derive(Resource, Default)]
pub struct GridEntityMap(pub HashMap<IVec2, Entity>);
impl GridEntityMap{
    pub fn get(&self, grid_pos: &GridPos) -> Option<Entity> {
        self.0.get(&grid_pos.0).and_then(|e| Some(*e))
    }
    pub fn insert(&mut self, grid_pos: &GridPos, entity: Entity) -> Option<Entity> {
        self.0.insert(grid_pos.0.clone(), entity)
    }
}

#[derive(Resource)]
pub struct GridGenSetting{
    pub height: u32,
    pub width: u32,
    pub background: String,
}
impl Default for GridGenSetting{
    fn default() -> Self {
        Self{
            height: MAP_HEIGHT,
            width: MAP_WIDTH,
            background: "basic_tile.png".to_string(),
        }
    }
}

#[derive(Resource, Default)]
pub struct SpawnTable(pub HashMap<String, fn(&mut Commands, Entity)>);
impl SpawnTable {
    pub fn insert(&mut self, key: String, val: fn(&mut Commands, Entity)) -> Option<fn(&mut Commands, Entity)>{
        self.0.insert(key, val)
    }
    pub fn get(&self, key: &String) -> Option<&fn(&mut Commands, Entity)> {
        self.0.get(key)
    }
}

#[derive(Resource, Default)]
pub struct Background(pub Option<Entity>);
impl Background {
    pub fn get(&self) -> Option<Entity> {
        self.0
    }
    pub fn set(&mut self, entity: Entity) -> Option<Entity> {
        let val = self.0;
        self.0 = Some(entity);
        val
    }
}
