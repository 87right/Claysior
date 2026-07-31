use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::input::{resource::LayeredButtonInput, system_set::InputLayer};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Test(None));
        app.add_systems(Update, (
            test,
            test_input
        ).in_set(InputLayer::UI));
    }
}

fn test(
    mut commands: Commands,
    mut keys: ResMut<LayeredButtonInput<KeyCode>>,
    mut test: ResMut<Test>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if let (camera, transform) = camera.into_inner()
    && keys.just_pressed(KeyCode::KeyE) {
        if let Some(entity) = test.0 {
            commands.entity(entity).despawn();
            test.0 = None;
        } else if let Ok(spawn_pos) = camera.viewport_to_world_2d(transform, vec2(240., 45.)) {
            test.0 = Some(
                commands.spawn((
                    Text2d::new("Freezed"),
                    TextColor(YELLOW.into()),
                    Transform::from_xyz(spawn_pos.x, spawn_pos.y, 5.),
                    TextFont::from_font_size(30.)
                )).id()
            );
        }
    }
    keys.consume();
}

fn test_input(
    mut mouse: ResMut<LayeredButtonInput<MouseButton>>,
    test: Res<Test>,
) {
    if test.0.is_some() {
        mouse.consume();
    }
}

#[derive(Resource)]
struct Test(Option<Entity>);
