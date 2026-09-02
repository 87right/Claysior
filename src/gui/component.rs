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

#[derive(Component)]
pub struct AutoInventoryDisplayUnit<T> 
where 
    T: DisplayableManuMaterial
{
    pub index: SlotID,
    pub curr: Option<Entity>,
    pub pos: Vec2,
    phantom_data: Option<T>,
}

impl<T> AutoInventoryDisplayUnit<T>
where 
    T: DisplayableManuMaterial
{
    pub fn new(slot_id: SlotID, pos: Vec2) -> Self {
        Self {
            index: slot_id,
            curr: None,
            pos,
            phantom_data: None,
        }
    }
}

#[derive(Component)]
pub struct AutoInventoryDisplay<T> 
where 
    T: DisplayableManuMaterial
{
    pub content: Vec<AutoInventoryDisplayUnit<T>>
}
impl<T> AutoInventoryDisplay<T>
where 
    T: DisplayableManuMaterial
{
    pub fn new(f: fn(Self) -> Self) -> Self {
        f(Self {
            content: vec![]
        })
    }
    pub fn add(mut self, slot_id: SlotID, pos: Vec2) -> Self {
        self.content.push(AutoInventoryDisplayUnit::<T>::new(slot_id, pos));
        self
    }
}

#[derive(Component)]
pub struct LinearInterpolation {
    pub from: Vec2,
    pub to: Vec2,
    pub timer: Timer,
    pub duration: f32,
}

impl LinearInterpolation {
    pub fn get_cur_pos(&self) -> Vec2 {
        (self.to - self.from) * (1.0 - self.timer.remaining().as_secs_f32() / self.duration) + self.from
    }
}
