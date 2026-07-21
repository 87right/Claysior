use bevy::prelude::*;

pub trait BasicNode {
    fn remove(commands: &mut EntityCommands);
    fn spawn(commands: &mut Commands, entity: Entity);
    fn get_id() -> String;
    fn register(app: &mut App) {}
}
