use bevy::prelude::*;

use crate::grid::component::*;

pub trait BasicNode {
    fn remove(commands: &mut EntityCommands);
    fn spawn(commands: &mut Commands, entity: Entity);
    fn get_id() -> String;
    fn register(app: &mut App);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    X,
    Y,
    NegX,
    NegY,
}
impl Direction {
    pub const ALL: [Self; 4] = [Self::X, Self::Y, Self::NegX, Self::NegY];
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Self::NegX,
            1 => Self::NegY,
            2 => Self::X,
            3 => Self::Y,
            _ => Self::NegX, // default
        }
    }
    pub fn get_id(&self) -> u8 {
        match self {
            Self::NegX => 0,
            Self::NegY => 1,
            Self::X => 2,
            Self::Y => 3,
        }
    }
    pub fn inverse(&self) -> Self {
        match self {
            Self::NegX => Self::X,
            Self::NegY => Self::Y,
            Self::X => Self::NegX,
            Self::Y => Self::NegY,
        }
    }
    pub fn into_grid_pos(self) -> GridPos {
        match self {
            Self::NegX => GridPos::NEG_X,
            Self::NegY => GridPos::NEG_Y,
            Self::X => GridPos::X,
            Self::Y => GridPos::Y,
        }
    }
}
