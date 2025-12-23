use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundManager {
    pub current_round: u32,
    #[allow(dead_code)]
    pub round_timer: Timer, // Không còn sử dụng sau khi đơn giản hóa logic
    pub spawn_timer: Timer,
    pub enemies_to_spawn: u32,
    pub elites_to_spawn: u32,
    pub yellow_enemies_to_spawn: u32,

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
            spawn_timer: Timer::from_seconds(
                crate::configs::enemy::BASE_SPAWN_INTERVAL,
                TimerMode::Repeating,
            ),
            enemies_to_spawn: crate::configs::enemy::BASE_ENEMY_COUNT,
            elites_to_spawn: 1,
            yellow_enemies_to_spawn: 0,

            round_state: RoundState::Spawning,
        }
    }
}
