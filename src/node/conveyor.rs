use bevy::prelude::*;

use crate::{
    consumable::{common::{Consumable, PortType}, component::*}, grid::{common::*, component::*, resource::*, system_set::*, util::*}, item::component::Item, node::*,
};

#[derive(Component)]
pub struct Conveyor {
    from: Direction,
    to: Direction,
    has_item: Option<Entity>,
}
impl BasicNode for Conveyor {
    fn get_id() -> String {
        "conveyor".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                on_placed.in_set(GridFixed::OnPlaced),
                on_network_changed.in_set(GridFixed::OnPlaced),
                on_left_clicked.in_set(GridFixed::MainUpdate),
                on_update.in_set(GridFixed::MainUpdate),
                despawn_entity.in_set(GridFixed::OnPlaced),
            ),
        );
    }
    fn remove(commands: &mut EntityCommands) {
        commands
            .remove::<Inventory<Item>>()
            .remove::<Channel<Item>>()
            .insert(RemainForDespawnEntity);
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            Conveyor {
                from: Direction::NegX,
                to: Direction::NegX,
                has_item: None,
            },
            Inventory::<Item>::new(1)
                .configure_slot(SlotID(0), 
                |slot| {
                    slot.set_max_volume(1);
                }),
            Channel::<Item>::default()
                .add_port(
                    PortType::Input,
                    Port::default()
                        .set_target_grid(
                            TargetGrid::Specific(Direction::NegX.into_grid_pos())
                        )
                )
                .add_port(
                    PortType::Output,
                    Port::default()
                        .set_target_grid(
                            TargetGrid::Specific(Direction::NegX.into_grid_pos())
                        ).set_mode(
                            PortMode::with_cool_down(1)
                        )
                ).configure_time_cost(10),
            TextureBuff("textures/tile/conveyor_0_0.png".to_string()),
        ));
    }
}
impl Conveyor {
    fn has_output(&self) -> bool {
        self.from != self.to
    }
    fn input_connected(
        &self, 
        conveyor_q: &Query<(&mut Conveyor, &mut Channel<Item>, &GridPos, Entity)>,
        pos: GridPos, 
        grid: &Res<GridEntityMap>,
    ) -> bool {
        if let Some(e) = grid.get(&(pos + self.from.into_grid_pos()))
        && let Ok((conveyor, _, _, _)) = conveyor_q.get(e) {
            conveyor.to == self.from.inverse()
        } else {
            false
        }
    }
    fn output_connected(
        &self, 
        conveyor_q: &Query<(&mut Conveyor, &mut Channel<Item>, &GridPos, Entity)>,
        pos: GridPos, 
        grid: &Res<GridEntityMap>,
    ) -> bool {
        if self.has_output()
        && let Some(e) = grid.get(&(pos + self.to.into_grid_pos()))
        && let Ok((conveyor, _, _, _)) = conveyor_q.get(e) {
            conveyor.from == self.to.inverse()
        } else {
            false
        }
    }
}

#[derive(Component)]
struct RemainForDespawnEntity;

fn despawn_entity(
    mut commands: Commands,
    conveyor_q: Query<(&Conveyor, Entity), With<RemainForDespawnEntity>>
) {
    for (conveyor, entity) in conveyor_q {
        if let Some(entity) = conveyor.has_item {
            commands.entity(entity).despawn();
        }
        commands.entity(entity)
            .remove::<Conveyor>()
            .remove::<RemainForDespawnEntity>();
    }
}

#[derive(Component)]
struct NetworkChanged;

fn on_network_changed(
    mut commands: Commands,
    mut conveyor_q: Query<(&mut Conveyor, &mut Channel<Item>, &GridPos, Entity)>,
    network_changed_q: Query<Entity, With<NetworkChanged>>,
    grid: Res<GridEntityMap>,
) {
    for e in network_changed_q {
        commands.entity(e).remove::<NetworkChanged>();

        let mut new_from;
        let mut new_to;
        let mut from_changed = false;
        let mut to_changed = false;
        {
            let Ok((conveyor, _, pos, _)) = conveyor_q.get(e) else {continue;};
            new_from = conveyor.from; new_to = conveyor.to;
            if !conveyor.input_connected(&conveyor_q, *pos, &grid) {
                for dir in Direction::ALL {
                    let cur_pos = *pos + dir.into_grid_pos();
                    let Some(cur_e) = grid.get(&cur_pos) else {continue;};
                    let Ok((cur_c, _, _, _)) = conveyor_q.get(cur_e) else {continue;}; 

                    if !cur_c.has_output()
                    || cur_c.output_connected(&conveyor_q, cur_pos, &grid){
                        continue;
                    }
                    from_changed = true;
                    new_from = dir;
                    commands.entity(cur_e).insert(NetworkChanged);
                    break;
                    
                }
            } else {
                from_changed = true;
            }
            if !conveyor.output_connected(&conveyor_q, *pos, &grid) {
                for dir in Direction::ALL {
                    let cur_pos = *pos + dir.into_grid_pos();
                    let Some(cur_e) = grid.get(&cur_pos) else {continue;};
                    let Ok((cur_c, _, _, _)) = conveyor_q.get(cur_e) else {continue;}; 

                    if cur_c.input_connected(&conveyor_q, cur_pos, &grid)
                    || dir == new_from{
                        continue;
                    }
                    to_changed = true;
                    new_to = dir;
                    commands.entity(cur_e).insert(NetworkChanged);
                    break;
                }
            } else {
                to_changed = true;
            }
        }
        if !from_changed && to_changed{
            new_from = new_to.inverse();
        } else if !to_changed {
            new_to = new_from;
        }
        let Ok((mut conveyor, mut channel, _, e)) = conveyor_q.get_mut(e) else {continue;};
        conveyor.from = new_from; conveyor.to = new_to;
        
        commands.entity(e).insert(TextureBuff(
            format!(
                "textures/tile/conveyor_{}_{}.png",
                conveyor.from.get_id(),
                conveyor.to.get_id()
            )
            .to_string(),
        ));

        update_port(&mut conveyor, &mut channel);
    }
}

fn on_placed(
    mut commands: Commands,
    mut self_q: Query<(&mut Conveyor, &mut Channel<Item>, &GridPos)>,
    placed_q: Query<Entity, With<Placed>>,
    grid: Res<GridEntityMap>,
) {
    for e in placed_q {
        let mut new_from = Direction::NegX;
        let mut new_to = Direction::NegX;
        let mut from_changed = false;
        let mut to_changed = false;
        if let Ok((_, _, pos)) = self_q.get(e) {
            for dir in Direction::ALL {
                let Some(cur_e) = grid.get(&(*pos + dir.into_grid_pos())) else {
                    continue;
                };
                if let Ok((cur_c, _, _)) = self_q.get(cur_e)
                {
                    commands.entity(cur_e).insert(NetworkChanged);
                    if cur_c.from == dir.inverse() {
                        to_changed = true;
                        new_to = dir;
                    } else if cur_c.to == dir.inverse() {
                        from_changed = true;
                        new_from = dir;
                    }
                }
            }
        }
        if !from_changed && to_changed{
            new_from = new_to.inverse();
        } else if !to_changed {
            new_to = new_from;
        }
        if let Ok((mut c, mut channel, _)) = self_q.get_mut(e) {
            c.from = new_from;
            c.to = new_to;
            commands.entity(e).insert(TextureBuff(
                format!(
                    "textures/tile/conveyor_{}_{}.png",
                    c.from.get_id(),
                    c.to.get_id()
                )
                .to_string(),
            ));
            update_port(&mut c, &mut channel);
        }
    }
}

fn on_left_clicked(
    mut commands: Commands,
    conveyor_q: Query<
        (
            &mut Conveyor,
            &mut Inventory<Item>,
            &mut Channel<Item>,
            Entity,
        ),
        With<LeftClicked>,
    >,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut c, mut inv, mut channel, e) in conveyor_q {
        if keys.pressed(KeyCode::ControlLeft) {
            replace::<air::Air>(&mut commands, e);
        }

        if keys.pressed(KeyCode::Space)
            && let Some(slot) = inv.get_mut(SlotID(0))
        {
            if slot.val.is_none() {
                slot.val = Some(Item::Clay);
                slot.vol = 1;
            }
        }

        let mut new_dir = Direction::NegX;
        if keys.pressed(KeyCode::KeyS) {
            new_dir = Direction::NegY;
        } else if keys.pressed(KeyCode::KeyD) {
            new_dir = Direction::X;
        } else if keys.pressed(KeyCode::KeyW) {
            new_dir = Direction::Y;
        } else if !keys.pressed(KeyCode::KeyA) {
            continue;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            c.to = new_dir;
        } else {
            c.from = new_dir;
        }
        update_port(&mut c, &mut channel);
        commands.entity(e).insert(TextureBuff(
            format!(
                "textures/tile/conveyor_{}_{}.png",
                c.from.get_id(),
                c.to.get_id()
            )
            .to_string(),
        ));
    }
}

fn update_port(
    conveyor: &mut Conveyor,
    channel: &mut Channel<Item>,
) {
    channel.input.get_mut(0).and_then(|x| {
        x.grid = TargetGrid::Specific(conveyor.from.into_grid_pos());
        None::<Item>
    });
    channel.output.get_mut(0).and_then(|x| {
        x.grid = TargetGrid::Specific(conveyor.to.into_grid_pos());
        x.active = conveyor.from != conveyor.to;
        None::<Item>
    });
}

fn on_update(
    mut commands: Commands,
    c_q: Query<(&mut Conveyor, &GridPos, &Inventory<Item>)>,
) {
    for (mut con, pos, inv) in c_q {
        if con.has_item.is_some()
        != inv.get(SlotID(0)).and_then(|x| {
            if x.reserved == 0 {
                x.val
            } else {
                None::<Item>
            }
        }).is_some() {
            if let Some(e) = con.has_item {
                con.has_item = None;
                commands.entity(e).despawn();
            } else {
                con.has_item = Some(commands.spawn((
                    TextureBuff(format!("textures/item/{}.png", inv.get(SlotID(0)).unwrap().val.unwrap().get_id()).to_string()),
                    Transform::from_xyz(pos.to_world_pos().x, pos.to_world_pos().y, 2.),
                )).id());
            }
        }
    }
}
