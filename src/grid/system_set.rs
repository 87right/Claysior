use crate::prelude::*;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum GridSystem {
    Logistics,
    PreUpdate,
    Update,
    PostUpdate,
    CleanUp,
}
