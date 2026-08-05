use crate::prelude::*;

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debug(false, Timer::from_seconds(1., TimerMode::Repeating)));
        app.add_systems(Update, (
            update_debug,
            switch_debug_mode,
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
) {
    if keys.just_released(KeyCode::F3) {
        debug.0 = !debug.0;
        if debug.0 {
            commands.spawn((
                Text2d::new(format!("{:.1}FPS", 1. / time.delta().as_secs_f32())),
                FPSViewer,
                Transform::from_xyz(0., 0., 1.),
                GameLayer::GUI,
            ));
        } else {
            for e in q {
                commands.entity(e).despawn();   
            }
        }
    }
}
