use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GridFixed {
    OnPlaced,
    ApplyDiff,
    IOReserve,
    IOExecute,
    MainUpdate,
    OnRemoved,
    Cleanup,
}