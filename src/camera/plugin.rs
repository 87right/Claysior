use crate::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_test_sprite));
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
