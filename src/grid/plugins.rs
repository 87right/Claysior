//! # Path: src/grid/plugins.rs

use bevy::prelude::*;
use crate::grid::{
    components::*,
    messages::*,
};
use crate::constants::*;

pub struct GridPlugins;
impl Plugin for GridPlugins {
    fn build(&self, app: &mut App) {
        add_messages(app);
        add_resource(app);
        add_systems_startup(app);
        add_systems_update(app);
    }
}

fn add_messages(app: &mut App) {
    app.add_message::<RightClicked>();
    app.add_message::<LeftClicked>();
    app.add_message::<NetworkUpdated>();
    app.add_message::<Placed>();
    app.add_message::<Broken>();
}

fn add_resource(app: &mut App) {
    app.insert_resource(WorldGrid::default());
}

fn add_systems_startup(app: &mut App) {
    app.add_systems(Startup, create_empty_world_grid);
}

fn add_systems_update(app: &mut App) {
    app.add_systems(Update, (
        handle_right_click,
        handle_left_click,
    ));
}

fn create_empty_world_grid(
    mut world_grid: ResMut<WorldGrid>, 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        BackGround,
        Transform::from_xyz(
            CELL_SIZE*(MAP_WIDTH as f32 / 2. - 0.5),
            CELL_SIZE*(MAP_HEIGHT as f32 / 2. - 0.5),
            0.
        ),
        Sprite::from_image(
            asset_server.load("textures/background/basic_tile.png")
        )
    ));
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let v = IVec2 {x, y};
            let id = commands.spawn((
                GridPos (v), 
                crate::nodes::empty::Empty {},
                Transform::from_xyz (
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    0.
                ),
            )).id();
            world_grid.0.insert(v, id);
        }
    }
}

fn handle_right_click(
    mut right: MessageWriter<RightClicked>,
    world_grid: Res<WorldGrid>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_and_transform: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = camera_and_transform.into_inner();
    if let Some(cursor_position) = window.cursor_position()
    && let Ok(world_cursor_position) = camera.viewport_to_world_2d(transform, cursor_position)
    && let Some(entity) = world_grid.0.get(&IVec2 {
        x: ((world_cursor_position.x + CELL_SIZE / 2.) / CELL_SIZE).floor() as i32, 
        y: ((world_cursor_position.y + CELL_SIZE / 2.) / CELL_SIZE).floor() as i32,
    })
    && mouse_button.just_released(MouseButton::Right) {
        right.write(RightClicked (*entity));
    }
}

fn handle_left_click(
    mut left: MessageWriter<LeftClicked>,
    world_grid: Res<WorldGrid>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_and_transform: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, transform) = camera_and_transform.into_inner();
    if let Some(cursor_position) = window.cursor_position()
    && let Ok(world_cursor_position) = camera.viewport_to_world_2d(transform, cursor_position)
    && let Some(entity) = world_grid.0.get(&IVec2 {
        x: ((world_cursor_position.x + CELL_SIZE / 2.) / CELL_SIZE).floor() as i32, 
        y: ((world_cursor_position.y + CELL_SIZE / 2.) / CELL_SIZE).floor() as i32,
    })
    && mouse_button.just_released(MouseButton::Left) {
        left.write(LeftClicked (*entity));
    }
}
