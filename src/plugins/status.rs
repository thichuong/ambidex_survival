use bevy::prelude::*;
use crate::resources::game_state::GameState;
use crate::systems::status::tick_status_system;

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_status_system.run_if(in_state(GameState::Playing)));
    }
}
