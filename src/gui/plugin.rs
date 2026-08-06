use bevy::{sprite::Anchor, window::WindowResized};

use crate::prelude::*;

const PADDING: f32 = 0.999;

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debug(false, Timer::from_seconds(1., TimerMode::Repeating)));
        app.add_systems(Startup, on_grid_reload);
        app.add_systems(Update, (
            update_debug,
            switch_debug_mode,
            on_window_resized,
            draw_grid_line,
        ));
    }
}

#[derive(Resource)]
struct Debug(bool, Timer);

#[derive(Component)]
struct FPSViewer;

fn update_debug(
    mut res: ResMut<Debug>,
    fps_viewer: Query<&mut Text2d, With<FPSViewer>>,
    time: Res<Time>,
) {
    if res.0 && res.1.tick(time.delta()).just_finished() {
        for mut text in fps_viewer {
            text.0 = format!("{:.1}FPS", 1. / time.delta().as_secs_f32());
        }
    }
}

fn draw_grid_line(
    mut gizmos: Gizmos,
    setting: Res<WorldGeneratingSetting>,
    debug: ResMut<Debug>,
) {
    if debug.0 {
        gizmos.cross_2d(Vec2::new(
            GameSetting::CELL_SIZE * (setting.width + 1) as f32 / 2.,
            GameSetting::CELL_SIZE * (setting.width + 1) as f32 / 2., 
        ), 12., bevy::color::palettes::css::FUCHSIA);
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

fn on_grid_reload(
    mut commands: Commands,
    background: Query<&mut Sprite, With<BackGround>>,
    setting: Res<WorldGeneratingSetting>,
    asset_server: Res<AssetServer>,
) {
    let mut exist = false;
    for mut sprite in background {
        exist = true;
        *sprite = Sprite::from_image(
            asset_server.load(format!("textures/background/{}.png", setting.background).to_string())
        );
    }
    if !exist {
        commands.spawn((
            Sprite::from_image(
                asset_server.load(format!("textures/background/{}.png", setting.background).to_string())
            ),
            BackGround,
            Anchor::BOTTOM_LEFT,
            Transform::from_xyz(
                GameSetting::CELL_SIZE / 2.,
                GameSetting::CELL_SIZE / 2.,
                0.,
            ),
            GameLayer::MAIN,
        ));
    }
}
