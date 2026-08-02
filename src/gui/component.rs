use crate::{common::constant::GAME_TPS, gui::prelude::*};

#[derive(Component, Clone, Copy)]
pub enum GUICore {
    FreeSprite {
        from: Vec2,
        to: Vec2,
        duration: f32,
        pros: f32,
    },
}
impl GUICore {
    pub fn free_sprite(from: Vec2, to: Vec2, ticks: u64) -> Self {
        let sec = ticks as f32 / GAME_TPS as f32;
        GUICore::FreeSprite { 
            from, 
            to, 
            duration: sec, 
            pros: 0.,
        }
    }
}
