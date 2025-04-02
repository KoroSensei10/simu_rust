use super::pixel_grid::PixelGrid;
use super::pixel_types::{Pixel, PixelType, SandColor};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use rand::Rng;

const SPAWN_RADIUS: i32 = 10;
const PIXEL_SIZE: i32 = 10;

#[derive(Resource)]
pub struct PixelSpawnType {
    pub pixel_type: PixelType,
}
impl Default for PixelSpawnType {
    fn default() -> Self {
        PixelSpawnType {
            pixel_type: PixelType::SAND(SandColor::default()),
        }
    }
}

pub fn change_pixel_spawn_type(
    mut pixel_spawn_type: ResMut<PixelSpawnType>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        pixel_spawn_type.pixel_type = match pixel_spawn_type.pixel_type {
            PixelType::SAND(_) => PixelType::WATER {
                stagnation_count: 0,
                max_stagnation: 100,
            },
            _ => PixelType::SAND(SandColor::default()),
        };
    }
}

pub fn reset_grid(
    mut pixel_grid: ResMut<PixelGrid>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::KeyR) {
        return;
    }
    for (_, entity) in pixel_grid.grid.drain() {
        commands.entity(entity).try_despawn();
    }
    pixel_grid.grid.clear();
}

pub fn spawn_pixel_to_cursor_position(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    timer: Res<crate::timers::CastTimer>,
    pixel_spawn_type: Res<PixelSpawnType>,
    mut pixel_grid: ResMut<PixelGrid>,
    mut commands: Commands,
) {
    let should_spawn = mouse_button_input.just_pressed(MouseButton::Left)
        || (timer.0.just_finished() && mouse_button_input.pressed(MouseButton::Left));

    if !should_spawn {
        return;
    }

    let Some(cursor_position) = crate::utils::get_cursor_pos(camera_query, windows) else {
        return;
    };

    let tile_position = (cursor_position / PIXEL_SIZE as f32).floor() * PIXEL_SIZE as f32;

    for dx in -SPAWN_RADIUS..=SPAWN_RADIUS {
        for dy in -SPAWN_RADIUS..=SPAWN_RADIUS {
            let dist_sq = dx * dx + dy * dy;

            if !((SPAWN_RADIUS - 1).pow(2)..=SPAWN_RADIUS.pow(2)).contains(&dist_sq) {
                continue;
            }

            let spawn_pos = tile_position + Vec2::new(dx as f32, dy as f32) * PIXEL_SIZE as f32;
            let grid_key = (spawn_pos.x as i32, spawn_pos.y as i32);

            if let std::collections::hash_map::Entry::Vacant(entry) =
                pixel_grid.grid.entry(grid_key)
            {
                let pixel_type = pixel_spawn_type.pixel_type.clone();
                let pixel_entity = commands
                    .spawn((
                        Sprite {
                            color: pixel_type.as_color(),
                            custom_size: Some(Vec2::splat(PIXEL_SIZE as f32)),
                            ..default()
                        },
                        Pixel { pixel_type },
                        Transform::from_translation((spawn_pos.x, spawn_pos.y, 0.).into()),
                    ))
                    .id();
                entry.insert(pixel_entity);
            }
        }
    }
}

pub fn update_particules(
    mut query: Query<(Entity, &mut Transform, &mut Pixel), With<Pixel>>,
    mut pixel_grid: ResMut<PixelGrid>,
) {
    let mut updates: Vec<(Entity, i32, i32, i32, i32)> = Vec::new();

    for (entity, transform, mut pixel) in query.iter_mut() {
        match &mut pixel.pixel_type {
            PixelType::SAND(_) => {
                let x = (transform.translation.x / PIXEL_SIZE as f32).floor() as i32;
                let y = (transform.translation.y / PIXEL_SIZE as f32).floor() as i32;

                let new_y = y - 1;
                if (new_y as f32) * PIXEL_SIZE as f32 > -(WINDOW_HEIGHT as f32 / 2.0) {
                    if !pixel_grid.grid.contains_key(&(x, new_y)) {
                        updates.push((entity, x, y, x, new_y));
                    } else {
                        let mut rng = rand::rng();
                        let bleft_available = !pixel_grid.grid.contains_key(&(x - 1, new_y))
                            && ((x as f32 - 1.) * PIXEL_SIZE as f32)
                                > (-(WINDOW_WIDTH as f32 / 2.));
                        let bright_available = !pixel_grid.grid.contains_key(&(x + 1, new_y))
                            && ((x as f32 + 1.) * PIXEL_SIZE as f32) < (WINDOW_WIDTH as f32 / 2.);
                        if bleft_available || bright_available {
                            if bleft_available && bright_available {
                                if rng.random_bool(0.5) {
                                    updates.push((entity, x, y, x - 1, new_y));
                                } else {
                                    updates.push((entity, x, y, x + 1, new_y));
                                }
                            } else if bleft_available {
                                updates.push((entity, x, y, x - 1, new_y));
                            } else if bright_available {
                                updates.push((entity, x, y, x + 1, new_y));
                            }
                        }
                    }
                }
            }
            PixelType::WATER {
                stagnation_count,
                max_stagnation,
            } => {
                let x = (transform.translation.x / PIXEL_SIZE as f32).floor() as i32;
                let y = (transform.translation.y / PIXEL_SIZE as f32).floor() as i32;
                let new_y = y - 1;

                if new_y as f32 * PIXEL_SIZE as f32 > -(WINDOW_HEIGHT as f32 / 2.0) {
                    if !pixel_grid.grid.contains_key(&(x, new_y)) {
                        // Tombe tout droit
                        updates.push((entity, x, y, x, new_y));
                        *stagnation_count = 0;
                        continue;
                    }
                }
                if stagnation_count > max_stagnation {
                    continue;
                }

                // Mouvement latéral si chute impossible
                let x_left = x - 1;
                let x_right = x + 1;
                let left_free = !pixel_grid.grid.contains_key(&(x_left, y));
                let right_free = !pixel_grid.grid.contains_key(&(x_right, y));

                if left_free || right_free {
                    let move_x = if left_free && right_free {
                        if rand::random() { x_left } else { x_right }
                    } else if left_free {
                        x_left
                    } else {
                        x_right
                    };
                    *stagnation_count += 1;
                    updates.push((entity, x, y, move_x, y));
                }
            }
            _ => {}
        }
    }
    for (entity, old_x, old_y, new_x, new_y) in updates {
        if let Ok((_, mut transform, _)) = query.get_mut(entity) {
            transform.translation.y = (new_y as f32 * PIXEL_SIZE as f32).floor();
            transform.translation.x = (new_x as f32 * PIXEL_SIZE as f32).floor();
            pixel_grid.remove_pixel(old_x, old_y);
            pixel_grid.set_pixel(new_x, new_y, entity);
        }
    }
}
