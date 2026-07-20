use bevy::prelude::*;
use crate::common::constant::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
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
}
