//! # Path: src/nodes/empty.rs

use bevy::prelude::*;
use crate::grid::messages::*;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Empty;
impl Registerable for Empty {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_right_clicked,
            on_left_clicked,
        ));
    }
}

fn on_right_clicked(
    mut rc: MessageReader<RightClicked>,
    mut q : Query<&mut Sprite, With<Empty>>,
    asset_server: Res<AssetServer>,
) {
    for m in rc.read() {
        let clicked_entity = m.0;
        if let Ok(mut sprite) = q.get_mut(clicked_entity) {
            *sprite = Sprite::from_image(
                asset_server.load("textures/debug_tile.png")
            );
        }
    }
}

fn on_left_clicked(
    mut lc: MessageReader<LeftClicked>,
    mut q : Query<&mut Sprite, With<Empty>>,
    asset_server: Res<AssetServer>,
) {
    for m in lc.read() {
        let clicked_entity = m.0;
        if let Ok(mut sprite) = q.get_mut(clicked_entity) {
            *sprite = Sprite::from_image(
                asset_server.load("textures/tile.png")
            );
        }
    }
}
