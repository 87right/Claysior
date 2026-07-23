use bevy::prelude::*;

use crate::{grid::{common::BasicNode, component::{LeftClicked, TextureBuff}, system_set::GridFixed, util::replace}, node::air::Air};

#[derive(Component)]
pub struct ClayOre{
    health: u32
}
impl BasicNode for ClayOre {
    fn get_id() -> String {
        "clay_ore".to_string()
    }
    fn remove(commands: &mut bevy::ecs::system::EntityCommands) {
        commands.remove::<ClayOre>();
    }
    fn spawn(commands: &mut bevy::ecs::system::Commands, entity: bevy::ecs::entity::Entity) {
        commands.entity(entity).insert((
            ClayOre{
                health: 5
            },
            TextureBuff("textures/tile/clay_ore_5.png".to_string()),
        ));
    }
    fn register(app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, on_left_clicked.in_set(GridFixed::MainUpdate));

    }
}


fn on_left_clicked(
    mut commands: Commands,
    node_q: Query<(&mut ClayOre, Entity), With<LeftClicked>>,
) {
    for (mut ore, e) in node_q {
        ore.health -= 1;
        if ore.health == 0 {
            replace::<Air>(&mut commands, e);
        } else {
            commands.entity(e).insert(TextureBuff(format!("textures/tile/clay_ore_{}.png", ore.health).to_string()));
        }
    }
}
