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
    let mut tasks: Vec<(Port<T>, Entity, GridPos, usize)> = vec![];
    for (channel, _, pos, e) in channel_q.as_readonly() {
        for (index, port) in channel.output.iter().enumerate() {
            tasks.push((*port, e, *pos, index));
        }
    }
    for (port, e, from_pos, index) in tasks {
        let Some(mut buff) = get_buff::<T>(&channel_q, port, e) else {
            continue;
        };
        let tasks = get_input_tasks::<T>(&channel_q, port, e, &grid);
        for e2 in tasks {
            if e != e2 
            && input::<T>(&mut channel_q, &mut buff, e2, e, from_pos, &grid) {
                break;
            }
        }
        apply(&mut channel_q, e, buff, index);
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

fn get_input_tasks<T>(
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