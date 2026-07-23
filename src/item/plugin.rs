use bevy::prelude::*;

use crate::item::component::*;

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, _app: &mut App) {}
}

fn item_entity_age_system(mut commands: Commands, item_q: Query<(&mut Age, Entity), With<Item>>) {
    for (mut age, e) in item_q {
        age.0 += 1;
        if age.0 == 10 {
            commands.entity(e).insert(Pickupable);
        }
    }
}
