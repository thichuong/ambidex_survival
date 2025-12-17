use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundManager {
    pub current_round: u32,
    pub round_timer: Timer, // Time between rounds or wave spawning? Let's use it for spawn_timer.
    pub spawn_timer: Timer,
    pub enemies_to_spawn: u32,
    pub enemies_killed_this_round: u32,
    pub round_state: RoundState,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RoundState {
    #[default]
    Spawning, // Active round, spawning enemies
    Fighting, // Spawning finished, waiting for clear
    Shop,     // Round cleared, shop open
}

impl Default for RoundManager {
    fn default() -> Self {
        Self {
            current_round: 1,
            round_timer: Timer::from_seconds(5.0, TimerMode::Once), // Break time
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            enemies_to_spawn: 10,
            enemies_killed_this_round: 0,
            round_state: RoundState::Spawning,
        }
    }
}
