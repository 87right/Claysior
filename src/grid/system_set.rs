use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GridFixed {
    OnPlaced,
    ApplyDiff,
    IOExecute,
    MainUpdate,
    OnRemoved,
    Cleanup,
}
