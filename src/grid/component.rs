use crate::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos{
    pub x: u64,
    pub y: u64,
}
impl std::ops::Add for GridPos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl GridPos {
    pub fn new(x: u64, y: u64) -> Self {
        Self {x, y}
    }
    pub fn into_world_pos(&self) -> Vec2 {
        Vec2 { 
            x: self.x as f32 * GameSetting::CELL_SIZE, 
            y: self.y as f32 * GameSetting::CELL_SIZE,
        }
    }
    pub fn into_transform(&self, z: f32) -> Transform {
        let vec2 = self.into_world_pos();
        Transform::from_xyz(vec2.x, vec2.y, z)
    }
}
