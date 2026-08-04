use crate::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn register_system_set(app: &mut App) {
    app.configure_sets(Update, (
        InputLayer::GUI,
        InputLayer::Grid.after(InputLayer::GUI),
        InputLayer::Camera.after(InputLayer::Grid),
    ));
}
