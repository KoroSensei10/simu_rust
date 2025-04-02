use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct PixelGrid {
    pub grid: HashMap<(i32, i32), Entity>,
}

impl PixelGrid {
    pub fn _get_pixel(&self, x: i32, y: i32) -> Option<Entity> {
        self.grid.get(&(x, y)).cloned()
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, entity: Entity) {
        self.grid.insert((x, y), entity);
    }

    pub fn remove_pixel(&mut self, x: i32, y: i32) -> Option<Entity> {
        self.grid.remove(&(x, y))
    }
}
