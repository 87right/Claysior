use bevy::prelude::*;

use crate::{
    grid::{
        common::*, 
        component::*,
        system_set::*,
        util::*,
        resource::*,
    }, 
    node::*,
};

#[derive(Component)]
pub struct Conveyor{
    direction: u8,
}
impl BasicNode for Conveyor {
    fn get_id() -> String {
        "conveyor".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(FixedUpdate, on_left_clicked.in_set(GridFixed::MainUpdate));
    }
    fn remove(commands: &mut EntityCommands) {
        commands.remove::<Conveyor>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            Conveyor{
                direction: 0
            },
            TextureBuff("textures/tile/conveyor_0.png".to_string()),
        ));
    }
}

fn on_left_clicked(
    mut commands: Commands,
    conveyor_q: Query<(&mut Conveyor, Entity), With<LeftClicked>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut c, e) in conveyor_q {
        if keys.pressed(KeyCode::ControlLeft) {
            replace::<air::Air>(&mut commands, e);
        } else if keys.pressed(KeyCode::ShiftLeft) {
            c.direction &= 0b0011;
            c.direction |= 
            if keys.pressed(KeyCode::KeyW) {
                0b1100
            } else if keys.pressed(KeyCode::KeyA) {
                0b0000
            } else if keys.pressed(KeyCode::KeyS) {
                0b0100
            } else if keys.pressed(KeyCode::KeyD) {
                0b1000
            } else {
                0
            };
        } else {
            c.direction &= 0b1100;
            c.direction |= 
            if keys.pressed(KeyCode::KeyW) {
                0b11
            } else if keys.pressed(KeyCode::KeyA) {
                0b00
            } else if keys.pressed(KeyCode::KeyS) {
                0b01
            } else if keys.pressed(KeyCode::KeyD) {
                0b10
            } else {
                0
            };
        }
        commands.entity(e).insert(TextureBuff(format!("textures/tile/conveyor_{}.png", c.direction).to_string()));
    }
}
