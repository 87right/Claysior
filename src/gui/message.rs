use crate::prelude::*;

#[derive(Message)]
pub struct MaterialMoved {
    pub from: MaterialSlotIdentifyer,
    pub to: Vec<MaterialSlotIdentifyer>,
    pub is_taken: bool,
    pub duration: f32,
}
