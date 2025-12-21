use crate::resources::game_state::GameState;
use crate::resources::polish::{spawn_trails, update_camera_shake};
use crate::systems::damage_text::{spawn_damage_text, update_damage_text};
use crate::utils::log_error;
use bevy::prelude::*;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_damage_text)
            .add_systems(
                Update,
                (spawn_trails, update_damage_text.pipe(log_error))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                update_camera_shake.run_if(in_state(GameState::Playing)),
            );
    }
}
