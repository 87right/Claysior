//! # Path: src/nodes/clay_ore.rs

use bevy::prelude::*;
use crate::grid::messages::*;
use crate::nodes::commons::*;

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

fn on_left_clicked(
    mut command: Commands,
    mut rc: MessageReader<LeftClicked>,
    mut q : Query<&mut ClayOre>,
    mut writer: MessageWriter<Placed>,
) {
    for m in rc.read() {
        let clicked_entity = m.0;
        if let Ok(mut val) = q.get_mut(clicked_entity) {
            if val.health == 0 {
                command.entity(clicked_entity).remove::<ClayOre>();
                command.entity(clicked_entity).insert(crate::nodes::empty::Empty);
                writer.write(Placed (clicked_entity));

            } else {
                val.health -= 1;
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
                asset_server.load("textures/clay_ore.png")
            );
        }
    }
}
