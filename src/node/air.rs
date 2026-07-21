use bevy::ecs::component::Component;

use crate::grid::common::BasicNode;

#[derive(Component)]
pub struct Air;
impl BasicNode for Air {
    fn get_id() -> String {
        "air".to_string()
    }
    fn remove(commands: &mut bevy::ecs::system::EntityCommands) {
        commands.remove::<Air>();
    }
    fn spawn(commands: &mut bevy::ecs::system::Commands, entity: bevy::ecs::entity::Entity) {
        commands.entity(entity).insert(
            Air
        );
    }
}
