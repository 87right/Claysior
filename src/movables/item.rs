//! # Path: src/movables/item.rs

use bevy::prelude::*;
use crate::commons::*;

#[derive(Component, Clone, Copy)]
pub struct Item {
    pub id  : Type,
    pub size: u64,
}
impl Registerable for Item {
    fn register(_app: &mut App) {
    }
}

#[derive(Component)]
pub struct DisplayItem;

#[derive(Clone, PartialEq, Copy)]
pub enum Type {
    Clay,
    Brick,
}
impl Type {
    fn get_id_str(&self) -> &str {
        match self {
            Type::Clay => "clay",
            Type::Brick => "brick",
        }
    }
    pub fn get_sprite(&self, asset_server: &Res<AssetServer>) -> Sprite {
        Sprite::from_image(asset_server.load(format!("textures/item/{}.png", self.get_id_str())))
    }
    pub fn get_max_stack_size(&self) -> u64 {
        match self {
            Type::Clay => 4,
            Type::Brick => 4,
        }
    }
}
