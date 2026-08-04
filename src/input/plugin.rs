use crate::prelude::*;

use core::hash::Hash;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        register_system_set(app);
        insert_button_input::<KeyCode>(app);
        insert_button_input::<MouseButton>(app);
    }
}

fn register_system_set(app: &mut App) {
    app.configure_sets(Update, (
        InputLayer::First,
        InputLayer::GUI.after(InputLayer::First),
        InputLayer::Grid.after(InputLayer::GUI),
        InputLayer::Camera.after(InputLayer::Grid),
        InputLayer::Last.after(InputLayer::Camera),
    ));
}

fn insert_button_input<T>(app: &mut App)
where 
    T: Clone + Eq + Hash + Send + Sync + 'static 
{
    app.insert_resource(LayeredButtonInput::<T>::default());
    app.add_systems(Update, (|mut buttons: ResMut<LayeredButtonInput<T>>, from: Res<ButtonInput<T>>| {
        buttons.reset(&from);
    }).in_set(InputLayer::First));
}
