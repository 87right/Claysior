use bevy::prelude::*;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum InputLayer {
    First,
    UI,
    Grid,
    Last,
}
