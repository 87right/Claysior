use bevy::prelude::*;

use crate::{
    consumable::{
        component::*,
        common::*,
    },
    grid::{
        resource::*,
        component::*,
        system_set::*,
    },
    item::component::Item,
};

pub struct ConsumablePlugin;
impl Plugin for ConsumablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (logistics_system::<Item>).in_set(GridFixed::IOExecute));
    }
}

fn logistics_system<T>(
    mut channel_q: Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    grid: Res<GridEntityMap>,
) 
where 
    T: Consumable
{
    let mut tasks: Vec<(Port<T>, Entity)> = vec![];
    for (channel, _, _, e) in channel_q.as_readonly() {
        for port in &channel.output {
            tasks.push((port.clone(), e));
        }
    }
    for (port, e) in tasks {
        let Some(mut buff) = get_buff::<T>(&channel_q, port, e) else { continue;};
        let tasks = get_input_tasks::<T>(&channel_q, port, e, &grid);
        for e in tasks {
            if input::<T>(&mut channel_q, &mut buff, e) {
                break;
            }
        }
        apply(&mut channel_q, e, buff);
    }
}

fn get_buff<T>(
    channel_q: &Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    port: Port<T>,
    e: Entity,
) -> Option<MaterialSlotBuff<T>> 
where 
    T: Consumable
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
    T: Consumable
{
    let Ok((_, _, p, _)) = channel_q.get(e) else {
        return vec![];
    };
    port.get_target_entity(*p, grid)
}

fn input<T>(
    channel_q: &mut Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    buff: &mut MaterialSlotBuff<T>,
    e: Entity
) -> bool
where 
    T: Consumable
{
    let Ok((mut c, mut i, _, _)) = channel_q.get_mut(e) else {
        return false;
    };
    c.insert(&mut *i, buff)
}

fn apply<T>(
    channel_q: &mut Query<(&mut Channel<T>, &mut Inventory<T>, &GridPos, Entity)>,
    e: Entity,
    buff: MaterialSlotBuff<T>,
) 
where 
    T: Consumable
{
    let Ok((_, mut inv, _, _)) = channel_q.get_mut(e) else {return;};
    inv.apply_buff(buff);
}
