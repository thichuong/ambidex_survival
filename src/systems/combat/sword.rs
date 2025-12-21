use super::CombatInputParams;
use crate::components::player::{Hand, HandType, Player, PlayerStats};
use crate::components::weapon::{
    SwingState, SwordMode, SwordState, SwordSwing, Weapon, WeaponType,
};
use crate::configs::weapons::sword;
use crate::systems::weapon_visuals::{spawn_sword_normal_visuals, spawn_sword_shattered_visuals};
use bevy::prelude::*;
use rand::Rng;

pub fn sword_weapon_system(
    mut params: CombatInputParams,
    player_query: Query<(Entity, &PlayerStats), With<Player>>,
    mut hand_query: Query<(
        Entity,
        &GlobalTransform,
        &Hand,
        &mut SwordState,
        &mut Weapon,
    )>,
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

    let Some((player_entity, stats)) = player_query.iter().next() else {
        return;
    };

    let left_pressed = params.mouse_input.pressed(MouseButton::Left);
    let right_pressed = params.mouse_input.pressed(MouseButton::Right);
    let left_just_pressed = params.mouse_input.just_pressed(MouseButton::Left);
    let right_just_pressed = params.mouse_input.just_pressed(MouseButton::Right);

    let q_just_pressed = params.key_input.just_pressed(KeyCode::KeyQ);
    let e_just_pressed = params.key_input.just_pressed(KeyCode::KeyE);

    for (hand_entity, hand_transform, hand, mut sword_state, mut weapon_data) in &mut hand_query {
        if hand.equipped_weapon != Some(WeaponType::Sword) {
            continue;
        }

        let hand_pos = hand_transform.translation().truncate();

        let (_, is_just_pressed, skill_pressed) = match hand.side {
            HandType::Left => (left_pressed, left_just_pressed, q_just_pressed),
            HandType::Right => (right_pressed, right_just_pressed, e_just_pressed),
        };

        let now = params.time.elapsed_secs();

        // Fire logic (Swing)
        if is_just_pressed && now - weapon_data.last_shot >= weapon_data.cooldown {
            fire_sword(
                &mut params,
                hand_pos,
                cursor_pos,
                player_entity,
                sword_state.mode,
                hand_entity,
                stats.damage_multiplier,
            );
            weapon_data.last_shot = now;
        }

        // Skill logic (Mode switch)
        if skill_pressed && now - weapon_data.last_skill_use >= weapon_data.skill_cooldown {
            match sword_state.mode {
                SwordMode::Normal => sword_state.mode = SwordMode::Shattered,
                SwordMode::Shattered => sword_state.mode = SwordMode::Normal,
            }
            weapon_data.last_skill_use = now;
        }
    }
}

fn fire_sword(
    params: &mut CombatInputParams,
    spawn_pos: Vec2,
    target_pos: Vec2,
    owner: Entity,
    sword_mode: SwordMode,
    hand_entity: Entity,
    damage_multiplier: f32,
) {
    let direction = (target_pos - spawn_pos).normalize_or_zero();
    let start_angle = direction.y.atan2(direction.x);
    let mut rng = rand::thread_rng();
    // 50% chance for clockwise vs counter-clockwise
    let swing_dir: f32 = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

    match sword_mode {
        SwordMode::Normal => {
            params
                .commands
                .spawn((
                    Transform::from_translation(spawn_pos.extend(0.0)),
                    Visibility::Visible,
                    SwordSwing {
                        state: SwingState::Swinging,
                        timer: Timer::from_seconds(sword::NORMAL_TIMER, TimerMode::Once),
                        base_angle: start_angle,
                        owner_entity: owner,
                        damage: sword::NORMAL_DAMAGE * damage_multiplier,
                        range: sword::NORMAL_RANGE,
                        damage_dealt: false,
                        hand_entity,
                        swing_direction: swing_dir,
                    },
                ))
                .with_children(|parent| {
                    spawn_sword_normal_visuals(parent, &params.cached_assets);
                });
        }
        SwordMode::Shattered => {
            params
                .commands
                .spawn((
                    Transform::from_translation(spawn_pos.extend(0.0)),
                    Visibility::Visible,
                    SwordSwing {
                        state: SwingState::Swinging,
                        timer: Timer::from_seconds(sword::SHATTERED_TIMER, TimerMode::Once),
                        base_angle: start_angle,
                        owner_entity: owner,
                        damage: sword::SHATTERED_DAMAGE * damage_multiplier,
                        range: sword::SHATTERED_RANGE,
                        damage_dealt: false,
                        hand_entity,
                        swing_direction: swing_dir,
                    },
                ))
                .with_children(|parent| {
                    spawn_sword_shattered_visuals(parent, &params.cached_assets);
                });
        }
    }
}
