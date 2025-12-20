use super::CombatInputParams;
use crate::components::physics::{Collider, IgnoreGrid, Velocity};
use crate::components::player::{Hand, HandType, Player};
use crate::components::weapon::{
    ActiveSpellSlot, AoEProjectile, ExplodingProjectile, GunMode, GunState, Lifetime, MagicLoadout,
    Projectile, SpellType, SwordMode, SwordState, Weapon, WeaponType,
};
use crate::configs::spells::{energy_bolt, global, laser, nova};
use crate::configs::weapons::{gun, shuriken, sword};
use crate::systems::weapon_visuals::{
    spawn_energy_bolt_visuals, spawn_global_visuals, spawn_gun_bullet_visuals, spawn_laser_visuals,
    spawn_nova_visuals, spawn_shuriken_visuals, spawn_sword_normal_visuals,
    spawn_sword_shattered_visuals,
};
use bevy::prelude::*;
use rand::Rng;

#[allow(clippy::too_many_lines)]
pub fn handle_combat_input(
    mut params: CombatInputParams,
    mut player_query: Query<(Entity, &mut Transform, &Player), With<Player>>,
    mut hand_query: Query<(
        Entity,
        &GlobalTransform,
        &Hand,
        &mut MagicLoadout,
        &mut SwordState,
        &mut GunState,
        &mut Weapon,
    )>,
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

    let (player_entity, mut player_transform, player) = player_query
        .single_mut()
        .map_err(|e| format!("Player not found: {e:?}"))?;

    let left_pressed = params.mouse_input.pressed(MouseButton::Left);
    let right_pressed = params.mouse_input.pressed(MouseButton::Right);
    let left_just_pressed = params.mouse_input.just_pressed(MouseButton::Left);
    let right_just_pressed = params.mouse_input.just_pressed(MouseButton::Right);

    let q_just_pressed = params.key_input.just_pressed(KeyCode::KeyQ);
    let e_just_pressed = params.key_input.just_pressed(KeyCode::KeyE);

    for (
        hand_entity,
        hand_transform,
        hand,
        mut magic_loadout,
        mut sword_state,
        mut gun_state,
        mut weapon_data,
    ) in &mut hand_query
    {
        let hand_pos = hand_transform.translation().truncate();

        let (_, is_just_pressed, skill_pressed) = match hand.side {
            HandType::Left => (left_pressed, left_just_pressed, q_just_pressed),
            HandType::Right => (right_pressed, right_just_pressed, e_just_pressed),
        };

        if let Some(weapon_type) = hand.equipped_weapon {
            match weapon_type {
                WeaponType::Magic => {
                    if skill_pressed {
                        match magic_loadout.active_slot {
                            ActiveSpellSlot::Primary => {
                                magic_loadout.active_slot = ActiveSpellSlot::Secondary;
                            }
                            ActiveSpellSlot::Secondary => {
                                magic_loadout.active_slot = ActiveSpellSlot::Primary;
                            }
                        }
                    }

                    let now = params.time.elapsed_secs();
                    if is_just_pressed && now - weapon_data.last_shot >= weapon_data.cooldown {
                        let spell_to_cast = match magic_loadout.active_slot {
                            ActiveSpellSlot::Primary => magic_loadout.primary,
                            ActiveSpellSlot::Secondary => magic_loadout.secondary,
                        };

                        cast_spell(
                            &mut params,
                            spell_to_cast,
                            player_entity,
                            &mut player_transform,
                            cursor_pos,
                            hand_pos,
                            player.damage_multiplier,
                        );
                        weapon_data.last_shot = now;
                    }
                }
                WeaponType::Gun => {
                    let cooldown = match gun_state.mode {
                        GunMode::Rapid => gun::RAPID_COOLDOWN,
                        _ => gun::STANDARD_COOLDOWN,
                    };

                    let should_fire = if gun_state.mode == GunMode::Rapid {
                        params.time.elapsed_secs() - weapon_data.last_shot >= cooldown
                    } else {
                        is_just_pressed
                    };

                    if should_fire {
                        fire_weapon(
                            &mut params,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            sword_state.mode,
                            gun_state.mode,
                            hand_entity,
                            player.damage_multiplier,
                        );
                        weapon_data.last_shot = params.time.elapsed_secs();
                    }

                    let now = params.time.elapsed_secs();
                    if skill_pressed
                        && now - weapon_data.last_skill_use >= gun::MODE_SWITCH_COOLDOWN
                        && perform_skill(
                            &mut params,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            &magic_loadout,
                            &mut sword_state,
                            &mut gun_state,
                            &mut player_transform,
                        )
                    {
                        weapon_data.last_skill_use = now;
                    }
                }
                _ => {
                    let now = params.time.elapsed_secs();
                    if is_just_pressed && now - weapon_data.last_shot >= weapon_data.cooldown {
                        fire_weapon(
                            &mut params,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            sword_state.mode,
                            gun_state.mode,
                            hand_entity,
                            player.damage_multiplier,
                        );
                        weapon_data.last_shot = now;
                    }
                    if skill_pressed
                        && now - weapon_data.last_skill_use >= weapon_data.skill_cooldown
                        && perform_skill(
                            &mut params,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            &magic_loadout,
                            &mut sword_state,
                            &mut gun_state,
                            &mut player_transform,
                        )
                    {
                        weapon_data.last_skill_use = now;
                    }
                }
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn cast_spell(
    params: &mut CombatInputParams,
    spell: SpellType,
    player_entity: Entity,
    player_transform: &mut Transform,
    cursor_pos: Vec2,
    spawn_pos: Vec2,
    damage_multiplier: f32,
) {
    let direction = (cursor_pos - spawn_pos).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    match spell {
        SpellType::EnergyBolt => {
            params
                .commands
                .spawn((
                    Transform::from_translation(spawn_pos.extend(0.0))
                        .with_rotation(Quat::from_rotation_z(angle)),
                    Visibility::Visible,
                    Collider::ball(4.0),
                    Velocity {
                        linvel: direction * energy_bolt::SPEED,
                        angvel: 0.0,
                    },
                    Projectile {
                        kind: WeaponType::Magic,
                        damage: energy_bolt::DAMAGE * damage_multiplier,
                        speed: energy_bolt::SPEED,
                        direction,
                        owner_entity: player_entity,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(energy_bolt::LIFETIME, TimerMode::Once),
                    },
                    ExplodingProjectile {
                        radius: energy_bolt::EXPLOSION_RADIUS,
                        damage: energy_bolt::DAMAGE * damage_multiplier,
                    },
                ))
                .with_children(|parent| {
                    spawn_energy_bolt_visuals(parent, &params.cached_assets);
                });
        }
        SpellType::Laser => {
            params
                .commands
                .spawn((
                    Transform::from_translation(spawn_pos.extend(0.0))
                        .with_rotation(Quat::from_rotation_z(angle)),
                    Visibility::Visible,
                    Collider::line(direction, laser::LENGTH, laser::WIDTH / 2.0),
                    Velocity::default(),
                    Projectile {
                        kind: WeaponType::Magic,
                        damage: laser::DAMAGE * damage_multiplier,
                        speed: 0.0,
                        direction,
                        owner_entity: player_entity,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(laser::LIFETIME, TimerMode::Once),
                    },
                    AoEProjectile::default(),
                    IgnoreGrid,
                ))
                .with_children(|parent| {
                    spawn_laser_visuals(parent, &params.cached_assets);
                });
        }
        SpellType::Nova => {
            params
                .commands
                .spawn((
                    Transform::from_translation(player_transform.translation),
                    Visibility::Visible,
                    Collider::ball(nova::RADIUS),
                    Velocity::default(),
                    Projectile {
                        kind: WeaponType::Magic,
                        damage: nova::DAMAGE * damage_multiplier,
                        speed: 0.0,
                        direction: Vec2::ZERO,
                        owner_entity: player_entity,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(nova::LIFETIME, TimerMode::Once),
                    },
                    AoEProjectile::default(),
                ))
                .with_children(|parent| {
                    spawn_nova_visuals(parent, &params.cached_assets);
                });
        }
        SpellType::Blink => {
            player_transform.translation = cursor_pos.extend(0.0);
        }
        SpellType::Global => {
            params
                .commands
                .spawn((
                    Transform::from_translation(player_transform.translation),
                    Visibility::Visible,
                    Collider::ball(global::RADIUS),
                    Velocity::default(),
                    Projectile {
                        kind: WeaponType::Magic,
                        damage: global::DAMAGE * damage_multiplier,
                        speed: 0.0,
                        direction: Vec2::ZERO,
                        owner_entity: player_entity,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(global::LIFETIME, TimerMode::Once),
                    },
                    AoEProjectile::default(),
                    IgnoreGrid,
                ))
                .with_children(|parent| {
                    spawn_global_visuals(parent, &params.cached_assets);
                });
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn perform_skill(
    params: &mut CombatInputParams,
    weapon_type: WeaponType,
    _spawn_pos: Vec2,
    cursor_pos: Vec2,
    player_entity: Entity,
    _magic_loadout: &MagicLoadout,
    sword_state: &mut SwordState,
    gun_state: &mut GunState,
    player_transform: &mut Transform,
) -> bool {
    match weapon_type {
        WeaponType::Shuriken => {
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
                    Transform::from_translation(player_transform.translation)
                        .with_scale(Vec3::splat(15.0)),
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
        WeaponType::Sword => {
            match sword_state.mode {
                SwordMode::Normal => sword_state.mode = SwordMode::Shattered,
                SwordMode::Shattered => sword_state.mode = SwordMode::Normal,
            }
            true
        }
        WeaponType::Gun => {
            match gun_state.mode {
                GunMode::Single => gun_state.mode = GunMode::Shotgun,
                GunMode::Shotgun => gun_state.mode = GunMode::Rapid,
                GunMode::Rapid => gun_state.mode = GunMode::Single,
            }
            true
        }
        WeaponType::Magic => false,
    }
}

#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
fn fire_weapon(
    params: &mut CombatInputParams,
    weapon_type: WeaponType,
    spawn_pos: Vec2,
    target_pos: Vec2,
    owner: Entity,
    sword_mode: SwordMode,
    gun_mode: GunMode,
    hand_entity: Entity,
    damage_multiplier: f32,
) {
    let direction = (target_pos - spawn_pos).normalize_or_zero();
    match weapon_type {
        WeaponType::Shuriken => {
            let mut shurikens: Vec<(Entity, f32)> = params
                .projectile_query
                .iter()
                .filter(|(_, _, p, _)| p.kind == WeaponType::Shuriken && p.owner_entity == owner)
                .map(|(e, _, _, l)| (e, l.timer.remaining_secs()))
                .collect();

            if shurikens.len() >= shuriken::MAX_COUNT {
                shurikens
                    .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
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
        WeaponType::Sword => {
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
                            crate::components::weapon::SwordSwing {
                                state: crate::components::weapon::SwingState::Swinging,
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
                            crate::components::weapon::SwordSwing {
                                state: crate::components::weapon::SwingState::Swinging,
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
        WeaponType::Gun => {
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
                        Collider::cuboid(10.0, 2.5),
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
                            timer: Timer::from_seconds(3.0, TimerMode::Once),
                        },
                    ))
                    .with_children(|parent| {
                        spawn_gun_bullet_visuals(parent, &params.cached_assets);
                    });
            }
        }
        WeaponType::Magic => {}
    }
}
