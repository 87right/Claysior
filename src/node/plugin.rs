use crate::prelude::*;

pub struct NodePlugin;
impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, node::Air::register_hooks);
    }
}
