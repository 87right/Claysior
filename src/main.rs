#![allow(dead_code)]

mod camera;
mod common;
mod consumable;
mod grid;
mod item;
mod node;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::camera::plugins::CameraPlugins)
        .add_plugins(crate::grid::plugin::GridPlugin)
        .add_plugins(crate::node::plugin::NodePlugin)
        .add_plugins(crate::item::plugin::ItemPlugin)
        .add_plugins(crate::consumable::plugin::ConsumablePlugin)
        .run();
}
