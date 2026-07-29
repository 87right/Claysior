use crate::recipe::prelude::*;

impl<Input, Output> Recipe<Input, Output> 
where
    Input: RecipeInput,
    Output: RecipeOutput,
{
    pub fn new() -> Self {
        Self(HashMap::<Input, Output>::new())
    }
    pub fn insert(&mut self, k: Input, v: Output) -> Option<Output> {
        self.0.insert(k, v)
    }
    pub fn get(&self, k: &Input) -> Option<&Output> {
        self.0.get(k)
    }
}