use crate::prelude::*;

#[derive(Component)]
#[require(
    Inventory::<Item>::test_constructor(1, |mut x| {
        for slot in x.iter_mut() {
            slot.setting().set_max_volume(1);
        }
        x
    }),
    Channel::<Item>::test_constructor(|x| {
        x.add_port(
            PortType::Output,
            Port::default().configure_target(
                GridSlice::Specific { pos: GridPos {x: 1, y: 0} }
            )
        ).add_port(
            PortType::Input,
            Port::default().configure_target(GridSlice::Any)
        )
    }),
    AutoInventoryDisplay::<Item>::new(SlotID(0), Vec2 { x: 15.0, y: 15.0 })
)]
pub struct Air;
impl BasicNode for Air {}
