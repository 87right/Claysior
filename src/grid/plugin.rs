use bevy::{
    prelude::*,
};
use crate::{
    grid::{
        system_set::*,
    }
};

pub struct GridPlugin;
impl Plugin for GridPlugin{
    fn build(&self, app: &mut App) {
        register_grid_update_schedule(app);
    }
}

fn register_grid_update_schedule(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_hz(1.0));
    app.configure_sets(FixedUpdate, (
        GridFixed::OnPlaced,
        GridFixed::IOReserve.after(GridFixed::OnPlaced),
        GridFixed::IOExecute.after(GridFixed::IOReserve),
        GridFixed::MainUpdate.after(GridFixed::IOExecute),
        GridFixed::Cleanup.after(GridFixed::MainUpdate),
        GridFixed::OnBroken.after(GridFixed::Cleanup),
    ));
}
