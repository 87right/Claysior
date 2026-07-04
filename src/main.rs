/* src/main.rs */

#![allow(dead_code)]

mod grid;
mod constants;

use bevy::{
    prelude::*,
    input::mouse::*,
};

use crate::grid::{
    components::*,
    plugins::*,
};
use crate::constants::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GridPlugins)
        .add_plugins(Setup)
        .add_plugins(Input)
        .run();
}

pub struct Setup;
impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands){
    commands.spawn((
        Camera2d,
        Camera::default(),
        Transform::from_xyz (
            MAP_WIDTH  as f32 * CELL_SIZE / 2.,
            MAP_HEIGHT as f32 * CELL_SIZE / 2.,
            100.
        ),
    ));
}

#[derive(Resource, Default)]
struct CameraDragData {
    last_cursor_pos: Vec2,
    last_camera_pos: Vec3,
}

struct Input;
impl Plugin for Input {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraDragData::default());
        app.add_systems(Update, (
            camera_movement_system,
            camera_zoom_system,
        ));
    }
}

fn camera_movement_system(
    mut camera_drag_data: ResMut<CameraDragData>, 
    camera_query: Single<&mut Transform, With<Camera>>,
    buttons: Res<ButtonInput<MouseButton>>, 
    window: Single<&Window>
) {
    let mut transform = camera_query.into_inner();
    if let Some(position) = window.cursor_position(){
        

        if buttons.just_pressed(MouseButton::Middle) {
            camera_drag_data.last_cursor_pos = position;
            camera_drag_data.last_camera_pos = transform.translation;
        }

        if buttons.pressed(MouseButton::Middle) {
            transform.translation = camera_drag_data.last_camera_pos + (camera_drag_data.last_cursor_pos - position).extend(0.)*Vec3 {x: 1., y:-1., z: 1.};
            camera_drag_data.last_cursor_pos = position;
            camera_drag_data.last_camera_pos = transform.translation;
        }
    }
}

fn camera_zoom_system(
    mut msr_scroll: MessageReader<MouseWheel>,
    projection_query: Single<&mut Projection, With<Camera>>,
) {
    let mut projection = projection_query.into_inner();
    if let Projection::Orthographic(ref mut orthographic) = *projection {
        for ms in msr_scroll.read() {
            const ZOOM_SPD: f32 = 0.1;
            orthographic.scale = (orthographic.scale - ms.y * ZOOM_SPD).clamp(0.1, 10.);
        }
    }
}
