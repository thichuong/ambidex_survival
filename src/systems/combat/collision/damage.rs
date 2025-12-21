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
    mut collision_events: MessageReader<CollisionEvent>,
    projectile_query: Query<&Projectile>,
    mut enemy_query: Query<&mut Enemy>,
    mut player_query: Query<(&mut Health, &CombatStats), With<Player>>,
    mut damage_events: MessageWriter<DamageEvent>,
    mut res: ResMut<crate::resources::polish::ScreenShake>,
) -> Result<(), String> {
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

        // Re-query for safety regarding mutable access in loop
        // Manually iterating to find success, since we only have one player
        if let Some((mut health, stats)) = player_query.iter_mut().next() {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0..1.0) < stats.crit_chance {
                final_damage *= stats.crit_damage;
                is_crit = true;
            }
            if stats.lifesteal > 0.0 {
                let heal_amount = final_damage * stats.lifesteal;
                health.current = (health.current + heal_amount).min(health.max);
            }
        } else {
            // Fallback if player dead/missing, just calc crit
            // We can't access stats without query. Assume no crit/no lifesteal if player missing.
        }

        enemy.health -= final_damage;

        damage_events.write(DamageEvent {
            damage: final_damage,
            position: event.position,
            is_crit,
        });

        res.add_trauma(if is_crit { 0.2 } else { 0.1 });
    }
    Ok(())
}
