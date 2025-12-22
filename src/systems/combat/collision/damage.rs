//! Damage processing when collisions occur

use crate::components::enemy::Enemy;
use crate::components::player::{CombatStats, Health, Player};
use crate::components::weapon::Projectile;
use crate::systems::combat::{CollisionEvent, DamageEvent};
use bevy::prelude::*;
use rand::Rng;

/// Processes collision events to apply damage to enemies.
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn damage_processing_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    projectile_query: Query<&Projectile>,
    mut enemy_query: Query<&mut Enemy>,
    player: Single<(&mut Health, &CombatStats), With<Player>>,
) {
    let (mut player_health, player_stats) = player.into_inner();
    for event in collision_events.read() {
        // Retrieve projectile data
        let Ok(projectile) = projectile_query.get(event.projectile) else {
            continue; // Projectile might have been despawned
        };

        // Retrieve enemy data
        let Ok(mut enemy) = enemy_query.get_mut(event.target) else {
            continue; // Enemy might have been despawned
        };

        let mut final_damage = projectile.damage;
        let mut is_crit = false;

        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < player_stats.crit_chance {
            final_damage *= player_stats.crit_damage;
            is_crit = true;
        }
        if player_stats.lifesteal > 0.0 {
            // AOE projectiles have 50% reduced lifesteal
            let lifesteal_multiplier = if projectile.is_aoe { 0.5 } else { 1.0 };
            let heal_amount = final_damage * player_stats.lifesteal * lifesteal_multiplier;
            player_health.current = (player_health.current + heal_amount).min(player_health.max);
        }

        enemy.health -= final_damage;
        commands.trigger(DamageEvent {
            entity: event.target,
            damage: final_damage,
            crit: is_crit,
        });

        if enemy.health <= 0.0 {
            commands.trigger(crate::systems::combat::EnemyDeathEvent {
                entity: event.target,
                position: event.position,
            });
        }
    }
}
