//! # Path: src/camera/components.rs

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CameraDragData {
    pub last_cursor_pos: Vec2,
    pub last_camera_pos: Vec3,
}
