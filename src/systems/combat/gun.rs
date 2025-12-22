use super::CombatInputParams;
use crate::components::physics::{Collider, Velocity};
use crate::components::player::{Hand, HandType, Player, PlayerStats};
use crate::components::weapon::{GunMode, GunState, Lifetime, Projectile, Weapon, WeaponType};
use crate::configs::weapons::gun;
use crate::systems::weapon_visuals::spawn_gun_bullet_visuals;
use bevy::prelude::*;
use rand::Rng;

#[allow(clippy::needless_pass_by_value)]
pub fn gun_weapon_system(
    mut params: CombatInputParams,
    player: Single<(Entity, &PlayerStats), With<Player>>,
    mut hand_query: Query<(Entity, &GlobalTransform, &Hand, &mut GunState, &mut Weapon)>,
) {
    let (camera, camera_transform) = *params.camera;
    let window = *params.window;

    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.get_point(0.0).truncate());

    let Some(cursor_pos) = cursor_pos else {
        return;
    };

    let (player_entity, stats) = *player;

    let left_pressed = params.mouse_input.pressed(MouseButton::Left);
    let right_pressed = params.mouse_input.pressed(MouseButton::Right);
    let left_just_pressed = params.mouse_input.just_pressed(MouseButton::Left);
    let right_just_pressed = params.mouse_input.just_pressed(MouseButton::Right);

    let q_just_pressed = params.key_input.just_pressed(KeyCode::KeyQ);
    let e_just_pressed = params.key_input.just_pressed(KeyCode::KeyE);

    for (_, hand_transform, hand, mut gun_state, mut weapon_data) in &mut hand_query {
        if hand.equipped_weapon != Some(WeaponType::Gun) {
            continue;
        }

        let hand_pos = hand_transform.translation().truncate();

        let (is_pressed, is_just_pressed, skill_pressed) = match hand.side {
            HandType::Left => (left_pressed, left_just_pressed, q_just_pressed),
            HandType::Right => (right_pressed, right_just_pressed, e_just_pressed),
        };

        let now = params.time.elapsed_secs();

        // Fire logic
        let cooldown = match gun_state.mode {
            GunMode::Rapid => gun::RAPID_COOLDOWN,
            _ => gun::STANDARD_COOLDOWN,
        };

        let should_fire = if gun_state.mode == GunMode::Rapid {
            is_pressed && now - weapon_data.last_shot >= cooldown
        } else {
            is_just_pressed && now - weapon_data.last_shot >= cooldown
        };

        if should_fire {
            fire_gun(
                &mut params,
                hand_pos,
                cursor_pos,
                player_entity,
                gun_state.mode,
                stats.damage_multiplier,
            );
            weapon_data.last_shot = now;
        }

        // Skill logic (Mode switch)
        if skill_pressed && now - weapon_data.last_skill_use >= gun::MODE_SWITCH_COOLDOWN {
            match gun_state.mode {
                GunMode::Single => gun_state.mode = GunMode::Shotgun,
                GunMode::Shotgun => gun_state.mode = GunMode::Rapid,
                GunMode::Rapid => gun_state.mode = GunMode::Single,
            }
            weapon_data.last_skill_use = now;
        }
    }
}

fn fire_gun(
    params: &mut CombatInputParams,
    spawn_pos: Vec2,
    target_pos: Vec2,
    owner: Entity,
    gun_mode: GunMode,
    damage_multiplier: f32,
) {
    let direction = (target_pos - spawn_pos).normalize_or_zero();
    let base_angle = direction.y.atan2(direction.x);
    let mut projectiles = Vec::new();

    match gun_mode {
        GunMode::Single => projectiles.push((0.0, gun::SINGLE_DAMAGE, gun::SINGLE_SPEED)),
        GunMode::Shotgun => {
            for &s in gun::SHOTGUN_SPREAD {
                projectiles.push((s, gun::SHOTGUN_DAMAGE, gun::SHOTGUN_SPEED));
            }
        }
        GunMode::Rapid => {
            let mut rng = rand::thread_rng();
            let jitter = rng.gen_range(-gun::RAPID_SPREAD_JITTER..gun::RAPID_SPREAD_JITTER);
            projectiles.push((jitter, gun::RAPID_DAMAGE, gun::RAPID_SPEED));
        }
    }

    for (offset, damage, speed) in projectiles {
        let angle = base_angle + offset;
        let dir = Vec2::new(angle.cos(), angle.sin());

        params
            .commands
            .spawn((
                Transform::from_translation(spawn_pos.extend(0.0))
                    .with_rotation(Quat::from_rotation_z(angle)),
                Visibility::Visible,
                Collider::cuboid(gun::BULLET_SIZE.0, gun::BULLET_SIZE.1),
                Velocity {
                    linvel: dir * speed,
                    angvel: 0.0,
                },
                Projectile {
                    kind: WeaponType::Gun,
                    damage: damage * damage_multiplier,
                    speed,
                    direction: dir,
                    owner_entity: owner,
                },
                Lifetime {
                    timer: Timer::from_seconds(gun::BULLET_LIFETIME, TimerMode::Once),
                },
            ))
            .with_children(|parent| {
                spawn_gun_bullet_visuals(parent, &params.cached_assets);
            });
    }
}
