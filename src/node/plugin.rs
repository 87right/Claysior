use crate::prelude::*;

pub struct NodePlugin;
impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        node::Air::plugin(app);
    }
}
