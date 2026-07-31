use bevy::prelude::*;

use crate::{consumable::{common::PortType, component::{Channel, Inventory, Port, PortMode, SlotID, TargetGrid, TargetSlot}}, grid::{common::*, component::{LeftClicked, TextureBuff}, system_set::GridFixed, util::replace}, item::component::Item};

#[derive(Component)]
pub struct ClayUnloader {
    direction: Direction
}
impl BasicNode for ClayUnloader {
    fn get_id() -> String {
        "clay_unloader".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(
            FixedUpdate, 
            on_clicked.in_set(GridFixed::MainUpdate),
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
            },
            Inventory::<Item>::new(1).configure_slot(SlotID(0), |slot| {
                slot.set_max_volume(1)
            }),
            Channel::<Item>::default().add_port(
                PortType::Pull, 
                Port::default().set_mode(
                    PortMode::with_cool_down(10)
                ).set_target_grid(
                    TargetGrid::Specific(Direction::NegX.into_grid_pos())
                ).set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                )
            ).add_port(
                PortType::Output, 
                Port::default().set_mode(
                    PortMode::with_cool_down(10),
                ).set_target_grid(
                    TargetGrid::Specific(Direction::X.into_grid_pos())
                ).set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                )
            ),
            TextureBuff("textures/tile/unloader_0.png".to_string())
        ));
    }
}


fn on_clicked(
    mut commands: Commands,
    furnace_q: Query<(&mut ClayUnloader, &mut Channel<Item>, Entity), With<LeftClicked>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut unloader, mut channel, entity) in furnace_q {
        if keys.pressed(KeyCode::ControlLeft) {
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
