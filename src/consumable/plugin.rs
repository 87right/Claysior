use bevy::prelude::*;

use crate::{
    consumable::{common::*, component::*},
    grid::{component::*, resource::*, system_set::*},
    item::component::Item,
};

pub struct ConsumablePlugin;
impl Plugin for ConsumablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                (logistics_system::<Item>).in_set(GridFixed::IOExecute),
                (channel_update::<Item>).in_set(GridFixed::ApplyDiff),
            ),
        );
    }
}

fn logistics_system<T>(
    mut channel_q: Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    grid: Res<GridEntityMap>,
) where
    T: Consumable,
{
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
        let Some(mut buff) = get_buff::<T>(&channel_q, port, e) else {
            continue;
        };
        let tasks = get_entity_tasks::<T>(&channel_q, port, e, &grid);
        for e2 in tasks {
            if e != e2 
            && input::<T>(&mut channel_q, &mut buff, e2, e, from_pos, &grid) {
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
                    && let Some(buff) = get_buff(&channel_q, *port, open_entity) {
                        open_ports.push((buff, open_entity));
                    }
                }
            }
        }
        for (buff, open_entity) in &mut open_ports {
            let mut open_pulled = None;
            if let Ok((mut channel, mut inventory, _, _)) = channel_q.get_mut(pull_entity) {
                if pull_entity != *open_entity
                && let Some(pull_port) = channel.pull.get_mut(index)
                && pull_port.insert(&mut inventory, &mut buff.content) {
                    open_pulled = Some((buff.clone(), *open_entity));
                    channel.inserted(index);
                }
            }
            if let Some((buff, open_entity)) = open_pulled 
            && let Ok((_, mut inventory, _, _)) = channel_q.get_mut(open_entity) {
                inventory.apply_buff(buff);
                break;
            }
        }
    }
}

fn get_buff<T>(
    channel_q: &Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    port: Port<T>,
    e: Entity,
) -> Option<MaterialSlotBuff<T>>
where
    T: Consumable,
{
    port.get_buff(channel_q.get(e).ok()?.1)
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
) -> bool
where
    T: Consumable,
{
    let Ok((mut c, mut i, pos, _)) = channel_q.get_mut(e) else {
        return false;
    };
    c.insert(&mut *i, buff, from_entity, from_pos, *pos, grid)
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
    channel.inserted(index);
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
    }
}