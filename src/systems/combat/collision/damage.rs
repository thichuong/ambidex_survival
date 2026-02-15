//! Damage processing when collisions occur

use crate::components::enemy::Enemy;
use crate::components::player::{CombatStats, Health, Player};
use crate::components::weapon::{ForcePull, ForcePush, Projectile};
use crate::resources::game_state::GameState;
use crate::systems::combat::{CollisionEvent, DamageEvent};
use bevy::prelude::*;
use rand::Rng;

/// Processes collision events to apply damage to enemies.
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn damage_processing_system(
    trigger: On<CollisionEvent>,
    mut commands: Commands,
    projectile_query: Query<(
        &Projectile,
        &Transform,
        Option<&ForcePush>,
        Option<&ForcePull>,
    )>,
    mut enemy_query: Query<(&mut Enemy, &Transform)>,
    player: Single<(Entity, &mut Health, &CombatStats), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let event = trigger.event();
    let (player_entity, mut player_health, player_stats) = player.into_inner();

    // Retrieve projectile data
    let Ok(projectile) = projectile_query.get(event.projectile) else {
        return; // Projectile might have been despawned
    };

    if event.target == player_entity {
        // Apply damage to player
        if player_health.invulnerability_timer.is_finished() {
            let mut final_damage = projectile.0.damage;
            let mut is_crit = false;

            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0..1.0) < projectile.0.crit_chance {
                final_damage *= projectile.0.crit_damage;
                is_crit = true;
            }

            player_health.current -= final_damage;
            player_health.invulnerability_timer.reset();

            commands.trigger(DamageEvent {
                entity: player_entity,
                damage: final_damage,
                crit: is_crit,
            });

            if player_health.current <= 0.0 {
                player_health.current = 0.0;
                next_state.set(GameState::GameOver);
            }
        }
        return;
    }

    // Retrieve enemy data
    let Ok((mut enemy, enemy_transform)) = enemy_query.get_mut(event.target) else {
        return; // Enemy might have been despawned
    };

    let (projectile, proj_transform, push, pull) = projectile_query.get(event.projectile).unwrap();

    let mut final_damage = projectile.damage;

    // Distance-based damage for Force spells
    if push.is_some() || pull.is_some() {
        let dist = proj_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());
        let radius = crate::configs::spells::force::RADIUS;
        let t = (dist / radius).clamp(0.0, 1.0);

        let bonus = if push.is_some() {
            // Push: damage higher when close (t near 0)
            (1.0 - t) * crate::configs::spells::force::DAMAGE_BONUS_MAX
        } else {
            // Pull: damage higher when far (t near 1)
            t * crate::configs::spells::force::DAMAGE_BONUS_MAX
        };
        final_damage += bonus * (projectile.damage / crate::configs::spells::force::DAMAGE_BASE);
    }

    let mut is_crit = false;

    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < projectile.crit_chance {
        final_damage *= projectile.crit_damage;
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
