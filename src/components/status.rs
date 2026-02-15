use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct UnitStatus {
    pub is_rooted: bool,
    pub rooted_timer: Timer,
}

impl UnitStatus {
    pub fn root(&mut self, duration: f32) {
        self.is_rooted = true;
        self.rooted_timer = Timer::from_seconds(duration, TimerMode::Once);
    }
}
