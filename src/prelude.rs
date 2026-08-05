pub use {
    bevy::prelude::*,
    crate::{
        input::{
            system_set::*,
            resource::LayeredButtonInput,
        },
        common::{
            constants::*,
        },
        manu_material::{
            common::*,
            inventory::*,
            channel::*,
        }
    }
};

pub mod plugin {
    pub use crate::{
        input::plugin::InputPlugin,
        camera::plugin::CameraPlugin,
        gui::plugin::GUIPlugin,
    };
}
