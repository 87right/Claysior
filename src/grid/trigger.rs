pub mod grid {
    use crate::prelude::*;

    #[derive(EntityEvent)]
    pub struct Clicked {
        pub entity: Entity
    }
}
