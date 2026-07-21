use bevy::prelude::*;

use crate::{common::constant::CELL_SIZE, grid::{
    common::BasicNode, component::{GridPos, PlaceBuff}, message::Removed, resource::{Background, GridEntityMap, GridGenSetting}
}};

pub fn replace<To: BasicNode>(
    commands: &mut Commands,
    removed_message_writer: &mut MessageWriter<Removed>,
    e: Entity, 
) {
    removed_message_writer.write(Removed(e));
    commands.entity(e).insert(PlaceBuff(To::get_id()));
}

pub fn respawn_grid(
    mut commands: Commands,
    mut grid_map: ResMut<GridEntityMap>,
    setting: Res<GridGenSetting>,
) {
    for y in 0..setting.height {
        for x in 0..setting.width {
            let cur_pos = GridPos(ivec2(x as i32, y as i32));
            let new_entity = commands.spawn(PlaceBuff::from_str("air")).id();
            if let Some(last_entity) = grid_map.insert(&cur_pos, new_entity) {
                commands.entity(last_entity).despawn();
            }
        }
    }
}

pub fn reload_background(
    mut commands: Commands,
    mut background: ResMut<Background>,
    setting: Res<GridGenSetting>,
    asset_server: Res<AssetServer>,
) {
    let sprite = (
        Sprite::from_image(
            asset_server.load(format!("textures/background/{}", &setting.background))
        ),
        Transform::from_xyz(
            CELL_SIZE*(setting.width  as f32 / 2. - 0.5),
            CELL_SIZE*(setting.height as f32 / 2. - 0.5),
            0.
        )
    );
    if let Some(entity) = background.get() {
        commands.entity(entity).insert(sprite);
    } else {
        background.set(
            commands.spawn(sprite).id()
        );
    }
}
