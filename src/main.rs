#![allow(dead_code)]

mod camera;
mod common;
mod grid;
mod node;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::camera::plugins::CameraPlugins)
        .add_plugins(crate::grid::plugin::GridPlugin)
        .run();
}
