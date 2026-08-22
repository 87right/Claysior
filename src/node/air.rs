use crate::prelude::*;

#[derive(Component)]
#[require(
    Inventory::<Item>::new(1),
    Channel::<Item>::test_constructor(|x| {
        x.add_port(
            PortType::Output,
            Port::default().configure_target(
                GridSlice::Specific { pos: GridPos {x: 1, y: 1} }
            )
        ).add_port(
            PortType::Input,
            Port::default().configure_target(GridSlice::Any)
        )
    }),
    AutoInventoryDisplay::<Item>::new(SlotID(0), Vec2 { x: 30.0, y: 0.0 })
)]
pub struct Air;
impl BasicNode for Air {}
