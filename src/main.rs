mod particules;
mod timers;
mod utils;

use bevy::{prelude::*, window::WindowResolution};

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 400;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(),));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pixel Snoup".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(timers::TimersPlugin)
        .add_plugins(particules::ParticulesPlugin)
        .add_systems(Startup, setup)
        .run();
}
