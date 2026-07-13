//! # Path: src/grid/components.rs

use bevy::prelude::*;
use std::collections::HashMap;
use crate::constants::CELL_SIZE;


#[derive(Resource, Default)]
pub struct WorldGrid (pub HashMap<IVec2, Entity>);

#[derive(Component)]
pub struct GridPos (pub IVec2);
impl GridPos {
    pub fn to_bottom_left_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.0.x as f32 * CELL_SIZE,
            y: self.0.y as f32 * CELL_SIZE,
        }
    }
    pub fn to_center_vec2(&self) -> Vec2{
        self.to_bottom_left_vec2() + Vec2 {
            x: CELL_SIZE / 2.,
            y: CELL_SIZE / 2.,
        }
    }
}

#[derive(Component)]
pub struct BackGround;
