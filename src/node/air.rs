use bevy::prelude::*;

use crate::{
    grid::{
        common::BasicNode,
        component::{LeftClicked, Placed, TextureBuff},
        system_set::GridFixed,
        util::replace,
    }, node::{clay_distributor::ClayDistributor, clay_generator::ClayGenerator, clay_ore::ClayOre, clay_table::ClayTable, clay_unloader::ClayUnloader, conveyor::Conveyor, furnace::ClayFurnace, trash_can::TrashCan},
};

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
        commands
            .entity(entity)
            .insert((Air, TextureBuff("textures/tile/air.png".to_string())));
    }
    fn register(app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, on_right_clicked.in_set(GridFixed::MainUpdate));
    }
}

fn on_placed(mut commands: Commands, q: Query<Entity, (With<Placed>, With<Air>)>) {
    for e in q {
        commands.entity(e).remove::<Sprite>();
    }
}

fn on_right_clicked(
    mut commands: Commands,
    node_q: Query<Entity, (With<LeftClicked>, With<Air>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for entity in node_q {
        if keys.pressed(KeyCode::Digit0) {
            replace::<ClayOre>(&mut commands, entity);
        } else if keys.pressed(KeyCode::Digit1) {
            replace::<Conveyor>(&mut commands, entity);
        } else if keys.pressed(KeyCode::Digit2) {
            replace::<ClayFurnace>(&mut commands, entity);
        } else if keys.pressed(KeyCode::Digit3) {
            replace::<ClayTable>(&mut commands, entity);
        } else if keys.pressed(KeyCode::Digit4) {
            replace::<ClayUnloader>(&mut commands, entity);
        } else if keys.pressed(KeyCode::Digit5) {
            replace::<ClayGenerator>(&mut commands, entity);
        } else if keys.pressed(KeyCode::Digit6) {
            replace::<TrashCan>(&mut commands, entity);
        } else if keys.pressed(KeyCode::Digit7) {
            replace::<ClayDistributor>(&mut commands, entity);
        }
    }
}
