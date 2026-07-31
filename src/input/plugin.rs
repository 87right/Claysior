use std::hash::Hash;

use bevy::prelude::*;

use crate::input::{
    resource::*,
    system_set::*,
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        register_system_set(app);
        register_button_input::<MouseButton>(app);
        register_button_input::<KeyCode>(app);
    }
}

fn register_system_set(app: &mut App) {
    app.configure_sets(Update, (
        InputLayer::First,
        InputLayer::UI.after(InputLayer::First),
        InputLayer::Grid.after(InputLayer::UI),
        InputLayer::Last.after(InputLayer::Grid),
    ));
}

fn register_button_input<T>(app: &mut App)
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    app.insert_resource(LayeredButtonInput::<T>::default());
    add_input_routine::<T>(app);
}

fn add_input_routine<T>(app: &mut App)
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    app.add_systems(Update, (
        write_input::<T>,
        reset_input::<T>,
    ).in_set(InputLayer::First));
}

fn reset_input<T>(mut buttons: ResMut<LayeredButtonInput<T>>)
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    buttons.reset_consumed();
}

fn write_input<T>(
    mut buttons: ResMut<LayeredButtonInput<T>>,
    from: Res<ButtonInput<T>>,
) 
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    buttons.refresh(&from);
}
