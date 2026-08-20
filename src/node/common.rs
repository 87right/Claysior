use crate::prelude::*;

pub trait BasicNode: Component + Sized {
    fn plugin(_app: &mut App) {}
}
