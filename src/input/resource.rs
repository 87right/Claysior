use std::hash::Hash;

use bevy::{
    prelude::*,
};

#[derive(Resource, Debug, Clone)]
pub struct LayeredButtonInput<T> 
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    val: ButtonInput<T>,
    consumed: bool,
}
impl<T> Default for LayeredButtonInput<T>
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    fn default() -> Self {
        Self {
            val: Default::default(),
            consumed: false,
        }
    }
}
impl<T> LayeredButtonInput<T>
where 
    T: Clone + Eq + Hash + Send + Sync + 'static
{
    pub fn refresh(&mut self, button_input: &ButtonInput<T>) {
        self.val = button_input.clone();
    }
    pub fn consume(&mut self) {
        self.consumed = true;
    }
    pub fn is_consumed(&self) -> bool {
        self.consumed
    }
    pub fn reset_consumed(&mut self) -> bool {
        let pre = self.consumed;
        self.consumed = false;
        pre
    }
    pub fn pressed(&self, key: T) -> bool {
        !self.consumed && self.val.pressed(key)
    }
    pub fn just_pressed(&self, key: T) -> bool {
        !self.consumed && self.val.just_pressed(key)
    }
    pub fn just_released(&self, key: T) -> bool {
        !self.consumed && self.val.just_released(key)
    }
    pub fn all_pressed(&self, key: impl IntoIterator<Item = T>) -> bool {
        !self.consumed && self.val.all_pressed(key)
    }
    pub fn all_just_pressed(&self, key: impl IntoIterator<Item = T>) -> bool {
        !self.consumed && self.val.all_just_pressed(key)
    }
    pub fn all_just_released(&self, key: impl IntoIterator<Item = T>) -> bool {
        !self.consumed && self.val.all_just_released(key)
    }
    pub fn any_pressed(&self, key: impl IntoIterator<Item = T>) -> bool {
        !self.consumed && self.val.any_pressed(key)
    }
    pub fn any_just_pressed(&self, key: impl IntoIterator<Item = T>) -> bool {
        !self.consumed && self.val.any_just_pressed(key)
    }
    pub fn any_just_released(&self, key: impl IntoIterator<Item = T>) -> bool {
        !self.consumed && self.val.any_just_released(key)
    }
    pub fn reset(&mut self, key: T) {
        self.val.reset(key);
    }
    pub fn reset_all(&mut self) {
        self.val.reset_all();
    }
    pub fn clear(&mut self) {
        self.val.clear();
        self.consumed = false;
    }
}
