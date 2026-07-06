//! # Path: src/nodes/empty.rs

use bevy::prelude::*;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Empty;
impl Registerable for Empty {
    fn register(_app: &mut App) {
        
    }
}
