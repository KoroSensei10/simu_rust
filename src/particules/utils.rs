use bevy::prelude::*;
use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Tries to find the longest valid vertical position for a pixel.
/// # Returns
/// the first valid position found, or None if no valid position is found.
pub fn check_vertical_position(
    grid: &HashMap<(i32, i32), Entity>,
    actual_x: i32,
    actual_y: i32,
    min_y: i32,
    updates: &BTreeMap<(i32, i32), (Entity, (i32, i32))>,
) -> Option<(i32, i32)> {
    let mut valid_y: Option<(i32, i32)> = None;
    for i in min_y..actual_y {
        if !grid.contains_key(&(actual_x, i)) {
            valid_y = Some((actual_x, i));
            break;
        } else if updates.contains_key(&(i, actual_x)) {
            valid_y = Some((actual_x, i));
            break;
        }
    }
    valid_y
}

pub fn check_sliding(
    grid: &HashMap<(i32, i32), Entity>,
    actual_x: i32,
    actual_y: i32,
    min_y: i32,
    updates: &BTreeMap<(i32, i32), (Entity, (i32, i32))>,
) -> Option<(i32, i32)> {
    let mut valid_x: Option<(i32, i32)> = None;
    let mut rng = rand::rng();
    let random_value: f32 = rng.random_range(0.0..1.0);
    let x_factor = { if random_value < 0.5 { 1 } else { -1 } }; // handle velocity
    let new_x_pos = actual_x + x_factor;
    let new_x_neg = actual_x - x_factor;
    for i in min_y..actual_y {
        if !grid.contains_key(&(new_x_pos, i)) {
            valid_x = Some((new_x_pos, i));
            break;
        } else if !grid.contains_key(&(new_x_neg, i)) {
            valid_x = Some((new_x_neg, i));
            break;
        } else if updates.contains_key(&(i, new_x_pos)) {
            valid_x = Some((new_x_pos, i));
            break;
        } else if updates.contains_key(&(i, new_x_neg)) {
            valid_x = Some((new_x_neg, i));
            break;
        }
    }
    valid_x
}
