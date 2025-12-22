use crate::systems::combat::{
    CollisionEvent, DamageEvent, EnemyDeathEvent, cleanup_pending_despawn,
    collision_detection_system, damage_processing_system, enemy_death_system, gun_weapon_system,
    handle_player_collision, magic_weapon_system, manage_lifetime, projectile_effect_system,
    shuriken_weapon_system, sword_weapon_system, update_enemy_grid, update_sword_mechanics,
};
use crate::systems::enemy::{enemy_chase_player, spawn_waves};

use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
            .add_message::<CollisionEvent>()
            .add_message::<EnemyDeathEvent>()
            .add_observer(enemy_death_system)
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
                    damage_processing_system,
                )
                    .run_if(in_state(crate::resources::game_state::GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    projectile_effect_system,
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
