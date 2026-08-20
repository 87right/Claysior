pub mod grid {
    use crate::prelude::*;

    #[derive(EntityEvent)]
    pub struct Clicked {
        entity: Entity
    }
}
