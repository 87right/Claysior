use crate::prelude::*;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        insert_resource(app);
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
                )).id()
            );
        }
    }
}

