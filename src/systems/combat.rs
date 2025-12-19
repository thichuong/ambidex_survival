use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, Velocity, check_collision};
use crate::components::player::{GameCamera, Hand, HandType, Player};
use crate::components::weapon::{
    ActiveSpellSlot, AoEProjectile, ExplodingProjectile, GunMode, GunState, Lifetime, MagicLoadout,
    Projectile, SpellType, SwingState, SwordMode, SwordState, SwordSwing, Weapon, WeaponType,
};
use crate::configs::spells::{energy_bolt, global, laser, nova};
use crate::configs::weapons::{gun, shuriken, sword};
use bevy::color::palettes::css::{AQUA, AZURE, PURPLE, YELLOW};
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
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
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
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
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

    let (player_entity, mut player_transform) = player_query
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
) {
    let direction = (cursor_pos - spawn_pos).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    match spell {
        SpellType::EnergyBolt => {
            params.commands.spawn((
                (
                    Mesh2d(params.meshes.add(Circle::new(8.0))),
                    MeshMaterial2d(params.materials.add(Color::from(PURPLE))),
                    Transform::from_translation(spawn_pos.extend(0.0)),
                ),
                Collider::ball(4.0),
                Velocity {
                    linvel: direction * energy_bolt::SPEED,
                    angvel: 0.0,
                },
                Projectile {
                    kind: WeaponType::Magic,
                    damage: energy_bolt::DAMAGE,
                    speed: energy_bolt::SPEED,
                    direction,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(energy_bolt::LIFETIME, TimerMode::Once),
                },
                ExplodingProjectile {
                    radius: energy_bolt::EXPLOSION_RADIUS,
                    damage: energy_bolt::DAMAGE,
                },
            ));
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
                        damage: laser::DAMAGE,
                        speed: 0.0,
                        direction,
                        owner_entity: player_entity,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(laser::LIFETIME, TimerMode::Once),
                    },
                    AoEProjectile::default(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Mesh2d(
                            params
                                .meshes
                                .add(Rectangle::new(laser::LENGTH, laser::WIDTH)),
                        ),
                        MeshMaterial2d(params.materials.add(Color::from(AQUA))),
                        Transform::from_xyz(laser::LENGTH / 2.0, 0.0, 0.0),
                    ));
                });
        }
        SpellType::Nova => {
            params.commands.spawn((
                Mesh2d(params.meshes.add(Circle::new(nova::RADIUS))),
                MeshMaterial2d(params.materials.add(Color::srgba(1.0, 0.0, 1.0, 0.4))),
                Transform::from_translation(player_transform.translation),
                Collider::ball(nova::RADIUS),
                Velocity::default(),
                Projectile {
                    kind: WeaponType::Magic,
                    damage: nova::DAMAGE,
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(nova::LIFETIME, TimerMode::Once),
                },
                AoEProjectile::default(),
            ));
        }
        SpellType::Blink => {
            player_transform.translation = cursor_pos.extend(0.0);
        }
        SpellType::Global => {
            params.commands.spawn((
                Mesh2d(params.meshes.add(Circle::new(global::RADIUS))),
                MeshMaterial2d(params.materials.add(Color::srgba(1.0, 1.0, 1.0, 0.1))),
                Transform::from_translation(player_transform.translation),
                Collider::ball(global::RADIUS),
                Velocity::default(),
                Projectile {
                    kind: WeaponType::Magic,
                    damage: global::DAMAGE,
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(global::LIFETIME, TimerMode::Once),
                },
                AoEProjectile::default(),
            ));
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
                    Mesh2d(params.meshes.add(Circle::new(15.0))),
                    MeshMaterial2d(params.materials.add(Color::srgba(0.0, 1.0, 1.0, 0.5))),
                    Transform::from_translation(player_transform.translation),
                    Lifetime {
                        timer: Timer::from_seconds(0.2, TimerMode::Once),
                    },
                ));
                player_transform.translation = location;
                params.commands.spawn((
                    Mesh2d(params.meshes.add(Circle::new(15.0))),
                    MeshMaterial2d(params.materials.add(Color::srgba(0.0, 1.0, 1.0, 0.5))),
                    Transform::from_translation(location),
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

            params.commands.spawn((
                Mesh2d(params.meshes.add(Rectangle::new(10.0, 10.0))),
                MeshMaterial2d(params.materials.add(Color::from(AZURE))),
                Transform::from_translation(spawn_pos.extend(0.0)),
                Collider::ball(5.0),
                Velocity {
                    linvel: direction * shuriken::SPEED,
                    angvel: 10.0,
                },
                Projectile {
                    kind: WeaponType::Shuriken,
                    damage: shuriken::DAMAGE,
                    speed: shuriken::SPEED,
                    direction,
                    owner_entity: owner,
                },
                Lifetime {
                    timer: Timer::from_seconds(shuriken::LIFETIME, TimerMode::Once),
                },
            ));
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
                                damage: sword::NORMAL_DAMAGE,
                                range: sword::NORMAL_RANGE,
                                damage_dealt: false,
                                hand_entity,
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Mesh2d(params.meshes.add(Rectangle::new(140.0, 10.0))),
                                MeshMaterial2d(
                                    params.materials.add(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                                ),
                                Transform::from_xyz(70.0, 0.0, 0.0),
                            ));
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
                                damage: sword::SHATTERED_DAMAGE,
                                range: sword::SHATTERED_RANGE,
                                damage_dealt: false,
                                hand_entity,
                            },
                        ))
                        .with_children(|parent| {
                            let mut rng = rand::thread_rng();
                            for _ in 0..40 {
                                let dist = rng.gen_range(50.0..sword::SHATTERED_RANGE);
                                let jitter_y = rng.gen_range(-15.0..15.0);
                                parent.spawn((
                                    Mesh2d(params.meshes.add(Circle::new(rng.gen_range(2.0..4.0)))),
                                    MeshMaterial2d(
                                        params.materials.add(Color::srgba(1.0, 1.0, 1.0, 0.9)),
                                    ),
                                    Transform::from_xyz(dist, jitter_y, 0.0),
                                ));
                            }
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

                params.commands.spawn((
                    Mesh2d(params.meshes.add(Rectangle::new(20.0, 5.0))),
                    MeshMaterial2d(params.materials.add(Color::from(YELLOW))),
                    Transform::from_translation(spawn_pos.extend(0.0))
                        .with_rotation(Quat::from_rotation_z(angle)),
                    Collider::cuboid(10.0, 2.5),
                    Velocity {
                        linvel: dir * speed,
                        angvel: 0.0,
                    },
                    Projectile {
                        kind: WeaponType::Gun,
                        damage,
                        speed,
                        direction: dir,
                        owner_entity: owner,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(3.0, TimerMode::Once),
                    },
                ));
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
    pub exploding_query: Query<'w, 's, &'static ExplodingProjectile>,
}

#[allow(clippy::unnecessary_wraps)]
pub fn resolve_damage(
    mut commands: Commands,
    mut projectile_query: Query<(
        Entity,
        &Projectile,
        &Transform,
        &Collider,
        Option<&mut AoEProjectile>,
    )>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy, &Collider), Without<Player>>,
    mut res: CombatResources,
    mut damage_events: MessageWriter<DamageEvent>,
) -> Result<(), String> {
    for (proj_entity, projectile, projectile_transform, proj_collider, mut aoe_opt) in
        &mut projectile_query
    {
        let proj_pos = projectile_transform.translation.truncate();
        let is_aoe = aoe_opt.is_some();
        let mut hits: Vec<(Entity, f32, Vec3)> = Vec::new();

        for (enemy_entity, enemy_transform, enemy, enemy_collider) in &enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            if check_collision(proj_pos, proj_collider, enemy_pos, enemy_collider)
                && projectile.owner_entity != enemy_entity
            {
                if let Some(ref aoe) = aoe_opt
                    && aoe.damaged_entities.contains(&enemy_entity)
                {
                    continue;
                }
                hits.push((enemy_entity, enemy.health, enemy_transform.translation));
            }
        }

        let mut should_despawn = false;
        for (enemy_entity, _, enemy_pos) in &hits {
            if let Some(ref mut aoe) = aoe_opt {
                aoe.damaged_entities.push(*enemy_entity);
            }

            if let Ok((_, _, mut enemy, _)) = enemy_query.get_mut(*enemy_entity) {
                enemy.health -= projectile.damage;
                damage_events.write(DamageEvent {
                    damage: projectile.damage,
                    position: enemy_pos.truncate(),
                });
                res.shake.add_trauma(0.1);

                if !is_aoe {
                    if let Ok(exploding) = res.exploding_query.get(proj_entity) {
                        commands.spawn((
                            Mesh2d(res.meshes.add(Circle::new(exploding.radius))),
                            MeshMaterial2d(res.materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))),
                            Transform::from_translation(projectile_transform.translation),
                            Collider::ball(exploding.radius),
                            Velocity::default(),
                            Projectile {
                                kind: projectile.kind,
                                damage: exploding.damage,
                                speed: 0.0,
                                direction: Vec2::ZERO,
                                owner_entity: projectile.owner_entity,
                            },
                            Lifetime {
                                timer: Timer::from_seconds(0.1, TimerMode::Once),
                            },
                            AoEProjectile::default(),
                        ));
                    }
                    should_despawn = true;
                }

                if enemy.health <= 0.0 {
                    commands.entity(*enemy_entity).despawn();
                    res.shake.add_trauma(0.3);

                    let mut rng = rand::thread_rng();
                    for _ in 0..5 {
                        let dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                            .normalize_or_zero();
                        commands.spawn((
                            Mesh2d(res.meshes.add(Circle::new(3.0))),
                            MeshMaterial2d(res.materials.add(Color::srgb(1.0, 0.0, 0.0))),
                            Transform::from_translation(*enemy_pos),
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

        if should_despawn && !is_aoe {
            commands.entity(proj_entity).despawn();
        }
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn update_sword_mechanics(
    mut commands: Commands,
    time: Res<Time>,
    mut sword_query: Query<(Entity, &mut SwordSwing, &mut Transform)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<SwordSwing>>,
    hand_query: Query<&GlobalTransform, With<Hand>>,
    mut res: CombatResources,
    mut damage_events: MessageWriter<DamageEvent>,
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
    mut query: Query<(Entity, &mut Lifetime)>,
) -> Result<(), String> {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn handle_player_collision(
    mut player_query: Query<(&mut Player, &Transform, &Collider)>,
    enemy_query: Query<(&Enemy, &Transform, &Collider), Without<Player>>,
    time: Res<Time>,
    mut res: CombatResources,
    mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>,
) -> Result<(), String> {
    if let Ok((mut player, player_transform, player_collider)) = player_query.single_mut() {
        player.invulnerability_timer.tick(time.delta());

        if !player.invulnerability_timer.is_finished() {
            return Ok(());
        }

        let player_pos = player_transform.translation.truncate();
        for (enemy, enemy_transform, enemy_collider) in &enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            if check_collision(player_pos, player_collider, enemy_pos, enemy_collider) {
                player.health -= enemy.damage;
                player.invulnerability_timer.reset();
                res.shake.add_trauma(0.5);

                if player.health <= 0.0 {
                    player.health = 0.0;
                    next_state.set(crate::resources::game_state::GameState::GameOver);
                }
                break; // One hit per frame max to avoid instant multiple collisions
            }
        }
    }
    Ok(())
}
