use bevy::prelude::*;

use crate::input::{resource::LayeredButtonInput, system_set::InputLayer};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Test(false));
        app.add_systems(Update, (
            test,
            test_input
        ).in_set(InputLayer::UI));
    }
}

fn test(
    mut keys: ResMut<LayeredButtonInput<KeyCode>>,
    mut test: ResMut<Test>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        test.0 = !test.0;
    }
    keys.consume();
}

fn test_input(
    mut mouse: ResMut<LayeredButtonInput<MouseButton>>,
    test: Res<Test>,
) {
    if test.0 {
        mouse.consume();
    }
}

#[derive(Resource)]
struct Test(bool);
