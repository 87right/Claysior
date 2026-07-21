use bevy::prelude::*;

use crate::{grid::{common::BasicNode, resource::SpawnTable}, node::*};


pub struct NodePlugin;
impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        register::<air::Air>(app);
    }
}

fn register<T: BasicNode + 'static>(app: &mut App) {
    app.add_systems(Startup, spawn_table_insert::<T>);
    T::register(app);
}

fn spawn_table_insert<T: BasicNode>(
    mut spawn_table: ResMut<SpawnTable>
) {
    spawn_table.insert(T::get_id(), T::spawn);
}
