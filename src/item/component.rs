use bevy::prelude::*;

use crate::consumable::common::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Item {
    Clay,
    Brick,
}
impl Consumable for Item {
    fn get_id(&self) -> String {
        match self {
            Item::Clay => "clay",
            Item::Brick => "brick",
        }.to_string()
    }
}

#[derive(Component)]
pub struct Pickupable;

#[derive(Component)]
pub struct Age(pub u32);
