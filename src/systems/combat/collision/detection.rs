//! Collision detection between projectiles and enemies

use super::ProjectileQueryItem;
use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, IgnoreGrid, UniformGrid, check_collision};
use crate::components::player::Player;
use crate::systems::combat::{CollisionEvent, PendingDespawn};
use bevy::prelude::*;

/// Get collision candidates from spatial grid based on projectile position and collider
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
/// Marks non-AoE projectiles with `PendingDespawn` immediately to prevent double-damage.
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn collision_detection_system(
    mut commands: Commands,
    mut projectile_query: Query<ProjectileQueryItem>,
    enemy_query: Query<(Entity, &Transform, &Collider), (With<Enemy>, Without<Player>)>,
    player_query: Single<(Entity, &Transform, &Collider), With<Player>>,
    grid: Res<UniformGrid>,
    mut collision_events: MessageWriter<CollisionEvent>,
) {
    let (player_entity, player_transform, player_collider) = *player_query;
    let player_pos = player_transform.translation.truncate();

    for (
        proj_entity,
        projectile,
        projectile_transform,
        proj_collider,
        mut aoe_opt,
        ignore_grid,
        visibility,
        pending_despawn,
    ) in &mut projectile_query
    {
        // Skip hidden projectiles
        if *visibility == Visibility::Hidden {
            continue;
        }
        // Skip projectiles already marked for despawn (prevents double-damage)
        if pending_despawn.is_some() {
            continue;
        }

        let proj_pos = projectile_transform.translation.truncate();

        // 1. Check against Enemies (via Grid)
        let candidates = get_collision_candidates(proj_pos, proj_collider, ignore_grid, &grid);
        let mut hit_anything = false;

        for enemy_entity in candidates {
            if let Ok((entity, enemy_transform, enemy_collider)) = enemy_query.get(enemy_entity) {
                let enemy_pos = enemy_transform.translation.truncate();
                if check_collision(proj_pos, proj_collider, enemy_pos, enemy_collider)
                    && projectile.owner_entity != entity
                {
                    // For AOE projectiles, track damaged entities to avoid multi-hit per enemy
                    if let Some(ref mut aoe) = aoe_opt {
                        if aoe.damaged_entities.contains(&entity) {
                            continue;
                        }
                        aoe.damaged_entities.push(entity);
                    }

                    collision_events.write(CollisionEvent {
                        projectile: proj_entity,
                        target: entity,
                        position: enemy_pos,
                        direction: projectile.direction,
                    });

                    hit_anything = true;

                    // Mark non-AoE projectiles for despawn immediately after first hit
                    if aoe_opt.is_none() {
                        commands.entity(proj_entity).insert(PendingDespawn);
                        break;
                    }
                }
            }
        }

        if hit_anything && aoe_opt.is_none() {
            continue;
        }

        // 2. Check against Player
        if projectile.owner_entity != player_entity
            && check_collision(proj_pos, proj_collider, player_pos, player_collider) {
                if let Some(ref mut aoe) = aoe_opt {
                    if aoe.damaged_entities.contains(&player_entity) {
                        continue;
                    }
                    aoe.damaged_entities.push(player_entity);
                }

                collision_events.write(CollisionEvent {
                    projectile: proj_entity,
                    target: player_entity,
                    position: player_pos,
                    direction: projectile.direction,
                });

                if aoe_opt.is_none() {
                    commands.entity(proj_entity).insert(PendingDespawn);
                }
            }
    }
}
