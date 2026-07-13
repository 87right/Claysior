//! # Path: src/nodes/clay_ore.rs

use bevy::prelude::*;
use crate::constants::*;
use crate::grid::{
    messages::*,
    components::*,
};
use crate::commons::*;
use crate::movables::item::*;
use crate::nodes::{
    commons::*,
    empty::Empty,
};

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
impl Spawnable for ClayOre {
    fn get_bundle() -> impl Bundle {
        (
            ClayOre {
                health: 5,
            },
        )
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
        if let Ok((mut val, grid_pos)) = q.get_mut(clicked_entity) {
            val.health -= 1;
            if val.health == 0 {
                replace::<ClayOre, Empty>(&mut command, &mut writer, clicked_entity);
                let pos = grid_pos.to_bottom_left_vec2();
                command.spawn((
                    Item {
                        id: Type::Clay,
                        size: 1,
                    },
                    Type::Clay.get_sprite(&asset_server),
                    Transform::from_xyz(
                        pos.x,
                        pos.y,
                        1.
                    )
                ));
            } else {
                command.entity(clicked_entity).insert(
                    Shake {
                        base_x: grid_pos.to_bottom_left_vec2().x,
                        scale: CELL_SIZE / 16.,
                        pace: 0.05,
                        timer: Timer::from_seconds(0.1, TimerMode::Once),
                    }
                );
            }
        }
    }
}


fn on_placed(
    mut commands: Commands,
    mut reader: MessageReader<Placed>,
    mut q : Query<(), With<ClayOre>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if q.get_mut(clicked_entity).is_ok() {
            commands.entity(clicked_entity).insert(Sprite::from_image(
                asset_server.load("textures/tile/clay_ore.png")
            ));
        }
    }
}
