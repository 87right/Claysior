//! # Path: src/main.rs

#![allow(dead_code)]

mod constants;
mod camera;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::camera::plugins::CameraPlugins)
        .run();
}
