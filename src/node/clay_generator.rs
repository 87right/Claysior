use crate::node::prelude::*;

#[derive(Component)]
pub struct ClayGenerator;
impl BasicNode for ClayGenerator {
    fn get_id() -> String {
        "clay_generator".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(FixedUpdate, (
            on_clicked,
            on_update,
        ).in_set(GridFixed::MainUpdate));
    }
    fn remove(commands: &mut EntityCommands) {
        commands
            .remove::<ClayGenerator>()
            .remove::<Inventory<Item>>()
            .remove::<Channel<Item>>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            ClayGenerator,
            Inventory::<Item>::new(1),
            Channel::<Item>::default().add_port(
                PortType::Open, 
                Port::default().set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                ).set_target_grid(
                    TargetGrid::Specific(Direction::NegX.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(20)
                )
            ).add_port(
                PortType::Open, 
                Port::default().set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                ).set_target_grid(
                    TargetGrid::Specific(Direction::X.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(20)
                )
            ).add_port(
                PortType::Open, 
                Port::default().set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                ).set_target_grid(
                    TargetGrid::Specific(Direction::NegY.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(20)
                )
            ).add_port(
                PortType::Open, 
                Port::default().set_target_slot(
                    TargetSlot::Specific(SlotID(0))
                ).set_target_grid(
                    TargetGrid::Specific(Direction::Y.into_grid_pos())
                ).set_mode(
                    PortMode::with_cool_down(20)
                )
            ),
            TextureBuff("textures/tile/clay_generator.png".to_string())
        ));
    }
}

fn on_update(
    cg_q: Query<&mut Inventory<Item>, With<ClayGenerator>>,
) {
    for mut inventory in cg_q {
        inventory.insert(
            SlotID(0), 
            &mut MaterialSlot::<Item>::new()
                .configure_value(Some(Item::Clay))
                .configure_volume(1)
            );
    }
}


fn on_clicked(
    mut commands: Commands,
    cg_q: Query<Entity, (With<LeftClicked>, With<ClayGenerator>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for entity in cg_q {
        if keys.pressed(KeyCode::ControlLeft) {
            replace::<crate::node::air::Air>(&mut commands, entity);
            continue;
        }
    }
}
