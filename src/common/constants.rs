use bevy::camera::visibility::RenderLayers;

pub struct GameLayer;
impl GameLayer {
    pub const MAIN: RenderLayers = RenderLayers::layer(0);
    pub const GUI: RenderLayers = RenderLayers::layer(1);
}

pub struct GameSetting;
impl GameSetting {
    pub const CELL_SIZE: f32 = 32.0;
    pub const TPS: f64 = 20.0;
}
