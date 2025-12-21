use crate::systems::physics::apply_velocity;
use crate::utils::log_error;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_velocity
                .pipe(log_error)
                .run_if(in_state(crate::resources::game_state::GameState::Playing)),
        );
    }
}
