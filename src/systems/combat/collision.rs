use super::{CollisionEvent, CombatResources, DamageEvent};
use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, IgnoreGrid, UniformGrid, Velocity, check_collision};
use crate::components::player::{CombatStats, Currency, Health, Player};
use crate::components::weapon::{AoEProjectile, ExplodingProjectile, Lifetime, Projectile};
use crate::systems::object_pooling::EffectType;
use crate::systems::weapon_visuals::spawn_bolt_explosion_visuals;
use bevy::prelude::*;
use rand::Rng;

pub type ProjectileQueryItem<'a> = (
    Entity,
    &'a Projectile,
    &'a Transform,
    &'a Collider,
    Option<Mut<'a, AoEProjectile>>,
    Option<&'a IgnoreGrid>,
    &'a Visibility,
);

pub fn get_collision_candidates(
    proj_pos: Vec2,
    proj_collider: &Collider,
    ignore_grid: Option<&IgnoreGrid>,
    grid: &UniformGrid,
) -> Vec<Entity> {
    if ignore_grid.is_some() {
        let (min, max) = match proj_collider {
            Collider::Circle { radius } => (proj_pos - *radius, proj_pos + *radius),
            Collider::Rectangle {
                half_width,
                half_height,
            } => {
                let half = Vec2::new(*half_width, *half_height);
                (proj_pos - half, proj_pos + half)
            }
            Collider::Line {
                direction,
                length,
                width,
            } => {
                let end_pos = proj_pos + *direction * *length;
                let min_pos = proj_pos.min(end_pos) - *width;
                let max_pos = proj_pos.max(end_pos) + *width;
                (min_pos, max_pos)
            }
        };
        grid.query_aabb(min, max)
    } else {
        grid.query_nearby(proj_pos)
    }
}

/// Detects collisions between projectiles and enemies.
/// Emits `CollisionEvent` when a collision occurs.
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn collision_detection_system(
    mut projectile_query: Query<ProjectileQueryItem>,
    enemy_query: Query<(Entity, &Transform, &Collider), (With<Enemy>, Without<Player>)>,
    grid: Res<UniformGrid>,
    mut collision_events: MessageWriter<CollisionEvent>,
) -> Result<(), String> {
    for (
        proj_entity,
        projectile,
        projectile_transform,
        proj_collider,
        mut aoe_opt,
        ignore_grid,
        visibility,
    ) in &mut projectile_query
    {
        if *visibility == Visibility::Hidden {
            continue;
        }
        let proj_pos = projectile_transform.translation.truncate();
        let candidates = get_collision_candidates(proj_pos, proj_collider, ignore_grid, &grid);

        for enemy_entity in candidates {
            if let Ok((entity, enemy_transform, enemy_collider)) = enemy_query.get(enemy_entity) {
                let enemy_pos = enemy_transform.translation.truncate();
                if check_collision(proj_pos, proj_collider, enemy_pos, enemy_collider)
                    && projectile.owner_entity != entity
                {
                    // For AOE projectiles, avoid hitting the same enemy multiple times if already in the list
                    if let Some(ref mut aoe) = aoe_opt {
                        if aoe.damaged_entities.contains(&entity) {
                            continue;
                        }
                        aoe.damaged_entities.push(entity);
                    }

                    collision_events.write(CollisionEvent {
                        projectile: proj_entity,
                        target: entity,
                        position: enemy_pos, // Use enemy position for hit location approximation
                        direction: projectile.direction,
                    });
                }
            }
        }
    }
    Ok(())
}

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

/// Handle Despawning of dead enemies, loot, and on-death effects
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn enemy_death_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform)>,
    mut player_query: Query<&mut Currency, With<Player>>,
    res: Res<crate::resources::cached_assets::CachedAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<(), String> {
    for (entity, enemy, transform) in &mut enemy_query {
        if enemy.health <= 0.0 {
            // Give Gold
            if let Some(mut currency) = player_query.iter_mut().next() {
                currency.gold += 10;
            }

            commands.entity(entity).despawn();

            // Spawn particles
            let mut rng = rand::thread_rng();
            for _ in 0..5 {
                let dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                    .normalize_or_zero();
                commands.spawn((
                    Mesh2d(res.unit_circle.clone()),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                    Transform::from_translation(transform.translation).with_scale(Vec3::splat(3.0)),
                    Velocity {
                        linvel: dir * 100.0,
                        angvel: 0.0,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(0.5, TimerMode::Once),
                    },
                ));
            }
        }
    }
    Ok(())
}

/// Handles visual effects and projectile logic (piercing, explosions, despawning) upon collision.
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn projectile_effect_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    projectile_query: Query<(
        &Projectile,
        Option<&ExplodingProjectile>,
        Option<&AoEProjectile>,
        &Transform,
    )>,
    mut res: CombatResources,
) -> Result<(), String> {
    let mut despawn_list = Vec::new();

    for event in collision_events.read() {
        if despawn_list.contains(&event.projectile) {
            continue;
        }

        if let Ok((projectile, exploding, aoe, transform)) = projectile_query.get(event.projectile)
        {
            // Handle Explosions
            if let Some(exploding) = exploding {
                let mut rng = rand::thread_rng();
                let random_rotation =
                    Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::TAU));
                let spawn_transform = Transform::from_translation(transform.translation)
                    .with_rotation(random_rotation);
                let lifetime = Lifetime {
                    timer: Timer::from_seconds(0.1, TimerMode::Once),
                };

                let req = res.effect_pool.spawn_or_get(
                    &mut commands,
                    EffectType::BoltExplosion,
                    spawn_transform,
                    lifetime,
                );

                // Update the explosion entity components
                commands.entity(req.entity).insert((
                    Velocity::default(),
                    Projectile {
                        kind: projectile.kind,
                        damage: exploding.damage,
                        speed: 0.0,
                        direction: Vec2::ZERO,
                        owner_entity: projectile.owner_entity,
                    },
                    AoEProjectile::default(), // Reset hit list for pooled entities
                    Collider::ball(exploding.radius), // Set correct explosion size
                    IgnoreGrid,               // Reliable AOE coverage
                ));

                if req.is_new {
                    commands.entity(req.entity).with_children(|parent| {
                        spawn_bolt_explosion_visuals(parent, &res.cached_assets, exploding.radius);
                    });
                }
            }

            // Despawn non-AoE projectiles upon hit
            if aoe.is_none() {
                despawn_list.push(event.projectile);
            }
        }
    }

    // Process Despawns
    for entity in despawn_list {
        commands.entity(entity).despawn();
    }
    Ok(())
}
