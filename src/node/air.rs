use crate::prelude::*;

#[derive(Component)]
pub struct Air;
impl BasicNode for Air {
    type All = Air;
    const DEFAULT: Self::All = Air;
}


