#![allow(dead_code)]

mod camera;
mod common;
mod consumable;
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
        .run();
}