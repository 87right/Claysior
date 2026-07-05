//! # Path: src/grid/plugins.rs

use bevy::prelude::*;
use crate::grid::components::*;
use crate::constants::*;

pub struct GridPlugins;
impl Plugin for GridPlugins {
    fn build(&self, app: &mut App) {
        add_resource(app);
        add_systems_startup(app);
    }
}

fn add_resource(app: &mut App) {
    app.insert_resource(WorldGrid::default());
}

fn add_systems_startup(app: &mut App) {
    app.add_systems(Startup, create_empty_world_grid);
}

fn create_empty_world_grid(
    mut world_grid: ResMut<WorldGrid>, 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let v = IVec2 {x, y};
            let id = commands.spawn((
                GridPos (v), 
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
