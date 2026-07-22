#![allow(dead_code)]

mod camera;
mod common;
mod grid;
mod node;
mod item;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::camera::plugins::CameraPlugins)
        .add_plugins(crate::grid::plugin::GridPlugin)
        .add_plugins(crate::node::plugin::NodePlugin)
        .add_plugins(crate::item::plugin::ItemPlugin)
        .run();
}
