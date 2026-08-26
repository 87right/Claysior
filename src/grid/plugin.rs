use crate::{gui::trigger::GridClicked, prelude::*};

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        insert_resource(app);
        configure_system_set(app);
        app.add_systems(Startup, create_empty_world);
    }
}

fn insert_resource(app: &mut App) {
    app.insert_resource(GridEntityMap::default());
    app.insert_resource(WorldGeneratingSetting::default());
}

fn configure_system_set(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_hz(GameSetting::TPS));
    app.configure_sets(
        FixedUpdate,
        (
            GridSystem::Logistics, 
            GridSystem::PreUpdate, 
            GridSystem::Update,
            GridSystem::PostUpdate,
            GridSystem::CleanUp,
        ).chain());
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
    grid: Res<GridEntityMap>
) {
    if let Some(entity) = grid.get(GridPos { x: trigger.index.x as u64, y: trigger.index.y as u64 }) {
        commands.trigger(grid::Clicked { entity });
    }
}
