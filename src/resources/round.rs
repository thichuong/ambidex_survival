use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundManager {
    pub current_round: u32,
    pub spawn_timer: Timer,
    pub enemies_to_spawn: u32,
    pub elites_to_spawn: u32,
    pub yellow_enemies_to_spawn: u32,

    pub round_state: RoundState,
    pub has_started: bool,
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
            spawn_timer: Timer::from_seconds(
                crate::configs::enemy::BASE_SPAWN_INTERVAL,
                TimerMode::Repeating,
            ),
            enemies_to_spawn: crate::configs::enemy::BASE_ENEMY_COUNT,
            elites_to_spawn: 1,
            yellow_enemies_to_spawn: 1,

            round_state: RoundState::Spawning,
            has_started: false,
        }
    }
}
