use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundManager {
    pub current_round: u32,
    #[allow(dead_code)]
    pub round_timer: Timer, // Không còn sử dụng sau khi đơn giản hóa logic
    pub spawn_timer: Timer,
    pub enemies_to_spawn: u32,

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

            round_state: RoundState::Spawning,
        }
    }
}
