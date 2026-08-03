use crate::node::prelude::*;

#[derive(Component)]
pub struct ClayDistributor {
    has_item: Option<Entity>
}
impl BasicNode for ClayDistributor {
    fn get_id() -> String {
        "clay_distributor".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(FixedUpdate, (
            on_update, 
            on_clicked
        ).chain().in_set(GridFixed::MainUpdate));
    }
    fn remove(commands: &mut EntityCommands) {
        commands
            .remove::<ClayDistributor>()
            .remove::<Inventory<Item>>()
            .remove::<Channel<Item>>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            ClayDistributor {
                has_item: None
            },
            Inventory::<Item>::new(1).configure_slot(SlotID(0), |slot| {
                slot.set_max_volume(1);
            }),
            Channel::<Item>::default().add_port(
                PortType::Input, 
                Port::default().set_mode(
                    PortMode::with_cool_down(1)
                )
            ).add_port(
                PortType::Output,
                Port::default().set_target_grid(
                    TargetGrid::Specific(Direction::NegX.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(1)
                )
            ).add_port(
                PortType::Output,
                Port::default().set_target_grid(
                    TargetGrid::Specific(Direction::NegY.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(1)
                )
            ).add_port(
                PortType::Output,
                Port::default().set_target_grid(
                    TargetGrid::Specific(Direction::X.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(1)
                )
            ).add_port(
                PortType::Output,
                Port::default().set_target_grid(
                    TargetGrid::Specific(Direction::Y.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(1)
                )
            ).configure_time_cost(10),
            TextureBuff("textures/tile/clay_distributor.png".to_string())
        ));

    }
}

fn on_clicked(
    mut commands: Commands,
    tc_q: Query<(Entity, &mut ClayDistributor), With<LeftClicked>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, mut dis) in tc_q {
        if keys.pressed(KeyCode::ControlLeft) {
            if let Some(e) = dis.has_item {
                commands.entity(e).despawn();
            }
            dis.has_item = None;
            replace::<crate::node::air::Air>(&mut commands, entity);
            continue;
        }
    }
}
 

fn on_update(
    mut commands: Commands,
    c_q: Query<(&mut ClayDistributor, &GridPos, &Inventory<Item>)>,
) {
    for (mut dis, pos, inv) in c_q {
        if dis.has_item.is_some()
        != inv.get(SlotID(0)).and_then(|x| {
            if x.reserved == 0 {
                x.val
            } else {
                None::<Item>
            }
        }).is_some() {
            if let Some(e) = dis.has_item {
                dis.has_item = None;
                commands.entity(e).despawn();
            } else {
                dis.has_item = Some(commands.spawn((
                    TextureBuff(format!("textures/item/{}.png", inv.get(SlotID(0)).unwrap().val.unwrap().get_id()).to_string()),
                    Transform::from_xyz(pos.to_world_pos().x, pos.to_world_pos().y, 2.),
                )).id());
            }
        }
    }
}
