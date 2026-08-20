use crate::prelude::*;

#[derive(Component)]
#[require(Inventory::<Item>::new(1))]
pub struct Air;
impl BasicNode for Air {}
