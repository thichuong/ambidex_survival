use super::{CombatContext, CombatInputParams};
use crate::components::physics::{Collider, Velocity};
use crate::components::player::{CombatStats, Hand, HandType, Player, PlayerStats, Progression};
use crate::components::weapon::{Faction, Lifetime, Projectile, Weapon, WeaponType};
use crate::configs::weapons::shuriken;
use crate::visuals::world::spawn_shuriken_visuals;
use bevy::prelude::*;

#[allow(clippy::too_many_lines)]
pub fn shuriken_weapon_system(
    mut params: CombatInputParams,
    mut player: Single<
        (
            Entity,
            &mut Transform,
            &PlayerStats,
            &CombatStats,
            &Progression,
        ),
        With<Player>,
    >,
    mut hand_query: Query<(Entity, &GlobalTransform, &Hand, &mut Weapon)>,
) {
    let (camera, camera_transform) = *params.camera;
    let window = *params.window;

    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());

    let Some(cursor_pos) = cursor_pos else {
        return;
    };

    let player_entity = player.0;
    let stats = player.2;
    let combat_stats = player.3;
    let progression = player.4;
    let player_transform = &mut *player.1;

    let left_pressed = params.mouse_input.pressed(MouseButton::Left);
    let right_pressed = params.mouse_input.pressed(MouseButton::Right);
    let left_just_pressed = params.mouse_input.just_pressed(MouseButton::Left);
    let right_just_pressed = params.mouse_input.just_pressed(MouseButton::Right);

    let q_just_pressed = params.key_input.just_pressed(KeyCode::KeyQ);
    let e_just_pressed = params.key_input.just_pressed(KeyCode::KeyE);

    for (_, hand_transform, hand, mut weapon_data) in &mut hand_query {
        if hand.equipped_weapon != Some(WeaponType::Shuriken) {
            continue;
        }

        let hand_pos = hand_transform.translation().truncate();

        let (_, is_just_pressed, skill_pressed) = match hand.side {
            HandType::Left => (left_pressed, left_just_pressed, q_just_pressed),
            HandType::Right => (right_pressed, right_just_pressed, e_just_pressed),
        };

        let now = params.time.elapsed_secs();

        // Fire logic
        if is_just_pressed && now - weapon_data.last_shot >= weapon_data.cooldown {
            fire_shuriken(
                &mut params,
                &CombatContext {
                    owner_entity: player_entity,
                    transform: &mut *player_transform,
                    cursor_pos,
                    spawn_pos: hand_pos,
                    damage_multiplier: stats.damage_multiplier,
                    combat_stats,
                    progression,
                },
                shuriken::MAX_COUNT,
                Faction::Player,
            );
            weapon_data.last_shot = now;
        }

        // Skill logic (Teleport)
        if skill_pressed
            && now - weapon_data.last_skill_use >= weapon_data.skill_cooldown
            && perform_shuriken_skill(
                &mut params,
                &mut CombatContext {
                    owner_entity: player_entity,
                    transform: &mut *player_transform,
                    cursor_pos,
                    spawn_pos: hand_pos,
                    damage_multiplier: stats.damage_multiplier,
                    combat_stats,
                    progression,
                },
            )
        {
            weapon_data.last_skill_use = now;
        }
    }
}

pub fn fire_shuriken(
    params: &mut CombatInputParams,
    ctx: &CombatContext,
    max_count: usize,
    faction: Faction,
) {
    let direction = (ctx.cursor_pos - ctx.spawn_pos).normalize_or_zero();

    let mut shurikens: Vec<(Entity, f32)> = params
        .projectile_query
        .iter()
        .filter(|(_, _, p, _)| p.kind == WeaponType::Shuriken && p.owner_entity == ctx.owner_entity)
        .map(|(e, _, _, l)| (e, l.timer.remaining_secs()))
        .collect();

    if shurikens.len() >= max_count {
        shurikens.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        if let Some((oldest_entity, _)) = shurikens.first()
            && let Ok(mut e) = params.commands.get_entity(*oldest_entity)
        {
            e.despawn();
        }
    }

    params
        .commands
        .spawn((
            Transform::from_translation(ctx.spawn_pos.extend(0.0)),
            Visibility::Visible,
            Collider::ball(shuriken::COLLIDER_RADIUS),
            Velocity {
                linvel: direction * shuriken::SPEED,
                angvel: shuriken::ROTATION_SPEED,
            },
            Projectile {
                kind: WeaponType::Shuriken,
                damage: shuriken::DAMAGE * ctx.damage_multiplier,
                speed: shuriken::SPEED,
                direction,
                owner_entity: ctx.owner_entity,
                is_aoe: false,
                faction,
                crit_chance: ctx.combat_stats.crit_chance,
                crit_damage: ctx.combat_stats.crit_damage,
            },
            Lifetime {
                timer: Timer::from_seconds(shuriken::LIFETIME, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            spawn_shuriken_visuals(parent, &params.cached_assets);
        });
}

fn perform_shuriken_skill(params: &mut CombatInputParams, ctx: &mut CombatContext) -> bool {
    let mut closest_proj: Option<(Entity, Vec3)> = None;
    let mut min_dist_sq = f32::MAX;

    for (entity, proj_tf, proj, _) in params.projectile_query.iter() {
        if proj.kind == WeaponType::Shuriken && proj.owner_entity == ctx.owner_entity {
            let translation = proj_tf.translation();
            let dist_sq = translation.truncate().distance_squared(ctx.cursor_pos);
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                closest_proj = Some((entity, translation));
            }
        }
    }

    if let Some((entity, location)) = closest_proj {
        let shuriken_location = location.truncate();
        params.commands.spawn((
            Mesh2d(params.cached_assets.unit_circle.clone()),
            MeshMaterial2d(params.cached_assets.mat_cyan_50.clone()),
            Transform::from_translation(ctx.transform.translation)
                .with_scale(Vec3::splat(shuriken::TELEPORT_VISUAL_SCALE)),
            Lifetime {
                timer: Timer::from_seconds(shuriken::TELEPORT_VISUAL_LIFETIME, TimerMode::Once),
            },
        ));
        ctx.transform.translation = shuriken_location.extend(0.0);
        params.commands.spawn((
            Mesh2d(params.cached_assets.unit_circle.clone()),
            MeshMaterial2d(params.cached_assets.mat_cyan_50.clone()),
            Transform::from_translation(location)
                .with_scale(Vec3::splat(shuriken::TELEPORT_VISUAL_SCALE)),
            Lifetime {
                timer: Timer::from_seconds(shuriken::TELEPORT_VISUAL_LIFETIME, TimerMode::Once),
            },
        ));
        params.commands.entity(entity).despawn();
        true
    } else {
        false
    }
}
