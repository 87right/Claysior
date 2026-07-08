//! # Path: src/nodes/conveyor.rs

use bevy::prelude::*;
use crate::commons::*;
use crate::grid::messages::*;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Conveyor {
    pub inventory: Inventory
}
impl Registerable for Conveyor {
    fn register(app: &mut App) {
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Spawnable for Conveyor {
    fn get_bundle() -> impl Bundle {
        (
            Conveyor {
                inventory: Inventory::create_empty(InventorySize(1)),
            },
        )
    }
}


fn on_placed(
    mut reader: MessageReader<Placed>,
    mut q : Query<&mut Sprite, With<Conveyor>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if let Ok(mut sprite) = q.get_mut(clicked_entity) {
            *sprite = Sprite::from_image(
                asset_server.load("textures/tile/conveyor.png")
            );
        }
    }
}
