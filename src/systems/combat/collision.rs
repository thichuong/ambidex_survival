use super::{CombatResources, DamageEvent};
use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, IgnoreGrid, UniformGrid, Velocity, check_collision};
use crate::components::player::Player;
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

#[allow(clippy::too_many_arguments)]
pub fn handle_projectile_hit(
    commands: &mut Commands,
    projectile: &Projectile,
    projectile_transform: &Transform,
    proj_entity: Entity,
    enemy_entity: Entity,
    enemy_pos: Vec3,
    enemy: &mut Enemy,
    is_aoe: bool,
    res: &mut CombatResources,
    damage_events: &mut MessageWriter<DamageEvent>,
    player_query: &mut Query<&mut Player>,
) -> bool {
    let mut should_despawn = false;
    enemy.health -= projectile.damage;
    damage_events.write(DamageEvent {
        damage: projectile.damage,
        position: enemy_pos.truncate(),
    });
    res.shake.add_trauma(0.1);

    if !is_aoe {
        if let Ok(exploding) = res.exploding_query.get(proj_entity) {
            let transform = Transform::from_translation(projectile_transform.translation);
            let lifetime = Lifetime {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            };

            let req = res.effect_pool.spawn_or_get(
                commands,
                EffectType::BoltExplosion,
                transform,
                lifetime,
            );

            if req.is_new {
                commands
                    .entity(req.entity)
                    .insert((
                        Velocity::default(),
                        Projectile {
                            kind: projectile.kind,
                            damage: exploding.damage,
                            speed: 0.0,
                            direction: Vec2::ZERO,
                            owner_entity: projectile.owner_entity,
                        },
                        AoEProjectile::default(),
                    ))
                    .with_children(|parent| {
                        spawn_bolt_explosion_visuals(parent, &res.cached_assets, exploding.radius);
                    });
            } else {
                let mut rng = rand::thread_rng();
                let rotation = Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::TAU));
                commands
                    .entity(req.entity)
                    .insert(transform.with_rotation(rotation));
            }
        }
        should_despawn = true;
    }

    if enemy.health <= 0.0 {
        if let Ok(mut player) = player_query.single_mut() {
            player.gold += 10;
        }
        commands.entity(enemy_entity).despawn();
        res.shake.add_trauma(0.3);

        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let dir =
                Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
            commands.spawn((
                Mesh2d(res.cached_assets.unit_circle.clone()),
                MeshMaterial2d(res.materials.add(Color::srgb(1.0, 0.0, 0.0))),
                Transform::from_translation(enemy_pos).with_scale(Vec3::splat(3.0)),
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
    should_despawn
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn resolve_damage(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut projectile_query: Query<ProjectileQueryItem>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy, &Collider), Without<Player>>,
    grid: Res<UniformGrid>,
    mut res: CombatResources,
    mut damage_events: MessageWriter<DamageEvent>,
) -> Result<(), String> {
    for (proj_entity, projectile, projectile_transform, proj_collider, mut aoe_opt, ignore_grid) in
        &mut projectile_query
    {
        let proj_pos = projectile_transform.translation.truncate();
        let is_aoe = aoe_opt.is_some();
        let mut hits: Vec<(Entity, f32, Vec3)> = Vec::new();

        let candidates = get_collision_candidates(proj_pos, proj_collider, ignore_grid, &grid);

        for enemy_entity in candidates {
            if let Ok((entity, enemy_transform, enemy, enemy_collider)) =
                enemy_query.get(enemy_entity)
            {
                let enemy_pos = enemy_transform.translation.truncate();
                if check_collision(proj_pos, proj_collider, enemy_pos, enemy_collider)
                    && projectile.owner_entity != entity
                {
                    if let Some(ref aoe) = aoe_opt
                        && aoe.damaged_entities.contains(&entity)
                    {
                        continue;
                    }
                    hits.push((entity, enemy.health, enemy_transform.translation));
                }
            }
        }

        for (enemy_entity, _, enemy_pos) in hits {
            if let Some(ref mut aoe) = aoe_opt {
                aoe.damaged_entities.push(enemy_entity);
            }

            if let Ok((_, _, mut enemy, _)) = enemy_query.get_mut(enemy_entity) {
                let should_despawn = handle_projectile_hit(
                    &mut commands,
                    projectile,
                    projectile_transform,
                    proj_entity,
                    enemy_entity,
                    enemy_pos,
                    &mut enemy,
                    is_aoe,
                    &mut res,
                    &mut damage_events,
                    &mut player_query,
                );

                if should_despawn && !is_aoe {
                    commands.entity(proj_entity).despawn();
                    break;
                }
            }
        }
    }
    Ok(())
}
