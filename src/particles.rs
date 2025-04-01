
use bevy::prelude::*;

enum PixelType {
    AIR,
    WATER,
    SAND,
}

enum PixelColor {
    SAND,
    WATER,
    AIR,
}

pub struct Pixel {
    pixel_type: PixelType,
    color: PixelColor,
    velocity: Vec2
}