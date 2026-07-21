use std::string::String;
use bevy::prelude::*;
use crate::common::constant::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPos(pub IVec2);
impl std::ops::Add for GridPos{
    type Output = GridPos;
    fn add(self, rhs: GridPos) -> GridPos {
        GridPos(self.0 + rhs.0)
    }
}
impl GridPos{
    pub fn x(&self) -> i32 {
        self.0.x
    }
    pub fn y(&self) -> i32 {
        self.0.y
    }
    pub fn to_world_pos(&self) -> Vec2 {
        Vec2 { 
            x: self.x() as f32 * CELL_SIZE, 
            y: self.y() as f32 * CELL_SIZE, 
        }
    }
    pub fn from_world_pos(mut world_pos: Vec2) -> Self {
        world_pos += vec2(1., 1.) * CELL_SIZE / 2.;
        Self(
            IVec2{
                x: (world_pos.x / CELL_SIZE).floor() as i32,
                y: (world_pos.y / CELL_SIZE).floor() as i32,
            }
        )
    }
}

#[derive(Component)]
pub struct PlaceBuff(pub String);
impl PlaceBuff{
    pub fn from_str(id: &str) -> Self {
        Self(id.to_string())
    }
}
