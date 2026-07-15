//! # Path: src/nodes/clay_furnace.rs

use bevy::prelude::*;

use crate::{
    commons::Registerable, 
    grid::messages::*, 
    movables::item::{
        Item, 
        Type
    }, 
    nodes::commons::{
        Inventory, 
        InventorySize, 
        InventorySlot,
        InventorySlotID, 
        Spawnable,
        InputPort,
        Port,
    }
};

const SLOT_INPUT : InventorySlotID = InventorySlotID (0);
const SLOT_FUEL  : InventorySlotID = InventorySlotID (1);
const SLOT_OUTPUT: InventorySlotID = InventorySlotID (2);

#[derive(Component)]
pub struct ClayFurnace {
    timer: Timer
}
impl Registerable for ClayFurnace {
    fn register(app: &mut bevy::app::App) {
        app.add_systems(Update, (
            on_update,
            on_left_clicked,
        ));
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Spawnable for ClayFurnace {
    fn get_bundle() -> impl Bundle {
        (
            ClayFurnace {
                timer: Timer::from_seconds(10., TimerMode::Once)
            },
            Inventory::create_empty (
                InventorySize (3),
            ),
            InputPort::new(
                Port::Single(
                    InventorySlotID(0)
                ),
            ),
        )
    }
}

fn on_update(
    mut commands: Commands,
    q: Query<(&mut ClayFurnace, &mut Inventory, &mut InputPort, Entity)>,
    time: Res<Time>,
    asset: Res<AssetServer>,
) {
    for (mut furnace, mut inventory, mut input, e) in q {
        if inventory.check_item(SLOT_INPUT).is_some() {
            if furnace.timer.tick(time.delta()).just_finished() {
                inventory.take_1(SLOT_INPUT);
                furnace.timer.reset();
            }
        }
        if input.recieved {
            input.recieved = false;
            if let Some(e) = input.display_item.take() {
                commands.entity(e).despawn();
            }
        }
        image_refresh(&mut commands.entity(e), &inventory, &asset);
    }
}

fn image_refresh(
    commands: &mut EntityCommands,
    inventory: &Inventory,
    asset: &Res<AssetServer>,
) {
    commands.insert(
        Sprite::from_image(
            asset.load(
                if inventory.check_item(SLOT_INPUT).is_some() {
                    "textures/tile/clay_furnace_1.png"
                } else {
                    "textures/tile/clay_furnace_0.png"
                }
            )
        )
    );
}

fn on_left_clicked(
    mut commands: Commands,
    mut lc: MessageReader<LeftClicked>,
    mut q : Query<(&mut ClayFurnace, &mut Inventory, Entity)>,
    asset: Res<AssetServer>,
) {
    for m in lc.read() {
        let clicked_entity = m.0;
        if let Ok((mut furnace, mut inventory, e)) = q.get_mut(clicked_entity) {
            inventory.write_item(SLOT_INPUT, InventorySlot(
                Some(Item{
                    id: Type::Clay,
                    size: 1
            })));
            commands.entity(e).insert(
                Sprite::from_image(
                    asset.load("textures/tile/clay_furnace_1.png")
                )
            );
            furnace.timer.reset();
        }
    }
}

fn on_placed(
    mut command: Commands,
    mut reader: MessageReader<Placed>,
    q : Query<(), With<ClayFurnace>>,
    asset: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if q.get(clicked_entity).is_ok() {
            command.entity(clicked_entity).insert(
                Sprite::from_image(
                    asset.load("textures/tile/clay_furnace_0.png")
                )
            );
        }
    }
}
