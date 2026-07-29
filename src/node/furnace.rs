use bevy::prelude::*;

use crate::{
    grid::{
        common::*,
        component::*,
        util::*,
        system_set::*,
    },
    consumable::{
        component::*,
        common::*,
    },
    item::{
        component::*,
    }
};

const INPUT: SlotID = SlotID(0);
const OUTPUT: SlotID = SlotID(1);

#[derive(Component, Debug)]
pub struct ClayFurnace {
    timer: Timer,
    running: bool,
}
impl BasicNode for ClayFurnace {
    fn get_id() -> String {
        "clay_furnace".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(Startup, register_recipe);
        app.add_systems(FixedUpdate, (
            on_clicked.in_set(GridFixed::MainUpdate),
            check_recipe.in_set(GridFixed::MainUpdate),
        ));
        app.insert_resource(FurnaceRecipe::default());
    }
    fn remove(commands: &mut EntityCommands) {
        commands
            .remove::<ClayFurnace>()
            .remove::<Inventory<Item>>()
            .remove::<Channel<Item>>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            ClayFurnace {
                timer: Timer::from_seconds(10., TimerMode::Once),
                running: false,
            },
            TextureBuff("textures/tile/clay_furnace_0.png".to_string()),
            Inventory::<Item>::new(2),
            Channel::<Item>::default().add_port(
                PortType::Input, 
                Port::default().set_target_grid(
                    TargetGrid::Any
                ).set_target_slot(
                    TargetSlot::Specific(INPUT)
                )
            )
        ));
    }
}

#[derive(Resource, Default)]
struct FurnaceRecipe(std::collections::HashMap<RecipeInputBuff, RecipeOutput>);

#[derive(Hash, PartialEq, Eq)]
struct RecipeInputBuff(Item);
struct RecipeOutput(Item, usize);

fn register_recipe(
    mut recipe: ResMut<FurnaceRecipe>
) {
    let input = RecipeInputBuff(Item::Clay);
    let output = RecipeOutput(Item::Clay, 1);
    recipe.0.insert(input, output);
}

fn check_recipe(
    mut commands: Commands,
    furnace_q: Query<(&mut ClayFurnace, &mut Inventory<Item>, Entity)>,
    recipe: Res<FurnaceRecipe>,
    time: Res<Time>,
) {
    for (mut furnace, mut inventory, e) in furnace_q {
        let item;
        if let Some(stick) = inventory.get(SlotID(0)).and_then(|x| x.val)
        && let Some(out) = recipe.0.get(&RecipeInputBuff(stick)) {
            if !furnace.running {
                furnace.running = true;
                commands.entity(e).insert(TextureBuff("textures/tile/clay_furnace_1.png".to_string()));
            }
            if furnace.timer.tick(time.delta()).just_finished() {
                furnace.timer.reset();
                item = out.0;
            } else {
                continue;
            }
        } else {
            if furnace.running {
                furnace.running = false;
                commands.entity(e).insert(TextureBuff("textures/tile/clay_furnace_0.png".to_string()));
            }
            continue;
        }
        let mut can_insert = false;
        if let Some(slot) = inventory.get_mut(OUTPUT) 
        && slot.insert(&mut MaterialSlot::<Item>{val: Some(item), vol: 1}){
            can_insert = true;
        }
        if can_insert 
        && let Some(slot) = inventory.get_mut(INPUT) {
            slot.vol -= 1;
            if slot.vol == 0 {
                slot.val = None;
            }
        }
    }
}

fn on_clicked(
    mut commands: Commands,
    furnace_q: Query<(&mut Inventory<Item>, &ClayFurnace, Entity), With<LeftClicked>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut inventory, furnace, entity) in furnace_q {
        if keys.pressed(KeyCode::ControlLeft) {
            replace::<crate::node::air::Air>(&mut commands, entity);
            continue;
        }
        if keys.pressed(KeyCode::Space) {
            inventory.insert(SlotID(0), &mut MaterialSlot { val: Some(Item::Clay), vol: 1 });
            println!("Inventory に Clay を insert しました。");
            continue;
        }
        println!("{furnace:#?}\n{inventory:#?}");
    }
}
