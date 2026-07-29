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
    },
    item::{
        component::*,
    }
};

#[derive(Component, Debug)]
pub struct ClayFurnace {
    timer: Timer
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
        commands.remove::<ClayFurnace>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            ClayFurnace {
                timer: Timer::from_seconds(10., TimerMode::Once),
            },
            TextureBuff("textures/tile/clay_furnace_0.png".to_string()),
            Inventory::<Item>::new(2),
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
    furnace_q: Query<(&mut ClayFurnace, &mut Inventory<Item>, Entity)>,
    recipe: Res<FurnaceRecipe>,
    time: Res<Time>,
) {
    for (mut furnace, mut inventory, _) in furnace_q {
        let item;
        if let Some(stick) = inventory.get(SlotID(0)).and_then(|x| x.val)
        && let Some(out) = recipe.0.get(&RecipeInputBuff(stick)) 
        && furnace.timer.tick(time.delta()).just_finished() {
            furnace.timer.reset();
            item = out.0;
        } else {
            continue;
        }
        let mut can_insert = false;
        if let Some(slot) = inventory.get_mut(SlotID(1)) 
        && slot.insert(&mut MaterialSlot::<Item>{val: Some(item), vol: 1}){
            can_insert = true;
        }
        if can_insert 
        && let Some(slot) = inventory.get_mut(SlotID(0)) {
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
