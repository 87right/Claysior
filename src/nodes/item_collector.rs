//! # Path: src/nodes/item_collector.rs

use bevy::prelude::*;
use crate::commons::*;
use crate::constants::*;
use crate::movables::item::Item;
use crate::nodes::commons::*;
use crate::grid::{
    messages::*,
    components::*,
};

#[derive(Component)]
pub struct ItemCollector;
impl Registerable for ItemCollector {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_update,
            on_left_clicked,
        ));
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Spawnable for ItemCollector {
    fn get_bundle() -> impl Bundle {
        (
            ItemCollector,
            Inventory::create_empty(InventorySize(1)),
        )
    }
}

fn on_update(
    mut command: Commands,
    mut writer: MessageWriter<ItemSendReq>,
    q: Query<(&GridPos, &mut Inventory, Entity), With<ItemCollector>>,
    items: Query<(&Item, &Transform, Entity)>,
    world_grid: Res<WorldGrid>,
) {
    for (grid_pos, mut inventory, e) in q {
        for (item, transform, e) in items {
            let pos = grid_pos.to_bottom_left_vec2();
            let diff = Vec2 {
                x: transform.translation.x - pos.x,
                y: transform.translation.y - pos.y,
            };
            if diff.length() < CELL_SIZE * 1.5
            && inventory.insert(*item) {
                command.entity(e).despawn();
            }
        }
        if inventory.check_item(InventorySlotID(0)).is_some() {
            let targets = [
                IVec2::NEG_X,
                IVec2::NEG_Y,
                IVec2::X,
                IVec2::Y,
            ];
            for target in targets {
                if let Some(e_to) = world_grid.0.get(&(target + grid_pos.0)) {
                    writer.write(ItemSendReq {
                        from  : e,
                        to    : *e_to,
                        index : InventorySlotID(0),
                        e_item: None
                    });
                }
            }
        }
    }
}

fn on_left_clicked(
    mut lc: MessageReader<LeftClicked>,
    mut q : Query<&mut Inventory, With<ItemCollector>>,
) {
    for m in lc.read() {
        let clicked_entity = m.0;
        if let Ok(mut inventory) = q.get_mut(clicked_entity) {
            inventory.take_item(InventorySlotID(0));
        }
    }
}

fn on_placed(
    mut commands: Commands,
    mut reader: MessageReader<Placed>,
    mut q : Query<(), With<ItemCollector>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if q.get_mut(clicked_entity).is_ok() {
            commands.entity(clicked_entity).insert(Sprite::from_image(
                asset_server.load("textures/tile/item_collector.png")
            ));
        }
    }
}
