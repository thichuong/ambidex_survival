use crate::systems::combat::{
    cleanup_pending_despawn, collision_detection_system, damage_processing_system,
    elite_ai::elite_ai_system, enemy_death_system, gun_weapon_system, handle_player_collision,
    magic_weapon_system, manage_lifetime, projectile_effect_system, shuriken_weapon_system,
    sword_weapon_system, update_enemy_grid, update_sword_mechanics, yellow_ai::yellow_ai_system,
};
use crate::systems::enemy::{enemy_chase_player, spawn_waves};

use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(enemy_death_system)
            .add_observer(damage_processing_system)
            .add_observer(projectile_effect_system)
            .add_observer(crate::systems::combat::magic::force_logic::force_effect_observer)
            .add_systems(
                Update,
                (
                    update_enemy_grid,
                    shuriken_weapon_system,
                    sword_weapon_system,
                    gun_weapon_system,
                    magic_weapon_system,
                    manage_lifetime,
                    collision_detection_system,
                    elite_ai_system,
                    yellow_ai_system,
                )
                    .run_if(in_state(crate::resources::game_state::GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    update_sword_mechanics,
                    handle_player_collision,
                    enemy_chase_player,
                    spawn_waves,
                    cleanup_pending_despawn,
                )
                    .run_if(in_state(crate::resources::game_state::GameState::Playing)),
            );
    }
}
