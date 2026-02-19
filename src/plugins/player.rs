use crate::resources::game_state::GameState;
use crate::systems::player::{aim_player, move_player, spawn_player};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                move_player
                    .run_if(in_state(GameState::Playing))
                    .before(crate::systems::status::tick_status_system),
            )
            .add_systems(PostUpdate, aim_player.run_if(in_state(GameState::Playing)));
    }
}
