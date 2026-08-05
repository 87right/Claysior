use bevy::camera::visibility::RenderLayers;

pub struct GameLayer;
impl GameLayer {
    pub const MAIN: RenderLayers = RenderLayers::layer(0);
    pub const GUI: RenderLayers = RenderLayers::layer(1);
}
