//! # Path: src/nodes/plugins.rs

use bevy::prelude::*;
use crate::nodes::{
    *,
    commons::*,
};

pub struct NodePlugins;
impl Plugin for NodePlugins {
    fn build(&self, app: &mut App) {
        register::<empty::Empty>(app);
        register::<clay_ore::ClayOre>(app);
    }
}

fn register<T: Registerable> (app: &mut App) {
    T::register(app);
}
