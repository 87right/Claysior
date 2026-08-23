use crate::prelude::*;

pub struct GridCommands<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    pos: GridPos,
}

pub trait GridCommand<'w, 's> {
    fn grid<'a>(&'a mut self, pos: GridPos) -> GridCommands<'a, 'w, 's>;
}

impl<'w, 's> GridCommand<'w, 's> for Commands<'w, 's> {
    fn grid<'a>(&'a mut self, pos: GridPos) -> GridCommands<'a, 'w, 's> {
        GridCommands { 
            commands: self, 
            pos 
        }
    }
}

impl<'a, 'w, 's> GridCommands<'a, 'w, 's> {
    pub fn replace<T: Bundle>(self, value: T) -> Self {
        let pos = self.pos;
        self.commands.queue(move |world: &mut World| {
            let mut z = 0.0;
            if let Some(setting) = world.get_resource::<WorldGeneratingSetting>() {
                z = 3. - pos.y as f32 / setting.height as f32;
            }
            let new_entity = world.spawn((value, pos, pos.into_transform(z))).id();
            if let Some(mut grid) = world.get_resource_mut::<GridEntityMap>() {
                if let Some(entity) = grid.insert(pos, new_entity)
                && let Ok(entity) = world.get_entity_mut(entity){
                    entity.despawn();
                }
            }
        });
        self
    }
}
