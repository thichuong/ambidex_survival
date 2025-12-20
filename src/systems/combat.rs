use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, IgnoreGrid, UniformGrid, Velocity, check_collision};
use crate::components::player::{GameCamera, Hand, HandType, Player};
use crate::components::weapon::{
    ActiveSpellSlot, AoEProjectile, ExplodingProjectile, GunMode, GunState, Lifetime, MagicLoadout,
    Projectile, SpellType, SwingState, SwordMode, SwordState, SwordSwing, Weapon, WeaponType,
};
use crate::configs::spells::{energy_bolt, global, laser, nova};
use crate::configs::weapons::{gun, shuriken, sword};
use crate::systems::object_pooling::{EffectType, PooledEffect, VisualEffectPool};
use crate::systems::weapon_visuals::{
    spawn_bolt_explosion_visuals, spawn_energy_bolt_visuals, spawn_global_visuals,
    spawn_gun_bullet_visuals, spawn_laser_visuals, spawn_nova_visuals, spawn_shuriken_visuals,
    spawn_sword_normal_visuals, spawn_sword_shattered_visuals,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use rand::Rng;

#[derive(Event, Message, Debug)]
pub struct DamageEvent {
    pub damage: f32,
    pub position: Vec2,
}

use bevy::window::PrimaryWindow;

#[derive(SystemParam)]
pub struct CombatInputParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub mouse_input: Res<'w, ButtonInput<MouseButton>>,
    pub key_input: Res<'w, ButtonInput<KeyCode>>,
    pub window_query: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<GameCamera>>,
    pub cached_assets: Res<'w, crate::resources::cached_assets::CachedAssets>,
    pub projectile_query: Query<
        'w,
        's,
        (
            Entity,
            &'static GlobalTransform,
            &'static Projectile,
            &'static Lifetime,
        ),
        Without<Player>,
    >,
}

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

        let (is_pressed, is_just_pressed, skill_pressed) = match hand.side {
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
                        is_pressed && params.time.elapsed_secs() - weapon_data.last_shot >= cooldown
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
                    Transform::from_translation(spawn_pos.extend(0.0)),
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

#[derive(SystemParam)]
pub struct CombatResources<'w, 's> {
    pub shake: ResMut<'w, crate::resources::polish::ScreenShake>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub cached_assets: Res<'w, crate::resources::cached_assets::CachedAssets>,
    pub exploding_query: Query<'w, 's, &'static ExplodingProjectile>,
    pub effect_pool: ResMut<'w, VisualEffectPool>,
}

type ProjectileQueryItem<'a> = (
    Entity,
    &'a Projectile,
    &'a Transform,
    &'a Collider,
    Option<Mut<'a, AoEProjectile>>,
    Option<&'a IgnoreGrid>,
);

fn get_collision_candidates(
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
fn handle_projectile_hit(
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
                MeshMaterial2d(res.materials.add(Color::srgb(1.0, 0.0, 0.0))), // Keep dynamic red for now or cache it
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

#[allow(
    clippy::unnecessary_wraps,
    clippy::needless_pass_by_value,
    clippy::too_many_arguments
)]
pub fn update_sword_mechanics(
    mut commands: Commands,
    time: Res<Time>,
    mut sword_query: Query<(Entity, &mut SwordSwing, &mut Transform)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<SwordSwing>>,
    hand_query: Query<&GlobalTransform, With<Hand>>,
    mut res: CombatResources,
    mut damage_events: MessageWriter<DamageEvent>,
    mut player_query: Query<&mut Player>,
) -> Result<(), String> {
    for (entity, mut swing, mut transform) in &mut sword_query {
        if let Ok(hand_transform) = hand_query.get(swing.hand_entity) {
            transform.translation = hand_transform.translation().truncate().extend(0.0);
        }

        swing.timer.tick(time.delta());

        match swing.state {
            SwingState::Swinging => {
                // Calculate current swing progress (0.0 to 1.0)
                let progress = swing.timer.fraction();

                // Swing from -90 degrees to +90 degrees (180-degree arc)
                // Mouse cursor is at the center (base_angle)
                let current_angle =
                    (progress - 0.5).mul_add(std::f32::consts::PI, swing.base_angle);
                transform.rotation = Quat::from_rotation_z(current_angle);

                if !swing.damage_dealt {
                    let sweep_radius = swing.range;

                    for (enemy_entity, enemy_tf, mut enemy) in &mut enemy_query {
                        let to_enemy =
                            enemy_tf.translation.truncate() - transform.translation.truncate();
                        let distance = to_enemy.length();

                        if distance <= sweep_radius && distance > 0.0 {
                            // Check if enemy is within the 180-degree arc
                            let enemy_direction = to_enemy / distance;
                            let base_direction =
                                Vec2::new(swing.base_angle.cos(), swing.base_angle.sin());
                            let dot = enemy_direction.dot(base_direction);

                            // dot > 0 means the angle is less than 90 degrees from base_angle
                            // This creates a 180-degree arc centered on the mouse cursor
                            if dot > 0.0 {
                                enemy.health -= swing.damage;
                                damage_events.write(DamageEvent {
                                    damage: swing.damage,
                                    position: enemy_tf.translation.truncate(),
                                });
                                res.shake.add_trauma(0.2);
                                if enemy.health <= 0.0 {
                                    if let Ok(mut player) = player_query.single_mut() {
                                        player.gold += 10;
                                    }
                                    commands.entity(enemy_entity).despawn();
                                    res.shake.add_trauma(0.4);

                                    let mut rng = rand::thread_rng();
                                    for _ in 0..5 {
                                        let dir = Vec2::new(
                                            rng.gen_range(-1.0..1.0),
                                            rng.gen_range(-1.0..1.0),
                                        )
                                        .normalize_or_zero();
                                        commands.spawn((
                                            Mesh2d(res.meshes.add(Circle::new(3.0))),
                                            MeshMaterial2d(
                                                res.materials.add(Color::srgb(1.0, 0.0, 0.0)),
                                            ),
                                            Transform::from_translation(enemy_tf.translation),
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
                        }
                    }
                    swing.damage_dealt = true;
                }

                if swing.timer.is_finished() {
                    swing.state = SwingState::Recover;
                    swing
                        .timer
                        .set_duration(std::time::Duration::from_secs_f32(0.1));
                    swing.timer.reset();
                }
            }
            SwingState::Recover => {
                if swing.timer.is_finished() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn manage_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime, Option<&PooledEffect>)>,
    mut effect_pool: ResMut<VisualEffectPool>,
) -> Result<(), String> {
    for (entity, mut lifetime, pooled) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            if let Some(pooled) = pooled {
                effect_pool.return_to_pool(entity, pooled.kind, &mut commands);
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
    Ok(())
}

/// Update enemy grid for spatial partitioning - rebuilds grid each frame
#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn update_enemy_grid(
    mut grid: ResMut<UniformGrid>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) -> Result<(), String> {
    grid.clear();
    for (entity, transform) in &enemy_query {
        grid.insert(entity, transform.translation.truncate());
    }
    Ok(())
}

/// Constant for push strength when player and enemy collide
const COLLISION_PUSH_STRENGTH: f32 = 200.0;

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn handle_player_collision(
    mut player_query: Query<(&mut Player, &mut Transform, &Collider), Without<Enemy>>,
    mut enemy_query: Query<(Entity, &mut Transform, &Enemy, &Collider), Without<Player>>,
    grid: Res<UniformGrid>,
    time: Res<Time>,
    mut res: CombatResources,
    mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>,
) -> Result<(), String> {
    if let Ok((mut player, mut player_transform, player_collider)) = player_query.single_mut() {
        player.invulnerability_timer.tick(time.delta());

        let player_pos = player_transform.translation.truncate();

        // Query nearby enemies using spatial grid instead of iterating all
        let nearby_entities = grid.query_nearby(player_pos);

        for enemy_entity in nearby_entities {
            if let Ok((_, mut enemy_transform, enemy, enemy_collider)) =
                enemy_query.get_mut(enemy_entity)
            {
                let enemy_pos = enemy_transform.translation.truncate();

                if check_collision(player_pos, player_collider, enemy_pos, enemy_collider) {
                    // Calculate push direction and apply separation
                    let diff = player_pos - enemy_pos;
                    let distance = diff.length();

                    if distance > 0.0 {
                        let push_dir = diff / distance;
                        let push_amount = COLLISION_PUSH_STRENGTH * time.delta_secs();

                        // Push both player and enemy apart
                        player_transform.translation.x += push_dir.x * push_amount;
                        player_transform.translation.y += push_dir.y * push_amount;
                        enemy_transform.translation.x -= push_dir.x * push_amount;
                        enemy_transform.translation.y -= push_dir.y * push_amount;
                    }

                    // Apply damage only if not invulnerable
                    if player.invulnerability_timer.is_finished() {
                        player.health -= enemy.damage;
                        player.invulnerability_timer.reset();
                        res.shake.add_trauma(0.5);

                        if player.health <= 0.0 {
                            player.health = 0.0;
                            next_state.set(crate::resources::game_state::GameState::GameOver);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
