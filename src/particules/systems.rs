use super::pixel_grid::PixelGrid;
use super::pixel_types::{FRICTION, Pixel, PixelType, SandColor};
use super::utils::{check_sliding, check_vertical_position};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use std::collections::BTreeMap;

const SPAWN_RADIUS: i32 = 10;

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

    let tile_position = (cursor_position).floor();

    for dx in -SPAWN_RADIUS..=SPAWN_RADIUS {
        for dy in -SPAWN_RADIUS..=SPAWN_RADIUS {
            let dist_sq = dx * dx + dy * dy;

            if !((SPAWN_RADIUS - 1).pow(2)..=SPAWN_RADIUS.pow(2)).contains(&dist_sq) {
                continue;
            }

            let spawn_pos = tile_position + Vec2::new(dx as f32, dy as f32);
            let grid_key = (spawn_pos.x as i32, spawn_pos.y as i32);

            if let std::collections::hash_map::Entry::Vacant(entry) =
                pixel_grid.grid.entry(grid_key)
            {
                let pixel_type = pixel_spawn_type.pixel_type.clone();
                let pixel_entity = commands
                    .spawn((
                        Sprite {
                            color: pixel_type.as_color(),
                            custom_size: Some(Vec2::ONE),
                            ..default()
                        },
                        Pixel {
                            pixel_type,
                            ..Default::default()
                        },
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
    let mut updates: BTreeMap<(i32, i32), (Entity, (i32, i32))> = BTreeMap::new();

    let mut sorted_particules: Vec<(Entity, Mut<Transform>, Mut<Pixel>)> = query
        .iter_mut()
        .filter(|(_, _, pixel)| {
            return match pixel.velocity.y {
                0.0 => match pixel.velocity.x {
                    0.0 => false,
                    _ => true,
                },
                _ => true,
            };
        })
        .collect();
    sorted_particules.sort_by(|a, b| a.1.translation.y.partial_cmp(&b.1.translation.y).unwrap());

    for (entity, transform, pixel) in sorted_particules.iter_mut() {
        match pixel.pixel_type {
            PixelType::SAND(_) => {
                let x = (transform.translation.x).floor() as i32;
                let y = (transform.translation.y).floor() as i32;

                let new_y = {
                    let mut tmp = (y as f32 + pixel.velocity.y).round() as i32;
                    if tmp < -(WINDOW_HEIGHT as f32 / 2.0) as i32 {
                        tmp = -(WINDOW_HEIGHT as f32 / 2.0) as i32;
                    }
                    tmp
                };
                if let Some((valid_x, valid_y)) =
                    check_vertical_position(&pixel_grid.grid, x, y, new_y, &updates)
                {
                    pixel.apply_gravity();
                    updates.insert((y, x), (*entity, (valid_x, valid_y))); // y first for sorting
                } else {
                    if let Some((valid_x, valid_y)) =
                        check_sliding(&pixel_grid.grid, x, y, new_y, &updates)
                    {
                        updates.insert((y, x), (*entity, (valid_x, valid_y)));
                        pixel.velocity.y = -1.0;
                    }
                    pixel.velocity.y = -0.0;
                }
            }
            _ => {
                todo!("Handle other pixel types");
            }
        }
    }

    for (y, x) in updates.keys() {
        if let Some((entity, (new_x, new_y))) = updates.get(&(*y, *x)) {
            if let Ok((_, mut transform, _)) = query.get_mut(*entity) {
                transform.translation.y = (*new_y as f32).floor();
                transform.translation.x = (*new_x as f32).floor();
                pixel_grid.remove_pixel(*x, *y);
                pixel_grid.set_pixel(*new_x, *new_y, *entity);
            }
        }
    }
}
