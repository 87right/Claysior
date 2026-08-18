use crate::{gui::trigger::GridClicked, prelude::*};

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        insert_resource(app);
        app.add_systems(Startup, create_empty_world);
    }
}

fn insert_resource(app: &mut App) {
    app.insert_resource(GridEntityMap::default());
    app.insert_resource(WorldGeneratingSetting::default());
}


pub fn create_empty_world(
    mut commands: Commands,
    mut grid: ResMut<GridEntityMap>,
    setting: Res<WorldGeneratingSetting>,
) {
    for y in 0..(setting.height) {
        for x in 0..(setting.width) {
            let pos = GridPos::new(x, y);
            grid.insert(
                pos,
                commands.spawn((
                    pos,
                    pos.into_transform(3. - y as f32 / setting.height as f32),
                    node::Air,
                )).id()
            );
        }
    }
    commands.spawn(interactive::Grid {
        base: Vec2 { x: 0.0, y: 0.0 },
        scale: Vec2 { x: GameSetting::CELL_SIZE, y: GameSetting::CELL_SIZE },
        size: IVec2 { x: setting.width as i32, y: setting.height as i32 },
    }).observe(test);
}

fn test(
    trigger: On<GridClicked>, 
    mut commands: Commands, 
    mut inv_q: Query<&mut Inventory<Item>>,
    grid: Res<GridEntityMap>
) {
    let pos = trigger.index;
    if let Some(e) = grid.get(GridPos { x: pos.x as u64, y: pos.y as u64 }) {
        if let Ok(mut inv) = inv_q.get_mut(e) {
            if inv.get(SlotID(0)).and_then(|x| x.get().0).is_none() {
                let mut slot = MaterialSlot::default();
                slot.set(Some(Item::Clay), 1);
                inv.apply_buff(&MaterialSlotBuff { 
                    slot,
                    id: SlotID(0), 
                });
                info!("クリックされたインベントリにテスト用アイテムを挿入しました: {}個", inv.get(SlotID(0)).unwrap().get().1);
            } else {
                info!("現在の個数: {}個", inv.get(SlotID(0)).unwrap().get().1);
            }
        } else {
            commands.entity(e).insert((
                Channel::<Item>::default()
                    .add_port(
                        PortType::Input,
                        Port::default()
                            .configure_target(
                                GridSlice::Any
                            )
                    ).add_port(
                        PortType::Output,
                        Port::default()
                            .configure_target(
                                GridSlice::Specific { pos: GridPos { x: 1, y: 0 } }
                            ) 
                    ),
                Inventory::<Item>::new(1),
            ));
            info!("チャンネルとインベントリを挿入しました");
        }
    }
}
