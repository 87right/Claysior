use crate::prelude::*;

#[derive(Component)]
pub struct BackGround;

pub mod gui_win{
    use crate::prelude::*;

    #[derive(Component)]
    pub struct Sub {

    }

    #[derive(Component)]
    pub struct PopUp {

    }

    #[derive(Component)]
    pub struct FullScr {

    }
}

pub mod interactive{
    use crate::prelude::*;

    #[derive(Component)]
    pub struct Grid{
        pub base: Vec2,
        pub scale: Vec2,
        pub size: IVec2,
    }

    impl Grid {
        pub fn check(&self, pos: Vec2) -> Option<IVec2> {
            let grid_x = ((pos.x - self.base.x) / self.scale.x).floor() as i32;
            let grid_y = ((pos.y - self.base.y) / self.scale.y).floor() as i32;
            if (0..self.size.x).contains(&grid_x) 
            && (0..self.size.y).contains(&grid_y) {
                Some(IVec2 { x: grid_x, y: grid_y })
            } else {
                None
            }
        }
    }
}
