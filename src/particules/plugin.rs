use super::{
    pixel_grid::PixelGrid,
    systems::{
        PixelSpawnType, change_pixel_spawn_type, reset_grid, spawn_pixel_to_cursor_position,
        update_particules,
    },
};
use bevy::prelude::*;

pub struct ParticulesPlugin;
impl Plugin for ParticulesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelGrid::default())
            .insert_resource(PixelSpawnType::default())
            .add_systems(Update, (update_particules, spawn_pixel_to_cursor_position))
            .add_systems(Update, (reset_grid, change_pixel_spawn_type));
    }
}
