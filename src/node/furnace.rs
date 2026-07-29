use bevy::prelude::*;

use crate::grid::{
    common::*,
    component::*,
    util::*,
    system_set::*,
};

#[derive(Component)]
pub struct ClayFurnace;
impl BasicNode for ClayFurnace {
    fn get_id() -> String {
        "clay_furnace".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(FixedUpdate, on_clicked.in_set(GridFixed::MainUpdate));
    }
    fn remove(commands: &mut EntityCommands) {
        commands.remove::<ClayFurnace>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            ClayFurnace,
            TextureBuff("textures/tile/clay_furnace_0.png".to_string()),
        ));
    }
}

fn on_clicked(
    mut commands: Commands,
    furnace_q: Query<Entity, (With<ClayFurnace>, With<LeftClicked>)>,
) {
    for entity in furnace_q {
        replace::<crate::node::air::Air>(&mut commands, entity);
    }
}
