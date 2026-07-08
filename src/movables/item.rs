//! # Path: src/movables/item.rs

use bevy::prelude::*;
use crate::commons::*;

#[derive(Component, Clone)]
pub struct Item {
    pub id  : Type,
    pub size: u64,
}
impl Registerable for Item {
    fn register(_app: &mut App) {
    }
}

#[derive(Clone)]
pub enum Type {
    Clay
}
impl Type {
    fn get_id_str(&self) -> &str {
        match self {
            Type::Clay => "clay"
        }
    }
    pub fn get_sprite(&self, asset_server: &Res<AssetServer>) -> Sprite {
        Sprite::from_image(asset_server.load(format!("textures/item/{}.png", self.get_id_str())))
    }
}
