//! # Path: src/nodes/empty.rs

use bevy::prelude::*;
use crate::grid::messages::*;
use crate::nodes::{
    clay_ore::ClayOre,
    conveyor::Conveyor,
    item_collector::*,
    commons::*,
};
use crate::commons::*;

#[derive(Component, Default)]
pub struct Empty;
impl Registerable for Empty {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_left_clicked,
        ));
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Spawnable for Empty {
    fn get_bundle() -> impl Bundle {
        (
            Empty,
        )
    }
}

fn on_left_clicked(
    mut command: Commands,
    mut lc: MessageReader<LeftClicked>,
    mut writer: MessageWriter<Placed>,
    q : Query<&Empty>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for m in lc.read() {
        let clicked_entity = m.0;
        if let Ok(_) = q.get(clicked_entity) {
            if keys.pressed(KeyCode::Digit1) {
                replace::<Empty, ClayOre>(&mut command, &mut writer, clicked_entity);
            } else if keys.pressed(KeyCode::Digit2) {
                replace::<Empty, Conveyor>(&mut command, &mut writer, clicked_entity);
            } else if keys.pressed(KeyCode::Digit3) {
                replace::<Empty, ItemCollector>(&mut command, &mut writer, clicked_entity);
            }
        }
    }
}

fn on_placed(
    mut command: Commands,
    mut reader: MessageReader<Placed>,
    q : Query<(), With<Empty>>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if q.get(clicked_entity).is_ok() {
            command.entity(clicked_entity).remove::<Sprite>();
        }
    }
}
