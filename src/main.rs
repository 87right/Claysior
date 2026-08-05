#![allow(dead_code)]

mod camera;
mod common;
mod manu_material;
mod grid;
mod gui;
mod input;
mod node;
mod recipe;
mod prelude;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugin::InputPlugin)
        .add_plugins(plugin::CameraPlugin)
        .add_plugins(plugin::GUIPlugin)
        .run();
}