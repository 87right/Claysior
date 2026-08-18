use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

pub trait BasicNode: Component + Sized {
    type All: Bundle;
    const DEFAULT: Self::All;

    fn new() -> Self::All {
        Self::DEFAULT
    }
    fn register_hooks(world: &mut World) {
        world
            .register_component_hooks::<Self>()
            .on_add(on_add::<Self>)
            .on_remove(on_remove::<Self>);
    }
}

fn on_add<T>(mut world: DeferredWorld, context: HookContext) 
where 
    T: BasicNode
{
    world.commands().entity(context.entity).insert(T::new());
}

fn on_remove<T>(mut world: DeferredWorld, context: HookContext) 
where 
    T: BasicNode
{
    world.commands().entity(context.entity).remove::<T::All>();
}
