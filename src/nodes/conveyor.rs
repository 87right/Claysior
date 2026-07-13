//! # Path: src/nodes/conveyor.rs

use bevy::prelude::*;
use crate::commons::*;
use crate::constants::CELL_SIZE;
use crate::grid::components::{GridPos, WorldGrid};
use crate::grid::messages::*;
use crate::movables::item::DisplayItem;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Conveyor {
    timer: Timer,
    display_item: Option<Entity>,
}
impl Registerable for Conveyor {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_item_sent,
            on_update,
        ));
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Spawnable for Conveyor {
    fn get_bundle() -> impl Bundle {
        (
            Conveyor {
                timer: Timer::from_seconds(2., TimerMode::Once),
                display_item: None
            },
            Inventory::create_empty(InventorySize(1))
        )
    }
}

fn on_item_sent(
    mut command: Commands,
    mut reader: MessageReader<ItemSendReq>,
    mut inventory_q: Query<&mut Inventory>,
    mut conveyor_q: Query<(&GridPos, &mut Conveyor)>,
    asset_server: Res<AssetServer>,
    world: Res<WorldGrid>,
) {
    for m in reader.read() {
        let can_receive = if let Ok(inventory) = inventory_q.get(m.to) 
        && let Ok((grid_pos, _)) = conveyor_q.get(m.to) 
        && let Some(target_e) = world.0.get(&(grid_pos.0 + IVec2::X)){
            *target_e == m.from && inventory.check_item(InventorySlotID(0)).is_none()
        } else {
            false
        };
        if !can_receive {continue;}

        let (grid_pos, mut conveyor) = conveyor_q.get_mut(m.to).unwrap();
        conveyor.timer.reset();

        if let Ok([mut to_inv, mut from_inv]) = inventory_q.get_many_mut([m.to, m.from]) {
            let new_item = from_inv.take_item(&m.index);
            to_inv.write_item(InventorySlotID(0), InventorySlot(new_item.clone()));
            if let Some(item) = new_item {
                let item_pos = grid_pos.to_bottom_left_vec2();
                if let Some(entity_item) = m.e_item {
                    conveyor.display_item = Some(entity_item);
                } else {
                    conveyor.display_item = Some(command.spawn((
                        DisplayItem,
                        item.id.get_sprite(&asset_server),
                        Transform::from_xyz(
                            item_pos.x + CELL_SIZE as f32 / 2.,
                            item_pos.y,
                            1.
                        )
                    )).id());
                }
            }
        }
    }
}

fn on_update(
    mut writer: MessageWriter<ItemSendReq>,
    mut display_item_q: Query<&mut Transform, With<DisplayItem>>,
    conveyor_q: Query<(&mut Inventory, &mut Conveyor, &GridPos, Entity)>,
    world: Res<WorldGrid>,
    time: Res<Time>,
) {
    for (inventory, mut conveyor, grid_pos, e) in conveyor_q {
        if inventory.check_item(InventorySlotID(0)).is_some() {
            conveyor.timer.tick(time.delta());
            if let Some(item_entity) = conveyor.display_item
            && let Ok(mut transform) = display_item_q.get_mut(item_entity) {
                transform.translation.x = grid_pos.to_bottom_left_vec2().x as f32 + CELL_SIZE * conveyor.timer.remaining().as_secs_f32() / 2.;
            }
            if conveyor.timer.is_finished()
            && let Some(to) = world.0.get(&(grid_pos.0 + IVec2::NEG_X)) {
                writer.write(ItemSendReq {
                    from: e,
                    to: *to,
                    index: InventorySlotID(0),
                    e_item: conveyor.display_item,
                });
            }
        } 
    }
}

fn on_placed(
    mut commands: Commands,
    mut reader: MessageReader<Placed>,
    mut q : Query<(), With<Conveyor>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if q.get_mut(clicked_entity).is_ok() {
            commands.entity(clicked_entity).insert(Sprite::from_image(
                asset_server.load("textures/tile/conveyor.png")
            ));
        }
    }
}
