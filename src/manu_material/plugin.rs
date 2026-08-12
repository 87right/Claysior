use crate::prelude::*;

pub struct ManuMaterialPlugin;
impl Plugin for ManuMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, logistics_system::<Item>);
    }
}

fn logistics_system<T>(
    mut log_node_q: Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos)>,
    grid: Res<GridEntityMap>
) 
where 
    T: ManuMaterial
{
    let mut orders = Vec::<LogisticsOrder::<T>>::default();
    for (mut channel, _, pos) in &mut log_node_q {
        channel.pull_order(*pos, &mut orders);
    }
    for mut order in orders {
        if let Some(from_entity) = grid.get(order.from)
        && let Ok((mut from_channel, mut from_inventory, _from_pos)) = (&mut log_node_q).get_mut(from_entity) {
            from_channel.write_order(&mut from_inventory, &mut order);
        } else {
            continue;
        }
        if let Some(to_entity) = grid.get(order.to)
        && let Ok((mut to_channel, mut to_inventory, _to_pos)) = (&mut log_node_q).get_mut(to_entity) {
            to_channel.response_order(&mut to_inventory, &mut order);
        } else {
            continue;
        }
        if let Some(from_entity) = grid.get(order.from)
        && let Ok((mut from_channel, mut from_inventory, _from_pos)) = (&mut log_node_q).get_mut(from_entity) {
            from_channel.check_order(&mut from_inventory, &order);
        } else {
            continue;
        }
    }
}
