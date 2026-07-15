//! # Path: src/nodes/plugins.rs

use bevy::prelude::*;
use crate::constants::PIE;
use crate::nodes::commons::InputPort;
use crate::nodes::commons::Inventory;
use crate::nodes::commons::ItemSendReq;
use crate::nodes::commons::Shake;
use crate::nodes::*;
use crate::commons::*;

pub struct NodePlugins;
impl Plugin for NodePlugins {
    fn build(&self, app: &mut App) {
        app.add_message::<ItemSendReq>();

        app.add_systems(Update, (
            shaking,
            handle_auto_input,
        ));

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

fn handle_auto_input(
    mut reader: MessageReader<ItemSendReq>,
    mut input_q: Query<&mut InputPort>,
    mut inv_q: Query<&mut Inventory>,
) {
    for m in reader.read() {
        if let Ok(mut port) = input_q.get_mut(m.to) {
            let mut can_process = false;
            if let Ok(from_inv) = inv_q.get(m.from)
            && let Ok(to_inv) = inv_q.get(m.to) 
            && let Some(try_item) = from_inv.check_item(m.index) {
                can_process = to_inv.can_insert(*port, *try_item);
            }
            if !can_process {continue;}
            let item = if let Ok(mut inv) = inv_q.get_mut(m.from) {
                inv.take_item(m.index)
            } else {
                None
            };
            if let Some(item) =  item
            && let Ok(mut to_inv) = inv_q.get_mut(m.to){
                to_inv.insert(item);
                port.recieved = true;
                port.display_item = m.e_item;
            }
        }
    }
}
