use crate::prelude::*;

#[derive(EntityEvent)]
pub struct GridClicked {
    pub entity: Entity,
    pub index: IVec2, 
}
