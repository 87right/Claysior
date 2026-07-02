use bevy::{
    prelude::*,
    color::palettes::css::*,
    input::mouse::*,
};
use std::collections::HashMap;

const CELL_SIZE: f32 = 64.;
const MAP_HEIGHT: i32 = 16;
const MAP_WIDTH: i32 = 16;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Setup)
        .add_plugins(Input)
        .run();
}

#[derive(Resource, Default)]
struct WorldGrid (HashMap<IVec2, Entity>);

#[derive(Component)]
enum GridState {
    Empty,
}

#[derive(Component)]
struct GridPos (IVec2);

pub struct Setup;
impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid::default());
        app.add_systems(Startup, (world_gen, setup_camera));
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

fn world_gen(
    mut world_grid: ResMut<WorldGrid>, 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let empty_color = Color::Srgba(AQUA);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let v = IVec2 {x, y};
            let id = commands.spawn((
                GridPos (v), 
                GridState::Empty,
                Sprite::from_image(
                    asset_server.load("textures/tile.png")
                ),
                Transform::from_xyz (
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    100.0
                ),
            )).id();
            world_grid.0.insert(v, id);
        }
    }
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
