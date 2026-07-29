use bevy::prelude::*;

use crate::{
    grid::{
        common::*,
        component::*,
        util::*,
        system_set::*,
    },
    node::{
        air::Air,
    }
};

#[derive(Component)]
pub struct ClayTable;
impl BasicNode for ClayTable {
    fn get_id() -> String {
        "clay_table".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(FixedUpdate, on_clicked.in_set(GridFixed::MainUpdate));
    }
    fn remove(commands: &mut EntityCommands) {
        commands.remove::<ClayTable>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            ClayTable,
            TextureBuff("textures/tile/clay_table.png".to_string()),
        ));
    }
}

fn on_clicked(
    mut commands: Commands,
    table_q: Query<Entity, (With<LeftClicked>, With<ClayTable>)>,
) {
    for entity in table_q {
        replace::<Air>(&mut commands, entity);
    }
}
