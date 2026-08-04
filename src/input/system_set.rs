use crate::prelude::*;

#[derive(SystemSet, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputLayer {
    First,
    GUI,
    Grid,
    Camera,
    Last,
}
