use crate::resources::game_state::GameState;
use crate::systems::player::{aim_player, move_player, spawn_player};
use crate::utils::log_error;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                move_player
                    .pipe(log_error)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                aim_player
                    .pipe(log_error)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
