use bevy::prelude::*;
use rand::Rng;

#[derive(PartialEq, Clone)]
pub enum SandColor {
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
pub enum PixelType {
    AIR,
    WATER {
        stagnation_count: i32,
        max_stagnation: i32,
    },
    SAND(SandColor),
}

impl PixelType {
    pub fn as_color(&self) -> Color {
        match self {
            PixelType::AIR => Color::NONE,
            PixelType::WATER { .. } => Color::srgb(0.0, 0.0, 1.0),
            PixelType::SAND(color) => color.as_color(),
        }
    }
}

#[derive(Component, Clone)]
pub struct Pixel {
    pub pixel_type: PixelType,
}
impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            pixel_type: PixelType::SAND(SandColor::default()),
        }
    }
}
