use bevy::prelude::*;

use crate::{
    consumable::{common::*, component::*, resource::RegisteredSlot}, grid::{component::*, resource::*, system_set::*}, item::component::Item,
};

pub struct ConsumablePlugin;
impl Plugin for ConsumablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                (logistics_system::<Item>).in_set(GridFixed::IOExecute),
                (
                    channel_update::<Item>,
                    inventory_update::<Item>,
                ).in_set(GridFixed::ApplyDiff),
            ),
        );
        app.insert_resource(RegisteredSlot::default());
    }
}

fn logistics_system<T>(
    mut commands: Commands,
    mut channel_q: Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    mut registered_slot: ResMut<RegisteredSlot>,
    grid: Res<GridEntityMap>,
) where
    T: Consumable,
{
    let mut t_moved: Vec<(Vec2, Vec2, u64, String)> = vec![];

    let mut active_tasks: Vec<(Port<T>, Entity, GridPos, usize)> = vec![];
    let mut passive_tasks: Vec<(Port<T>, Entity, GridPos, usize)> = vec![];
    for (channel, _, pos, e) in channel_q.as_readonly() {
        for (index, port) in channel.output.iter().enumerate() {
            active_tasks.push((*port, e, *pos, index));
        }
        for (index, port) in channel.pull.iter().enumerate() {
            passive_tasks.push((*port, e, *pos, index));
        }
    }
    for (port, e, from_pos, index) in active_tasks {
        let Some((Some(mut buff), time_cost)) = get_buff::<T>(&channel_q, port, e) else {
            continue;
        };
        let tasks = get_entity_tasks::<T>(&channel_q, port, e, &grid);
        for e2 in tasks {
            let mut pos = GridPos(ivec2(0, 0));
            let Some(item) = buff.content.val else {continue;};
            if e != e2 
            && input::<T>(&mut channel_q, &mut buff, e2, e, from_pos, &grid, time_cost, &mut registered_slot, &mut pos) {
                t_moved.push((from_pos.to_world_pos(), pos.to_world_pos(), time_cost, format!("textures/item/{}.png", item.get_id())));
                break;
            }
        }
        apply(&mut channel_q, e, buff, index);
    }
    for (pull_port, pull_entity, pull_pos, index) in passive_tasks {
        let tasks = get_entity_tasks::<T>(&channel_q, pull_port, pull_entity, &grid);
        let mut open_ports = vec![];
        for open_entity in tasks {
            if let Ok((channel, _, open_pos, _)) = channel_q.get(open_entity) {
                for port in &channel.open {
                    if port.grid.check(*open_pos, pull_pos)
                    && let Some((Some(buff), _)) = get_buff(&channel_q, *port, open_entity) {
                        open_ports.push((buff, open_entity, *open_pos));
                    }
                }
            }
        }
        for (buff, open_entity, open_pos) in &mut open_ports {
            let mut open_pulled = None;
            let Some(item) = buff.content.val else {continue;};
            if let Ok((mut channel, mut inventory, _, _)) = channel_q.get_mut(pull_entity) {
                let time_cost = channel.time_cost;
                if pull_entity != *open_entity
                && let Some(pull_port) = channel.pull.get_mut(index)
                && pull_port.insert(&mut inventory, &mut buff.content, time_cost, &mut registered_slot, pull_entity) {
                    open_pulled = Some((buff.clone(), *open_entity));
                    channel.inserted(PortType::Pull, index);
                    t_moved.push((open_pos.to_world_pos(), pull_pos.to_world_pos(), time_cost, item.get_id()));
                }
            }
            if let Some((buff, open_entity)) = open_pulled 
            && let Ok((_, mut inventory, _, _)) = channel_q.get_mut(open_entity) {
                inventory.apply_buff(buff);
                break;
            }
        }
    }

    for (from, to, ticks, texture_source) in t_moved {
        crate::gui::util::spawn_free_sprite(&mut commands, from, to, ticks, texture_source);
    }
}

fn get_buff<T>(
    channel_q: &Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    port: Port<T>,
    e: Entity,
) -> Option<(Option<MaterialSlotBuff<T>>, u64)>
where
    T: Consumable,
{
    let (channel, inventory, _, _) = channel_q.get(e).ok()?;
    Some((port.get_buff(inventory), channel.time_cost))
}

fn get_entity_tasks<T>(
    channel_q: &Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    port: Port<T>,
    e: Entity,
    grid: &Res<GridEntityMap>,
) -> Vec<Entity>
where
    T: Consumable,
{
    let Ok((_, _, p, _)) = channel_q.get(e) else {
        return vec![];
    };
    port.get_target_entity(*p, grid)
}

fn input<T>(
    channel_q: &mut Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    buff: &mut MaterialSlotBuff<T>,
    e: Entity,
    from_entity: Entity,
    from_pos: GridPos,
    grid: &Res<GridEntityMap>,
    time_cost: u64,
    registered_slot: &mut RegisteredSlot,
    res_pos: &mut GridPos,
) -> bool
where
    T: Consumable,
{
    let Ok((mut c, mut i, pos, _)) = channel_q.get_mut(e) else {
        return false;
    };
    *res_pos = *pos;
    c.insert(
        &mut *i, 
        buff, 
        from_entity, 
        from_pos, 
        e, 
        *pos, 
        grid, 
        registered_slot, 
        time_cost
    )
}

fn apply<T>(
    channel_q: &mut Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    e: Entity,
    buff: MaterialSlotBuff<T>,
    index: usize,
) where
    T: Consumable,
{
    let Ok((mut channel, mut inv, _, _)) = channel_q.get_mut(e) else {
        return;
    };
    channel.inserted(PortType::Output, index);
    inv.apply_buff(buff);
}


fn channel_update<T> (
    channel_q: Query<(&mut Channel<T>, &Inventory<T>)>
) where 
    T: Consumable,
{
    for (mut channel, inventory) in channel_q {
        for port in &mut channel.input {
            port.mode.update();
        }
        for port in &mut channel.output {
            if port.get_first(inventory).is_some() {
                port.mode.update();
            }
        }
        for port in &mut channel.pull {
            port.mode.update();
        }
        for port in &mut channel.open {
            port.mode.update();
        }
    }
}

fn inventory_update<T> (
    mut registered_slot: ResMut<RegisteredSlot>,
    mut inventory_q: Query<&mut Inventory<T>>,
) where 
    T: Consumable,
{
    let mut valid_list = vec![];
    for (index, (entity, slot_id)) in registered_slot.0.iter().enumerate() {
        if let Ok(mut inventory) = inventory_q.get_mut(*entity) 
        && let Some(slot) = inventory.get_mut(*slot_id) {
            if slot.update_and_is_valid() {
                valid_list.push(index);
            }
        }
    }
    for index in valid_list {
        registered_slot.remove(index);
    }
}