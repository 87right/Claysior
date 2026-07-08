//! # Path: src/nodes/clay_ore.rs

use bevy::prelude::*;
use crate::constants::*;
use crate::grid::{
    messages::*,
    components::*,
};
use crate::commons::*;
use crate::movables::item::*;

#[derive(Component)]
pub struct ClayOre {
    pub health: u32
}
impl Registerable for ClayOre {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_left_clicked,
        ));
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Default for ClayOre {
    fn default() -> Self {
        Self {
            health: 5
        }
    }
}

fn on_left_clicked(
    mut command: Commands,
    mut rc: MessageReader<LeftClicked>,
    mut q : Query<(&mut ClayOre, &GridPos)>,
    mut writer: MessageWriter<Placed>,
    asset_server: Res<AssetServer>,
) {
    for m in rc.read() {
        let clicked_entity = m.0;
        if let Ok((mut val, GridPos (grid_pos))) = q.get_mut(clicked_entity) {
            val.health -= 1;
            if val.health == 0 {
                command.entity(clicked_entity).remove::<ClayOre>();
                command.entity(clicked_entity).insert(crate::nodes::empty::Empty);
                writer.write(Placed (clicked_entity));

                command.spawn((
                    Item {
                        id: Type::Clay,
                        size: 1,
                    },
                    Type::Clay.get_sprite(&asset_server),
                    Transform::from_xyz(
                        grid_pos.x as f32 * CELL_SIZE,
                        grid_pos.y as f32 * CELL_SIZE,
                        1.
                    )
                ));
            }
        }
    }
}


fn on_placed(
    mut reader: MessageReader<Placed>,
    mut q : Query<&mut Sprite, With<ClayOre>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if let Ok(mut sprite) = q.get_mut(clicked_entity) {
            *sprite = Sprite::from_image(
                asset_server.load("textures/tile/clay_ore.png")
            );
        }
    }
}
