use crate::prelude::*;

#[derive(SystemSet, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputLayer {
    GUI,
    Grid,
    Camera,
}
