// mod particles;
mod utils;

use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use utils::get_cursor_pos;

const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 200;
const PIXEL_SIZE: i32 = 1;
const SPAWN_RADIUS: i32 = 10;
const SPAWN_TIMER: f32 = 0.2;

#[derive(PartialEq, Clone)]
enum SandColor {
    YELLOW,
    ORANGE,
    RED,
}

impl SandColor {
    pub fn as_color(&self) -> Color {
        match self {
            SandColor::YELLOW => Color::srgb(1.0, 0.85, 0.3),
            SandColor::ORANGE => Color::srgb(1.0, 0.75, 0.3),
            SandColor::RED => Color::srgb(1.0, 0.65, 0.3),
        }
    }
}

impl Default for SandColor {
    fn default() -> Self {
        let mut rng = rand::rng();
        let random_value: f32 = rng.random_range(0.0..1.0);
        if random_value < 0.33 {
            SandColor::YELLOW
        } else if random_value < 0.66 {
            SandColor::ORANGE
        } else {
            SandColor::RED
        }
    }
}

#[derive(PartialEq, Clone)]
enum PixelType {
    AIR,
    WATER,
    SAND(SandColor),
}

impl PixelType {
    pub fn as_color(&self) -> Color {
        match self {
            PixelType::AIR => Color::NONE,
            PixelType::WATER => Color::srgb(0.0, 0.0, 1.0),
            PixelType::SAND(color) => color.as_color(),
        }
    }
}

#[derive(Component, Clone)]
struct Pixel {
    pixel_type: PixelType,
}
impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            pixel_type: PixelType::SAND(SandColor::default()), // Default pixel type
        }
    }
}

#[derive(Resource, Default)]
struct PixelGrid {
    grid: HashMap<(i32, i32), Entity>, // (x, y) -> Pixel Entity
}

#[derive(Resource)]
struct CastTimer(Timer);
impl CastTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(SPAWN_TIMER, TimerMode::Repeating))
    }
}
impl Default for CastTimer {
    fn default() -> Self {
        Self::new()
    }
}

fn falling_sand(
    mut query: Query<(Entity, &mut Transform, &mut Pixel)>,
    mut pixel_grid: ResMut<PixelGrid>,
) {
    let mut updates: Vec<(Entity, i32, i32, i32, i32)> = Vec::new();

    for (entity, transform, _) in query.iter().filter(|e| e.2.pixel_type != PixelType::AIR) {
        let x = (transform.translation.x / PIXEL_SIZE as f32) as i32;
        let y = (transform.translation.y / PIXEL_SIZE as f32) as i32;

        let new_y = y - 1;
        if (new_y as f32) * PIXEL_SIZE as f32 > -(WINDOW_HEIGHT as f32 / 2.0) {
            if !pixel_grid.grid.contains_key(&(x, new_y)) {
                updates.push((entity, x, y, x, new_y));
            } else {
                let mut rng = rand::rng();
                let left_available = !pixel_grid.grid.contains_key(&(x - 1, new_y))
                    && ((x as f32 - 1.) * PIXEL_SIZE as f32) > (-(WINDOW_WIDTH as f32 / 2.));
                let right_available = !pixel_grid.grid.contains_key(&(x + 1, new_y))
                    && ((x as f32 + 1.) * PIXEL_SIZE as f32) < (WINDOW_WIDTH as f32 / 2.);
                if left_available || right_available {
                    if left_available && right_available {
                        if rng.random_bool(0.5) {
                            updates.push((entity, x, y, x - 1, new_y));
                        } else {
                            updates.push((entity, x, y, x + 1, new_y));
                        }
                    } else if left_available {
                        updates.push((entity, x, y, x - 1, new_y));
                    } else if right_available {
                        updates.push((entity, x, y, x + 1, new_y));
                    }
                }
            }
        }
    }

    // Applique les déplacements après l'itération
    for (entity, old_x, old_y, new_x, new_y) in updates {
        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
            transform.translation.y = new_y as f32 * PIXEL_SIZE as f32;
            transform.translation.x = new_x as f32 * PIXEL_SIZE as f32;
            pixel_grid.grid.remove(&(old_x, old_y));
            pixel_grid.grid.insert((new_x, new_y), entity);
        }
    }
}

fn spawn_pixel_to_cursor_position(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    time: Res<Time>,
    mut timer: ResMut<CastTimer>,
    mut pixel_grid: ResMut<PixelGrid>,
    mut commands: Commands,
) {
    if !timer.0.tick(time.delta()).just_finished() || !mouse_button_input.pressed(MouseButton::Left)
    {
        return;
    }

    let Some(cursor_position) = get_cursor_pos(camera_query, windows) else {
        return;
    };

    // let tile_position = cursor_position.trunc();
    let tile_position = Vec2::new(
        (cursor_position.x / PIXEL_SIZE as f32).floor() * PIXEL_SIZE as f32,
        (cursor_position.y / PIXEL_SIZE as f32).floor() * PIXEL_SIZE as f32,
    );

    for dx in -SPAWN_RADIUS..=SPAWN_RADIUS {
        for dy in -SPAWN_RADIUS..=SPAWN_RADIUS {
            if dx * dx + dy * dy <= SPAWN_RADIUS * SPAWN_RADIUS {
                let spawn_pos = Vec2::new(
                    tile_position.x + dx as f32 * PIXEL_SIZE as f32,
                    tile_position.y + dy as f32 * PIXEL_SIZE as f32,
                );

                let grid_key = (spawn_pos.x as i32, spawn_pos.y as i32);
                if pixel_grid.grid.contains_key(&grid_key) {
                    continue;
                }

                let pixel_type = PixelType::SAND(SandColor::default());
                let pixel_entity = commands
                    .spawn((
                        Sprite {
                            color: pixel_type.as_color(),
                            custom_size: Some(Vec2::new(PIXEL_SIZE as f32, PIXEL_SIZE as f32)),
                            anchor: bevy::sprite::Anchor::TopLeft,
                            ..default()
                        },
                        Pixel { pixel_type },
                        Transform::from_translation((spawn_pos.x, spawn_pos.y, 0.).into()),
                    ))
                    .id();

                pixel_grid.grid.insert(grid_key, pixel_entity);
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn reset_grid(
    mut pixel_grid: ResMut<PixelGrid>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::KeyR) {
        return;
    }
    // Despawn all entities in the pixel grid
    for (_, entity) in pixel_grid.grid.drain() {
        commands.entity(entity).despawn_recursive(); // generates some warnings
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pixel Snoup".to_string(),
                resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(PixelGrid::default())
        .insert_resource(CastTimer::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (falling_sand, spawn_pixel_to_cursor_position))
        .add_systems(Update, reset_grid)
        .run();
}
