use crate::recipe::prelude::*;

pub trait RecipeInput: Hash + PartialEq + Eq {}
pub trait RecipeOutput {}
