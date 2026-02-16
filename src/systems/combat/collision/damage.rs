//! Damage processing when collisions occur

use crate::components::enemy::Enemy;
use crate::components::player::{CombatStats, Health, Player};
use crate::components::weapon::Projectile;
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
        Option<&crate::components::weapon::DistanceDamageBonus>,
    )>,
    mut enemy_query: Query<(&mut Enemy, &Transform)>,
    player: Single<(Entity, &mut Health, &CombatStats), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let event = trigger.event();
    let (player_entity, mut player_health, player_stats) = player.into_inner();

    // Retrieve projectile data
    let Ok(projectile_data) = projectile_query.get(event.projectile) else {
        return; // Projectile might have been despawned
    };
    let projectile = projectile_data.0;

    if event.target == player_entity {
        // Apply damage to player
        if player_health.invulnerability_timer.is_finished() {
            let mut final_damage = projectile.damage;
            let mut is_crit = false;

            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0..1.0) < projectile.crit_chance {
                final_damage *= projectile.crit_damage;
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

    let (projectile, proj_transform, distance_bonus) = projectile_data;

    let mut final_damage = projectile.damage;

    // Distance-based damage for Force spells (or any other)
    if let Some(bonus) = distance_bonus {
        let dist = proj_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());

        // optimal_distance: 0.0 means closer is better (Push), 1.0 means farther is better (Pull)
        // distance factor t: 0.0 at dist=0, 1.0 at dist=radius
        let t = (dist / bonus.radius).clamp(0.0, 1.0);

        let multiplier = if bonus.optimal_distance < 0.5 {
            // Close range bonus (Push behavior)
            (1.0 - t).max(0.0)
        } else {
            // Long range bonus (Pull behavior)
            t
        };

        final_damage += multiplier * bonus.max_bonus;
    }

    let mut is_crit = false;

    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < projectile.crit_chance {
        final_damage *= projectile.crit_damage;
        is_crit = true;
    }
    if player_stats.lifesteal > 0.0 {
        let aoe_penalty = if projectile.is_aoe { 0.5 } else { 1.0 };
        let heal_amount =
            final_damage * player_stats.lifesteal * projectile.lifesteal_efficiency * aoe_penalty;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::weapon::{DistanceDamageBonus, Faction, Projectile, WeaponType};

    #[test]
    fn test_distance_damage_bonus_push() {
        let mut app = App::new();
        app.add_message::<CollisionEvent>();
        app.add_message::<DamageEvent>();
        app.add_observer(damage_processing_system);
        app.init_resource::<NextState<GameState>>();

        // Setup Player
        let player = app
            .world_mut()
            .spawn((
                Player,
                Health::default(),
                CombatStats::default(),
                Transform::default(),
            ))
            .id();

        // Setup Enemy at distance 10.0
        let enemy = app
            .world_mut()
            .spawn((
                Enemy {
                    health: 100.0,
                    ..default()
                },
                Transform::from_xyz(10.0, 0.0, 0.0),
            ))
            .id();

        // Setup Projectile at origin with Bonus (Push behavior: close = more damage)
        // Radius 100. Distance 10. t = 0.1. Multiplier = 0.9.
        // Base damage 10. Max bonus 10.
        // Expected damage = 10 + 0.9 * 10 = 19.
        let projectile = app
            .world_mut()
            .spawn((
                Projectile {
                    kind: WeaponType::Magic,
                    damage: 10.0,
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: player,
                    is_aoe: false,
                    faction: Faction::Player,
                    crit_chance: 0.0,
                    crit_damage: 2.0,
                    lifesteal_efficiency: 1.0,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                DistanceDamageBonus {
                    max_bonus: 10.0,
                    optimal_distance: 0.0, // Push
                    radius: 100.0,
                },
            ))
            .id();

        // Trigger Event
        app.world_mut().trigger(CollisionEvent {
            projectile,
            target: enemy,
            position: Vec2::ZERO,
        });

        // Check Enemy Health
        let enemy_comp = app.world().get::<Enemy>(enemy).unwrap();
        // 100 - 19 = 81
        assert!(
            (enemy_comp.health - 81.0).abs() < 0.001,
            "Health should be 81.0, got {}",
            enemy_comp.health
        );
    }

    #[test]
    fn test_distance_damage_bonus_pull() {
        let mut app = App::new();
        app.add_message::<CollisionEvent>();
        app.add_message::<DamageEvent>();
        app.add_observer(damage_processing_system);
        app.init_resource::<NextState<GameState>>();

        // Setup Player
        let player = app
            .world_mut()
            .spawn((
                Player,
                Health::default(),
                CombatStats::default(),
                Transform::default(),
            ))
            .id();

        // Setup Enemy at distance 90.0
        let enemy = app
            .world_mut()
            .spawn((
                Enemy {
                    health: 100.0,
                    ..default()
                },
                Transform::from_xyz(90.0, 0.0, 0.0),
            ))
            .id();

        // Setup Projectile at origin with Bonus (Pull behavior: far = more damage)
        // Radius 100. Distance 90. t = 0.9. Multiplier = 0.9.
        // Base damage 10. Max bonus 10.
        // Expected damage = 10 + 0.9 * 10 = 19.
        let projectile = app
            .world_mut()
            .spawn((
                Projectile {
                    kind: WeaponType::Magic,
                    damage: 10.0,
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: player,
                    is_aoe: false,
                    faction: Faction::Player,
                    crit_chance: 0.0,
                    crit_damage: 2.0,
                    lifesteal_efficiency: 1.0,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                DistanceDamageBonus {
                    max_bonus: 10.0,
                    optimal_distance: 1.0, // Pull
                    radius: 100.0,
                },
            ))
            .id();

        // Trigger Event
        app.world_mut().trigger(CollisionEvent {
            projectile,
            target: enemy,
            position: Vec2::ZERO,
        });

        // Check Enemy Health
        let enemy_comp = app.world().get::<Enemy>(enemy).unwrap();
        // 100 - 19 = 81
        assert!(
            (enemy_comp.health - 81.0).abs() < 0.001,
            "Health should be 81.0, got {}",
            enemy_comp.health
        );
    }
}
