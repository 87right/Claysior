use crate::node::prelude::*;

#[derive(Component)]
pub struct ClayUnloader {
    direction: Direction,
    has_item: Option<Entity>,
}
impl BasicNode for ClayUnloader {
    fn get_id() -> String {
        "clay_unloader".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(
            FixedUpdate, 
            (
                on_update,
                on_clicked,
            ).chain().in_set(GridFixed::MainUpdate),
        );
    }
    fn remove(commands: &mut EntityCommands) {
        commands
            .remove::<ClayUnloader>()
            .remove::<Channel<Item>>()
            .remove::<Inventory<Item>>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            ClayUnloader {
                direction: Direction::NegX,
                has_item: None
            },
            Inventory::<Item>::new(1).configure_slot(SlotID(0), |slot| {
                slot.set_max_volume(1)
            }),
            Channel::<Item>::default().add_port(
                PortType::Pull, 
                Port::default().set_mode(
                    PortMode::with_cool_down(1)
                ).set_target_grid(
                    TargetGrid::Specific(Direction::NegX.into_grid_pos())
                ).set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                )
            ).add_port(
                PortType::Output, 
                Port::default().set_mode(
                    PortMode::with_cool_down(1),
                ).set_target_grid(
                    TargetGrid::Specific(Direction::X.into_grid_pos())
                ).set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                )
            ).configure_time_cost(10),
            TextureBuff("textures/tile/unloader_0.png".to_string())
        ));
    }
}


fn on_update(
    mut commands: Commands,
    cu_q: Query<(&mut ClayUnloader, &GridPos, &Inventory<Item>)>,
) {
    for (mut cu, pos, inv) in cu_q {
        if cu.has_item.is_some()
        != inv.get(SlotID(0)).and_then(|x| {
            if x.reserved == 0 {
                x.val
            } else {
                None::<Item>
            }
        }).is_some() {
            if let Some(e) = cu.has_item {
                cu.has_item = None;
                commands.entity(e).despawn();
            } else {
                cu.has_item = Some(commands.spawn((
                    TextureBuff(format!("textures/item/{}.png", inv.get(SlotID(0)).unwrap().val.unwrap().get_id()).to_string()),
                    Transform::from_xyz(pos.to_world_pos().x, pos.to_world_pos().y, 2.),
                )).id());
            }
        }
    }
}

fn on_clicked(
    mut commands: Commands,
    furnace_q: Query<(&mut ClayUnloader, &mut Channel<Item>, Entity), With<LeftClicked>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut unloader, mut channel, entity) in furnace_q {
        if keys.pressed(KeyCode::ControlLeft) {
            if let Some(e) = unloader.has_item {
                commands.entity(e).despawn();
            }
            unloader.has_item = None;
            replace::<crate::node::air::Air>(&mut commands, entity);
            continue;
        }
        
        if keys.pressed(KeyCode::KeyW) {
            unloader.direction = Direction::Y;
        } else if keys.pressed(KeyCode::KeyA) {
            unloader.direction = Direction::NegX;
        } else if keys.pressed(KeyCode::KeyS) {
            unloader.direction = Direction::NegY;
        } else if keys.pressed(KeyCode::KeyD) {
            unloader.direction = Direction::X;
        } else {
            continue;
        }
        channel
            .get_port(PortType::Output, 0)
            .and_then(|port| {
                port.grid = TargetGrid::Specific(unloader.direction.inverse().into_grid_pos());
                None::<Port<Item>>
            });
        channel
            .get_port(PortType::Pull, 0)
            .and_then(|port| {
                port.grid = TargetGrid::Specific(unloader.direction.into_grid_pos());
                None::<Port<Item>>
            });
        commands.entity(entity).insert(
            TextureBuff(format!("textures/tile/unloader_{}.png", unloader.direction.get_id()).to_string())
        );
    }
}
