use bevy::prelude::*;

const SPAWN_TIMER: f32 = 0.2;

pub struct TimersPlugin;

impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CastTimer::default())
            .add_systems(Update, inc_timer_system);
    }
}

#[derive(Resource)]
pub struct CastTimer(pub Timer);
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

fn inc_timer_system(time: Res<Time>, mut timer: ResMut<CastTimer>) {
    timer.0.tick(time.delta());
}
