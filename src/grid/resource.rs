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

#[derive(Resource, Default)]
pub struct SyncMouseButtonInput{
    p: u8,
    r: u8,
    jp: u8,
    jr: u8,
}
impl SyncMouseButtonInput {
    const LEFT   : u8 = 1 << 0;
    const RIGHT  : u8 = 1 << 1;
    const MIDDLE : u8 = 1 << 2;
    const BACK   : u8 = 1 << 3;
    const FORWARD: u8 = 1 << 4;
    fn dispatch(button: MouseButton) -> u8 {
        match button {
            MouseButton::Left    => Self::LEFT,
            MouseButton::Right   => Self::RIGHT,
            MouseButton::Middle  => Self::MIDDLE,
            MouseButton::Back    => Self::BACK,
            MouseButton::Forward => Self::FORWARD,
            _ => 0,
        }
    }
    fn read(mut f: impl FnMut(MouseButton) -> bool) -> u8 {
        let val =
          if f(MouseButton::Left   ) {Self::LEFT   } else {0}
        | if f(MouseButton::Right  ) {Self::RIGHT  } else {0}
        | if f(MouseButton::Middle ) {Self::MIDDLE } else {0}
        | if f(MouseButton::Back   ) {Self::BACK   } else {0}
        | if f(MouseButton::Forward) {Self::FORWARD} else {0};
        val
    }
    pub fn write(&mut self, button: &ButtonInput<MouseButton>) {
        self.p  |= Self::read(|b| button.pressed(b));
        self.r  |= Self::read(|b| !button.pressed(b));
        self.jp |= Self::read(|b| button.just_pressed(b));
        self.jr |= Self::read(|b| button.just_released(b));
    }
    pub fn clear(&mut self) {
        self.p = 0; self.r = 0; self.jp = 0; self.jr = 0;
    }
    pub fn pressed(&self, button: MouseButton) -> bool {
        self.p & Self::dispatch(button) > 0
    }
    pub fn released(&self, button: MouseButton) -> bool {
        self.r & Self::dispatch(button) > 0
    }
    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.jp & Self::dispatch(button) > 0
    }
    pub fn just_released(&self, button: MouseButton) -> bool {
        self.jr & Self::dispatch(button) > 0
    }
}
