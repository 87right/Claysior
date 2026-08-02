use crate::gui::prelude::*;

pub fn spawn_free_sprite(
    commands: &mut Commands,
    from: Vec2,
    to: Vec2,
    ticks: u64,
    texture_source: String,
) {
    commands.spawn((
        GUICore::free_sprite(from, to, ticks),
        TextureBuff(texture_source),
    ));
}
