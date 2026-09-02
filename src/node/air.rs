use crate::prelude::*;

#[derive(Component)]
#[require(
    Inventory::<Item>::test_constructor(4, |mut x| {
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
            PortType::Output,
            Port::default().configure_target(
                GridSlice::Specific { pos: GridPos {x: 0, y: 1} }
            )
        ).add_port(
            PortType::Input,
            Port::default().configure_target(GridSlice::Any)
        ).configure_cd(10)
    }),
    AutoInventoryDisplay::<Item>::new(|x| {
        x
            .add(SlotID(0), Vec2 { x: 7.5, y: 7.5 })
            .add(SlotID(1), Vec2 { x: 7.5, y: 22.5 })
            .add(SlotID(2), Vec2 { x: 22.5, y: 7.5 })
            .add(SlotID(3), Vec2 { x: 22.5, y: 22.5 })
    }),
)]
pub struct Air;
impl BasicNode for Air {
    fn plugin(app: &mut App) {
        app.add_observer(on_clicked);
    }
}

fn on_clicked(
    trigger: On<grid::Clicked>,
    mut air_q: Query<&mut Inventory::<Item>, With<Air>>,
) {
    if let Ok(mut inv) = air_q.get_mut(trigger.entity) 
    && let Some(slot) = inv.get_mut(SlotID(0)) {
        slot.set(Some(Item::Clay), 1);
    }
}
