use crate::prelude::*;

pub struct ManuMaterialPlugin;
impl Plugin for ManuMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            logistics_system::<Item>.in_set(GridSystem::Logistics),
            handle_slot_cd::<Item>.in_set(GridSystem::CleanUp)
        ));
    }
}

fn logistics_system<T>(
    mut log_node_q: Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos)>,
    mut writer: MessageWriter<MaterialMoved>,
    grid: Res<GridEntityMap>
) 
where 
    T: ManuMaterial
{
    let mut orders = Vec::<LogisticsOrder::<T>>::default();
    for (mut channel, _, pos) in &mut log_node_q {
        channel.pull_order(*pos, &mut orders);
    }
    for order in &mut orders {
        if let Some(from_entity) = grid.get(order.from)
        && let Ok((mut from_channel, mut from_inventory, _from_pos)) = (&mut log_node_q).get_mut(from_entity) {
            from_channel.write_order(&mut from_inventory, order);
        } else {
            continue;
        }
        if let Some(to_entity) = grid.get(order.to)
        && let Ok((mut to_channel, mut to_inventory, _to_pos)) = (&mut log_node_q).get_mut(to_entity) {
            to_channel.response_order(&mut to_inventory, order);
        } else {
            continue;
        }
        if let Some(from_entity) = grid.get(order.from)
        && let Ok((mut from_channel, mut from_inventory, _from_pos)) = (&mut log_node_q).get_mut(from_entity) {
            from_channel.check_order(&mut from_inventory, order);
        } else {
            continue;
        }
    }
    for order in orders {
        if order.is_done() {
            let to = order.to;
            let wrapped_moved_data: Vec<_> = 
                order.moved_slot_id.iter().map(move |id| { 
                    MaterialSlotIdentifyer {
                        pos: to,
                        slot_id: *id
                    }
                }).collect();
            writer.write(MaterialMoved { 
                from: MaterialSlotIdentifyer { 
                    pos: order.from, 
                    slot_id: SlotID(order.client_id) 
                }, 
                to: wrapped_moved_data, 
                is_taken: order.slot.is_some_and(|x| x.slot.get_raw().0.is_none()),
                duration: order.cd_ticks as f32 / GameSetting::TPS as f32
            });
        }
    }
}

fn handle_slot_cd<T>(
    inv_q: Query<&mut Inventory<T>>
)
where
    T: ManuMaterial
{
    for mut inv in inv_q {
        for slot in inv.iter_mut() {
            slot.update();
        }
    }
}
