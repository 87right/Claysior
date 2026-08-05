use bevy::{sprite::Anchor, window::WindowResized};

use crate::prelude::*;

const PADDING: f32 = 0.999;

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debug(false, Timer::from_seconds(1., TimerMode::Repeating)));
        app.add_systems(Update, (
            update_debug,
            switch_debug_mode,
            on_window_resized,
        ));
    }
}

#[derive(Resource)]
struct Debug(bool, Timer);

#[derive(Component)]
struct FPSViewer;

fn update_debug(
    mut res: ResMut<Debug>,
    debug: Query<&mut Text2d, With<FPSViewer>>,
    time: Res<Time>,
) {
    if res.0 && res.1.tick(time.delta()).just_finished() {
        for mut text in debug {
            text.0 = format!("{:.1}FPS", 1. / time.delta().as_secs_f32());
        }
    }
}

fn switch_debug_mode(
    mut commands: Commands,
    mut debug: ResMut<Debug>,
    keys: Res<LayeredButtonInput<KeyCode>>,
    q: Query<Entity, With<FPSViewer>>,
    time: Res<Time>,
    window: Single<&Window>
) {
    if keys.just_released(KeyCode::F3) {
        debug.0 = !debug.0;
        if debug.0 {
            commands.spawn((
                Text2d::new(format!("{:.1}FPS", 1. / time.delta().as_secs_f32())),
                FPSViewer,
                Transform::from_xyz((-window.width() / 2.) * PADDING, (window.height() / 2.) * PADDING, 1.),
                GameLayer::GUI,
                Anchor::TOP_LEFT,
                GUITransform {
                    x: 0.,
                    y: 1024.
                }
            ));
        } else {
            for e in q {
                commands.entity(e).despawn();   
            }
        }
    }
}

#[derive(Component)]
pub struct GUITransform {
    x: f32,
    y: f32,
}

fn on_window_resized(
    mut resize_reader: MessageReader<WindowResized>,
    mut gui_q: Query<(&mut Transform, &GUITransform)>,
) {
    for e in resize_reader.read() {
        let x_scale = e.width / 1024.;
        let y_scale = e.height / 1024.;
        for (mut transform, gui) in &mut gui_q {
            transform.translation.x = (gui.x - 512.) * x_scale * PADDING;
            transform.translation.y = (gui.y - 512.) * y_scale * PADDING;
        }
    }
}
