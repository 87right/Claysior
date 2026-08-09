use crate::prelude::*;

use core::hash::Hash;

#[derive(Resource)]
pub struct LayeredButtonInput<T>
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    button_input: ButtonInput<T>,
    consumed: bool
}
impl<T> Default for LayeredButtonInput<T>
where 
    T: Clone + Eq + Hash + Send + Sync + 'static 
{
    fn default() -> Self {
        Self {
            button_input: Default::default(),
            consumed: false
        }
    }
}
impl<T> LayeredButtonInput<T> 
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    pub fn is_consumed(&self) -> bool {
        self.consumed
    }
    pub fn consume(&mut self) {
        self.consumed = true;
    }
    pub fn reset(&mut self, button_input: &ButtonInput<T>) {
        self.button_input = button_input.clone();
        self.consumed = false;
    }
    pub fn pressed(&self, input: T) -> bool {
        !self.consumed && self.button_input.pressed(input)
    }
    pub fn just_pressed(&self, input: T) -> bool {
        !self.consumed && self.button_input.just_pressed(input)
    }
    pub fn just_released(&self, input: T) -> bool {
        !self.consumed && self.button_input.just_released(input)
    }
}
