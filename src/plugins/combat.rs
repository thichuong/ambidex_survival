use crate::systems::combat::{
    CollisionEvent, DamageEvent, cleanup_pending_despawn, collision_detection_system,
    damage_processing_system, enemy_death_system, gun_weapon_system, handle_player_collision,
    magic_weapon_system, manage_lifetime, projectile_effect_system, shuriken_weapon_system,
    sword_weapon_system, update_enemy_grid, update_sword_mechanics,
};
use crate::systems::enemy::{enemy_chase_player, spawn_waves};
use crate::utils::log_error;
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
            .add_message::<CollisionEvent>()
            .add_observer(enemy_death_system)
            .add_systems(
                Update,
                (
                    update_enemy_grid.pipe(log_error),
                    shuriken_weapon_system.pipe(log_error),
                    sword_weapon_system.pipe(log_error),
                    gun_weapon_system.pipe(log_error),
                    magic_weapon_system.pipe(log_error),
                    manage_lifetime.pipe(log_error),
                    collision_detection_system.pipe(log_error),
                    damage_processing_system.pipe(log_error),
                    projectile_effect_system,
                    update_sword_mechanics.pipe(log_error),
                    handle_player_collision.pipe(log_error),
                    enemy_chase_player.pipe(log_error),
                    spawn_waves.pipe(log_error),
                    cleanup_pending_despawn,
                )
                    .run_if(in_state(crate::resources::game_state::GameState::Playing)),
            );
    }
}
