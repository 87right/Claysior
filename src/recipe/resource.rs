use crate::recipe::prelude::*;

#[derive(Resource)]
pub struct Recipe<Input, Output>(pub HashMap<Input, Output>)
where
    Input: RecipeInput,
    Output: RecipeOutput;
