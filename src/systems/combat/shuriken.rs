use super::CombatInputParams;
use crate::components::physics::{Collider, Velocity};
use crate::components::player::{Hand, HandType, Player, PlayerStats};
use crate::components::weapon::{Lifetime, Projectile, Weapon, WeaponType};
use crate::configs::weapons::shuriken;
use crate::systems::weapon_visuals::spawn_shuriken_visuals;
use bevy::prelude::*;

#[allow(clippy::too_many_lines)]
pub fn shuriken_weapon_system(
    mut params: CombatInputParams,
    mut player_query: Query<(Entity, &mut Transform, &PlayerStats), With<Player>>,
    mut hand_query: Query<(Entity, &GlobalTransform, &Hand, &mut Weapon)>,
) -> Result<(), String> {
    let (camera, camera_transform) = params
        .camera_query
        .single()
        .map_err(|e| format!("Camera not found: {e:?}"))?;
    let window = params
        .window_query
        .single()
        .map_err(|e| format!("Window not found: {e:?}"))?;

    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
        .ok_or_else(|| "Cursor position not available".to_string())?;

    let (player_entity, mut player_transform, stats) = player_query
        .single_mut()
        .map_err(|e| format!("Player not found: {e:?}"))?;

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
                hand_pos,
                cursor_pos,
                player_entity,
                stats.damage_multiplier,
            );
            weapon_data.last_shot = now;
        }

        // Skill logic (Teleport)
        if skill_pressed && now - weapon_data.last_skill_use >= weapon_data.skill_cooldown
            && perform_shuriken_skill(
                &mut params,
                cursor_pos,
                player_entity,
                &mut player_transform,
            ) {
                weapon_data.last_skill_use = now;
            }
    }

    Ok(())
}

fn fire_shuriken(
    params: &mut CombatInputParams,
    spawn_pos: Vec2,
    target_pos: Vec2,
    owner: Entity,
    damage_multiplier: f32,
) {
    let direction = (target_pos - spawn_pos).normalize_or_zero();

    let mut shurikens: Vec<(Entity, f32)> = params
        .projectile_query
        .iter()
        .filter(|(_, _, p, _)| p.kind == WeaponType::Shuriken && p.owner_entity == owner)
        .map(|(e, _, _, l)| (e, l.timer.remaining_secs()))
        .collect();

    if shurikens.len() >= shuriken::MAX_COUNT {
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
            Transform::from_translation(spawn_pos.extend(0.0)),
            Visibility::Visible,
            Collider::ball(5.0),
            Velocity {
                linvel: direction * shuriken::SPEED,
                angvel: 15.0,
            },
            Projectile {
                kind: WeaponType::Shuriken,
                damage: shuriken::DAMAGE * damage_multiplier,
                speed: shuriken::SPEED,
                direction,
                owner_entity: owner,
            },
            Lifetime {
                timer: Timer::from_seconds(shuriken::LIFETIME, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            spawn_shuriken_visuals(parent, &params.cached_assets);
        });
}

fn perform_shuriken_skill(
    params: &mut CombatInputParams,
    cursor_pos: Vec2,
    player_entity: Entity,
    player_transform: &mut Transform,
) -> bool {
    let mut closest_proj: Option<(Entity, Vec3)> = None;
    let mut min_dist_sq = f32::MAX;

    for (entity, proj_tf, proj, _) in params.projectile_query.iter() {
        if proj.kind == WeaponType::Shuriken && proj.owner_entity == player_entity {
            let translation = proj_tf.translation();
            let dist_sq = translation.truncate().distance_squared(cursor_pos);
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                closest_proj = Some((entity, translation));
            }
        }
    }

    if let Some((entity, location)) = closest_proj {
        params.commands.spawn((
            Mesh2d(params.cached_assets.unit_circle.clone()),
            MeshMaterial2d(params.cached_assets.mat_cyan_50.clone()),
            Transform::from_translation(player_transform.translation).with_scale(Vec3::splat(15.0)),
            Lifetime {
                timer: Timer::from_seconds(0.2, TimerMode::Once),
            },
        ));
        player_transform.translation = location;
        params.commands.spawn((
            Mesh2d(params.cached_assets.unit_circle.clone()),
            MeshMaterial2d(params.cached_assets.mat_cyan_50.clone()),
            Transform::from_translation(location).with_scale(Vec3::splat(15.0)),
            Lifetime {
                timer: Timer::from_seconds(0.2, TimerMode::Once),
            },
        ));
        params.commands.entity(entity).despawn();
        true
    } else {
        false
    }
}
