//! # Path: src/grid/components.rs

use bevy::prelude::*;
use std::collections::HashMap;


#[derive(Resource, Default)]
pub struct WorldGrid (pub HashMap<IVec2, Entity>);

#[derive(Component)]
pub struct GridPos (pub IVec2);
