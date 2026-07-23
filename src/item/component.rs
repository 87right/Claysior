use bevy::prelude::*;

use crate::consumable::common::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Clay,
}
impl Consumable for Item {}

#[derive(Component)]
pub struct Pickupable;

#[derive(Component)]
pub struct Age(pub u32);
