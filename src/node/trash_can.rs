use crate::node::prelude::*;

#[derive(Component)]
pub struct TrashCan;
impl BasicNode for TrashCan {
    fn get_id() -> String {
        "trash_can".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(FixedUpdate, on_clicked.in_set(GridFixed::MainUpdate));
    }
    fn remove(commands: &mut EntityCommands) {
        commands
            .remove::<TrashCan>()
            .remove::<Inventory<Item>>()
            .remove::<Channel<Item>>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            TrashCan,
            Inventory::<Item>::new(1),
            Channel::<Item>::default().add_port(
                PortType::Input, 
                Port::default()
            ),
            TextureBuff("textures/tile/trash_can.png".to_string()),
        ));
    }
}

fn on_update(
    cg_q: Query<&mut Inventory<Item>, With<TrashCan>>,
) {
    for mut inventory in cg_q {
        inventory.content.get_mut(0).and_then(|slot| {
            slot.val = None;
            slot.vol = 0;
            None::<&mut MaterialSlot<Item>>
        });
    }
}

fn on_clicked(
    mut commands: Commands,
    tc_q: Query<Entity, (With<LeftClicked>, With<TrashCan>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for entity in tc_q {
        if keys.pressed(KeyCode::ControlLeft) {
            replace::<crate::node::air::Air>(&mut commands, entity);
            continue;
        }
    }
}
