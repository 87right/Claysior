use bevy::prelude::*;

#[derive(Component)]
pub struct Channel<T: Component> {
    pub input: Vec<Port<T>>,
    pub output: Vec<Port<T>>,
    pub gather: Vec<Port<T>>,
}

#[derive(Component)]
pub struct Port<T: Component> {
    dummy: T
}
