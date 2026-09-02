use bevy::{sprite::Anchor, window::WindowResized};

use crate::{camera::component::MainCamera, prelude::*};

const PADDING: f32 = 0.999;

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debug(false, Timer::from_seconds(1., TimerMode::Repeating)));
        app.add_systems(Startup, (on_grid_reload, register_hooks::<Item>));
        app.add_systems(PostUpdate, (
            update_debug,
            switch_debug_mode,
            on_window_resized,
            draw_grid_line,
            consume_image_buff,
            auto_item_display::<Item>,
            linear_interpolation,
        ));
        app.add_systems(Update, (
            bind_full_scr,
            on_grid_clicked,
        ).in_set(InputLayer::GUI));
        app.add_message::<MaterialMoved>();
    }
}

#[derive(Resource)]
struct Debug(bool, Timer);

#[derive(Component)]
struct FPSViewer;

fn update_debug(
    mut res: ResMut<Debug>,
    fps_viewer: Query<&mut Text2d, With<FPSViewer>>,
    time: Res<Time>,
) {
    if res.0 && res.1.tick(time.delta()).just_finished() {
        for mut text in fps_viewer {
            text.0 = format!("{:.1}FPS", 1. / time.delta().as_secs_f32());
        }
    }
}

fn register_hooks<T: DisplayableManuMaterial>(world: &mut World) {
    world.register_component_hooks::<AutoInventoryDisplay<T>>()
        .on_remove(|mut world, context| {
            let mut entities = vec![];
            if let Ok(entity) = world.get_entity(context.entity)
            && let Ok(display_entity) = entity.get_components::<&AutoInventoryDisplay<T>>() { 
                for display_entity in &display_entity.content {
                    if let Some(display_entity) = display_entity.curr {
                        entities.push(display_entity);
                    }
                }
            }
            for display_entity in entities {
                world.commands().entity(display_entity).despawn();
            }
        }
    );
}

fn draw_grid_line(
    mut gizmos: Gizmos,
    setting: Res<WorldGeneratingSetting>,
    debug: ResMut<Debug>,
) {
    if debug.0 {
        gizmos.cross_2d(Vec2::new(
            GameSetting::CELL_SIZE * setting.width  as f32 / 2.,
            GameSetting::CELL_SIZE * setting.height as f32 / 2., 
        ), 12., bevy::color::palettes::css::FUCHSIA);
    }
}

fn switch_debug_mode(
    mut commands: Commands,
    mut debug: ResMut<Debug>,
    keys: Res<LayeredButtonInput<KeyCode>>,
    q: Query<Entity, With<FPSViewer>>,
    time: Res<Time>,
    window: Single<&Window>
) {
    if keys.just_released(KeyCode::F3) {
        debug.0 = !debug.0;
        if debug.0 {
            commands.spawn((
                Text2d::new(format!("{:.1}FPS", 1. / time.delta().as_secs_f32())),
                FPSViewer,
                Transform::from_xyz((-window.width() / 2.) * PADDING, (window.height() / 2.) * PADDING, 1.),
                GameLayer::GUI,
                Anchor::TOP_LEFT,
                GUITransform {
                    x: 0.,
                    y: 1024.,
                    width: 50.,
                    height: 100., 
                }
            ));
        } else {
            for e in q {
                commands.entity(e).despawn();   
            }
        }
    }
}

#[derive(Component)]
pub struct GUITransform {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

fn on_window_resized(
    mut resize_reader: MessageReader<WindowResized>,
    mut gui_q: Query<(&mut Transform, &GUITransform)>,
) {
    for e in resize_reader.read() {
        let x_scale = e.width / 1024.;
        let y_scale = e.height / 1024.;
        for (mut transform, gui) in &mut gui_q {
            transform.translation.x = (gui.x - 512.) * x_scale * PADDING;
            transform.translation.y = (gui.y - 512.) * y_scale * PADDING;
        }
    }
}

fn on_grid_reload(
    mut commands: Commands,
    background: Query<&mut Sprite, With<BackGround>>,
    setting: Res<WorldGeneratingSetting>,
    asset_server: Res<AssetServer>,
) {
    let mut exist = false;
    for mut sprite in background {
        exist = true;
        *sprite = Sprite::from_image(
            asset_server.load(format!("textures/background/{}.png", setting.background))
        );
    }
    if !exist {
        commands.spawn((
            Sprite::from_image(
                asset_server.load(format!("textures/background/{}.png", setting.background))
            ),
            BackGround,
            Anchor::BOTTOM_LEFT,
            Transform::from_xyz(
                0.,
                0.,
                0.,
            ),
            GameLayer::MAIN,
        ));
    }
}

fn bind_full_scr(
    mut commands: Commands,
    fs_q: Query<Entity, With<gui_win::FullScr>>,
    mut keys: ResMut<LayeredButtonInput<KeyCode>>,
    mut mouse: ResMut<LayeredButtonInput<MouseButton>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        println!("Escape Pressed");
        let mut exist = false;
        for fs in fs_q {
            commands.entity(fs).despawn();
            exist = true;
        }
        if exist {
            keys.consume();
        } else {
            commands.spawn(gui_win::FullScr {});
        }
    } else {
        // debug
        if !fs_q.is_empty() {
            keys.consume();
            mouse.consume();
        }
    }
}

fn on_grid_clicked(
    mut commands: Commands,
    mut mouse: ResMut<LayeredButtonInput<MouseButton>>, 
    grids: Query<(&interactive::Grid, Entity)>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, transform) = camera.into_inner();
    if mouse.just_released(MouseButton::Left) 
    && let Some(cursor) = window.into_inner().cursor_position() {
        for (grid, e) in grids {
            if let Ok(cursor) = camera.viewport_to_world_2d(transform, cursor)
            && let Some(pos) = grid.check(cursor) {
                commands.trigger(crate::gui::trigger::GridClicked {
                    entity: e,
                    index: pos,
                });
                mouse.consume();
                return;
            }
        }
    }
}

fn consume_image_buff(
    mut commands: Commands,
    tasks: Query<(Entity, &texture_material_buff::FromImage)>,
    asset: Res<AssetServer>,
) {
    for (e, &texture_material_buff::FromImage(id)) in tasks {
        commands.entity(e) 
            .remove::<texture_material_buff::FromImage>()
            .insert(
                Sprite::from_image(
                    asset.load(id)
                )
            );
    }
}

fn auto_item_display<T>
(
    mut reader: MessageReader<MaterialMoved>,
    mut commands: Commands,
    mut targ: Query<(&Inventory<T>, &mut AutoInventoryDisplay<T>), Changed<Inventory<T>>>,
    grid: Res<GridEntityMap>,
)
where 
    T: DisplayableManuMaterial
{
    for moved_data in reader.read() {
        let mut stripped_entity = None;
        let mut from_pos = moved_data.from.pos.into_world_pos();
        if moved_data.is_taken
        && let Some(from_entity) = grid.get(moved_data.from.pos) 
        && let Ok((_, mut cont)) = targ.get_mut(from_entity) {
            for cont in &mut cont.content {
                if cont.index == moved_data.from.slot_id {
                    stripped_entity = cont.curr.take();
                    from_pos += cont.pos;
                    info!("Found");
                    break;
                }
            }
        }
        for MaterialSlotIdentifyer{pos, slot_id} in &moved_data.to {
            if let Some(to_entity) = grid.get(*pos)
            && let Ok((inv, mut cont)) = targ.get_mut(to_entity) {
                for cont in &mut cont.content {
                    if cont.index != *slot_id {
                        continue;
                    }
                    let li = LinearInterpolation {
                            from: from_pos,
                            to: pos.into_world_pos() + cont.pos,
                            timer: Timer::from_seconds(moved_data.duration, TimerMode::Once),
                            duration: moved_data.duration,
                    };
                    
                    if let Some(entity) = stripped_entity.take() {
                        cont.curr = Some(entity);
                        commands.entity(entity).insert(li);
                    } else if let Some(item) = inv.get(*slot_id) 
                    && let Some(item) = item.get_raw().0 {
                        cont.curr = Some(item.insert_texture(commands.spawn(li)).id());
                    }
                    break;
                } 
            }
        }
    }
}

fn linear_interpolation(
    mut commands: Commands,
    q: Query<(Entity, &mut Transform, &mut LinearInterpolation)>,
    time: Res<Time>
) {
    for (e, mut transform, mut data) in q {
        if data.timer.tick(time.delta()).just_finished() {
            commands.entity(e).remove::<LinearInterpolation>();
        }
        transform.translation = data.get_cur_pos().extend(transform.translation.z);
    }
}
