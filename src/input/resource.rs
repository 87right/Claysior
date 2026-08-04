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
