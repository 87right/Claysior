use bevy::prelude::*;

use crate::{grid::{common::BasicNode, component::{LeftClicked, Placed, TextureBuff}, system_set::GridFixed, util::replace}, node::clay_ore::ClayOre};

#[derive(Component)]
pub struct Air;
impl BasicNode for Air {
    fn get_id() -> String {
        "air".to_string()
    }
    fn remove(commands: &mut bevy::ecs::system::EntityCommands) {
        commands.remove::<Air>();
    }
    fn spawn(commands: &mut bevy::ecs::system::Commands, entity: bevy::ecs::entity::Entity) {
        commands.entity(entity).insert((
            Air,
            TextureBuff("textures/tile/air.png".to_string()),
        ));
    }
    fn register(app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, on_right_clicked.in_set(GridFixed::MainUpdate));
    }
}

fn on_placed(
    mut commands: Commands,
    q: Query<Entity, (With<Placed>, With<Air>)>,
) {
    for e in q {
        commands.entity(e).remove::<Sprite>();
    }
}

fn on_right_clicked(
    mut commands: Commands,
    node_q: Query<Entity, (With<LeftClicked>, With<Air>)>,
) {
    for entity in node_q {
        replace::<ClayOre>(&mut commands, entity);
    }
}
