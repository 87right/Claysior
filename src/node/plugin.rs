use bevy::prelude::*;

use crate::{grid::{common::BasicNode, component::Removed, resource::SpawnTable, system_set::GridFixed}, node::*};


pub struct NodePlugin;
impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        register::<air::Air>(app);
        register::<clay_ore::ClayOre>(app);
        register::<conveyor::Conveyor>(app);
    }
}

fn register<T: BasicNode + Component + 'static>(app: &mut App) {
    app.add_systems(Startup, spawn_table_insert::<T>);
    app.add_systems(FixedUpdate, (on_remove::<T>).in_set(GridFixed::OnRemoved));
    T::register(app);
}

fn spawn_table_insert<T: BasicNode>(
    mut spawn_table: ResMut<SpawnTable>
) {
    spawn_table.insert(T::get_id(), T::spawn);
}

fn on_remove<T: BasicNode + Component>(
    mut commands: Commands,
    t_q: Query<Entity, (With<T>, With<Removed>)>
) {
    for e in t_q {
        T::remove(&mut commands.entity(e));
    }
}
