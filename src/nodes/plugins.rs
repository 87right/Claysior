//! # Path: src/nodes/plugins.rs

use bevy::prelude::*;
use crate::constants::PIE;
use crate::nodes::commons::ItemSendReq;
use crate::nodes::commons::Shake;
use crate::nodes::*;
use crate::commons::*;

pub struct NodePlugins;
impl Plugin for NodePlugins {
    fn build(&self, app: &mut App) {
        app.add_message::<ItemSendReq>();

        app.add_systems(Update, shaking);

        register::<empty::Empty>(app);
        register::<clay_ore::ClayOre>(app);
        register::<conveyor::Conveyor>(app);
        register::<item_collector::ItemCollector>(app);
        register::<clay_furnace::ClayFurnace>(app);
    }
}

fn register<T: Registerable> (app: &mut App) {
    T::register(app);
}

fn shaking(
    mut command: Commands,
    q: Query<(&mut Shake, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut shake, mut transform, entity) in q {
        if shake.timer.tick(time.delta()).just_finished() {
            command.entity(entity).remove::<Shake>();
        }
        transform.translation.x = shake.base_x + shake.scale * (shake.timer.remaining().as_secs_f32() * 2. * PIE / shake.pace).sin();
    }
}
