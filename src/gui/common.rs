use crate::prelude::*;

pub trait DisplayableManuMaterial: ManuMaterial {
    fn insert_texture<'a>(&self, commands: EntityCommands<'a>) -> EntityCommands<'a>;
}

pub mod texture_material_buff {
    use crate::prelude::*;

    #[derive(Component)]
    pub struct FromImage(pub &'static str);
}
