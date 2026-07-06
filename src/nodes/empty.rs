//! # Path: src/nodes/empty.rs

use bevy::prelude::*;
use crate::grid::messages::*;
use crate::nodes::clay_ore::ClayOre;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Empty;
impl Registerable for Empty {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_right_clicked,
            on_left_clicked,
        ));
        app.add_systems(PostUpdate, on_placed);
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
    mut command: Commands,
    mut lc: MessageReader<LeftClicked>,
    mut writer: MessageWriter<Placed>,
    q : Query<&Empty>,
) {
    for m in lc.read() {
        let clicked_entity = m.0;
        if let Ok(_) = q.get(clicked_entity) {
            command.entity(clicked_entity).remove::<Empty>();
            command.entity(clicked_entity).insert(ClayOre {health: 5});
            writer.write(Placed (clicked_entity));
        }
    }
}

fn on_placed(
    mut reader: MessageReader<Placed>,
    mut q : Query<&mut Sprite, With<Empty>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if let Ok(mut sprite) = q.get_mut(clicked_entity) {
            *sprite = Sprite::from_image(
                asset_server.load("textures/tile.png")
            );
        }
    }
}
