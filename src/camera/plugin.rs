use bevy::input::mouse::MouseWheel;

use crate::prelude::*;

use crate::camera::component::*;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraDragData::default());
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (
            mouse_input,
            key_input,
            camera_zoom_system,
        ).in_set(InputLayer::Camera));
    }
}

fn spawn_camera(mut commands: Commands, setting: Res<WorldGeneratingSetting>) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(
            GameSetting::CELL_SIZE * setting.width  as f32 / 2.,
            GameSetting::CELL_SIZE * setting.height as f32 / 2., 
            0.),
        MainCamera,
        GameLayer::MAIN,
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0., 0., 5.),
        GUICamera,
        GameLayer::GUI,
    ));
}

fn mouse_input(
    mut camera_drag_data: ResMut<CameraDragData>,
    camera_query: Single<(&mut Transform, &Projection), (With<Camera>, With<MainCamera>)>,
    buttons: Res<LayeredButtonInput<MouseButton>>,
    window: Single<&Window>,
) {
    let (mut transform, projection) = camera_query.into_inner();
    if let Some(position) = window.cursor_position()
        && let Projection::Orthographic(ref orthographic) = *projection
    {
        if buttons.just_pressed(MouseButton::Left) {
            camera_drag_data.last_cursor_pos = position;
            camera_drag_data.last_camera_pos = transform.translation;
        }

        if buttons.pressed(MouseButton::Left) {
            transform.translation = camera_drag_data.last_camera_pos
                + (camera_drag_data.last_cursor_pos - position).extend(0.)
                    * Vec3 {
                        x: 1.,
                        y: -1.,
                        z: 1.,
                    }
                    * orthographic.scale;
            camera_drag_data.last_cursor_pos = position;
            camera_drag_data.last_camera_pos = transform.translation;
        }
    }
}

fn key_input(
    camera_q: Single<(&mut Transform, &Projection), (With<Camera>, With<MainCamera>)>,
    keys: Res<LayeredButtonInput<KeyCode>>,
) {
    let (mut transform, projection) = camera_q.into_inner();
    const SPD: f32 = 2.;
    let mut y_vel: f32 = 0.;
    let mut x_vel: f32 = 0.;
    if keys.pressed(KeyCode::KeyW) {y_vel += 1.;}
    if keys.pressed(KeyCode::KeyS) {y_vel -= 1.;}
    if keys.pressed(KeyCode::KeyA) {x_vel -= 1.;}
    if keys.pressed(KeyCode::KeyD) {x_vel += 1.;}
    if let Projection::Orthographic(ref orthographic) = *projection {
        transform.translation += Vec3 {
            x: x_vel, 
            y: y_vel, 
            z: 0.
        } * SPD * orthographic.scale;
    }
}

fn camera_zoom_system(
    mut msr_scroll: MessageReader<MouseWheel>,
    mouse: Res<LayeredButtonInput<MouseButton>>,
    projection_query: Single<&mut Projection, (With<Camera>, With<MainCamera>)>,
) {
    if mouse.is_consumed() {return;}
    let mut projection = projection_query.into_inner();
    if let Projection::Orthographic(ref mut orthographic) = *projection {
        for ms in msr_scroll.read() {
            const ZOOM_SPD: f32 = 0.1;
            orthographic.scale = (orthographic.scale - ms.y * ZOOM_SPD).clamp(0.1, 10.);
        }
    }
}
