use bevy::input::mouse::MouseWheel;

use crate::prelude::*;

use crate::camera::component::CameraDragData;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraDragData::default());
        app.add_systems(Startup, (spawn_camera, spawn_test_sprite));
        app.add_systems(Update, (
            camera_movement_system,
            camera_zoom_system,
        ).in_set(InputLayer::Camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
        Transform::from_xyz(0., 0., 5.),
    ));
}

fn spawn_test_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Sprite::from_image(
            asset_server.load("textures/background/basic_tile.png")
        )
    ));
}

fn camera_movement_system(
    mut camera_drag_data: ResMut<CameraDragData>,
    camera_query: Single<(&mut Transform, &Projection), With<Camera>>,
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

fn camera_zoom_system(
    mut msr_scroll: MessageReader<MouseWheel>,
    mouse: Res<LayeredButtonInput<MouseButton>>,
    projection_query: Single<&mut Projection, With<Camera>>,
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
