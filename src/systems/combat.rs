use crate::components::enemy::Enemy;
use crate::components::player::{GameCamera, Hand, HandType, Player};
use crate::components::weapon::{
    ActiveSpellSlot, BowMode, BowState, ExplodingProjectile, Lifetime, MagicLoadout, Projectile,
    SpellType, SwingState, SwordMode, SwordState, SwordSwing, Weapon, WeaponType,
};
use crate::configs::spells::{energy_bolt, global, laser, nova};
use crate::configs::weapons::{bow, shuriken, sword};
use bevy::color::palettes::css::{AQUA, AZURE, PURPLE, YELLOW};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[allow(clippy::too_many_arguments)]
pub fn handle_combat_input(
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    mut hand_query: Query<(
        Entity,
        &GlobalTransform,
        &Hand,
        &mut MagicLoadout,
        &mut SwordState,
        &mut BowState,
        &mut Weapon,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    projectile_query: Query<(Entity, &GlobalTransform, &Projectile), Without<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<Player>>, // For Global spell
) {
    // ... (Keep early returns)
    let (camera, camera_transform) = if let Ok(res) = camera_query.get_single() {
        res
    } else {
        return;
    };
    let window = if let Ok(w) = window_query.get_single() {
        w
    } else {
        return;
    };

    let cursor_pos = if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        world_position
    } else {
        return;
    };

    let (player_entity, mut player_transform) = if let Ok(p) = player_query.get_single_mut() {
        p
    } else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    // Input States
    let left_pressed = mouse_input.pressed(MouseButton::Left);
    let right_pressed = mouse_input.pressed(MouseButton::Right);
    let left_just_pressed = mouse_input.just_pressed(MouseButton::Left);
    let right_just_pressed = mouse_input.just_pressed(MouseButton::Right);

    let q_just_pressed = key_input.just_pressed(KeyCode::KeyQ);
    let e_just_pressed = key_input.just_pressed(KeyCode::KeyE);

    for (
        hand_entity,
        hand_transform,
        hand,
        mut magic_loadout,
        mut sword_state,
        mut bow_state,
        mut weapon_data,
    ) in hand_query.iter_mut()
    {
        let hand_pos = hand_transform.translation().truncate();
        let direction = (cursor_pos - hand_pos).normalize_or_zero();
        let _angle = direction.y.atan2(direction.x);

        // Input mapping
        let (is_pressed, is_just_pressed, skill_pressed) = match hand.hand_type {
            HandType::Left => (left_pressed, left_just_pressed, q_just_pressed),
            HandType::Right => (right_pressed, right_just_pressed, e_just_pressed),
        };

        if let Some(weapon_type) = hand.equipped_weapon {
            match weapon_type {
                WeaponType::Magic => {
                    // Toggle Active Slot (Skill Key)
                    // Magic Swap is instant (no cooldown) or very short? Let's keep it instant for fluidity.
                    if skill_pressed {
                        match magic_loadout.active_slot {
                            ActiveSpellSlot::Primary => {
                                magic_loadout.active_slot = ActiveSpellSlot::Secondary;
                                println!("Magic: Switched to Secondary Spell");
                            }
                            ActiveSpellSlot::Secondary => {
                                magic_loadout.active_slot = ActiveSpellSlot::Primary;
                                println!("Magic: Switched to Primary Spell");
                            }
                        }
                    }

                    // Cast Active Spell (Click)
                    let now = time.elapsed_seconds();
                    if is_just_pressed && now - weapon_data.last_shot >= weapon_data.cooldown {
                        let spell_to_cast = match magic_loadout.active_slot {
                            ActiveSpellSlot::Primary => magic_loadout.primary,
                            ActiveSpellSlot::Secondary => magic_loadout.secondary,
                        };

                        cast_spell(
                            &mut commands,
                            spell_to_cast,
                            player_entity,
                            &mut player_transform,
                            cursor_pos,
                            hand_pos,
                            &mut meshes,
                            &mut materials,
                            &mut enemy_query,
                        );
                        weapon_data.last_shot = now;
                    }
                }
                WeaponType::Bow => {
                    // Bow Logic (Supports Rapid Fire)
                    let cooldown = match bow_state.mode {
                        BowMode::Rapid => bow::RAPID_COOLDOWN,
                        _ => bow::STANDARD_COOLDOWN,
                    };

                    let should_fire = if bow_state.mode == BowMode::Rapid {
                        is_pressed && time.elapsed_seconds() - weapon_data.last_shot >= cooldown
                    } else {
                        is_just_pressed
                    };

                    if should_fire {
                        fire_weapon(
                            &mut commands,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            &mut meshes,
                            &mut materials,
                            sword_state.mode,
                            bow_state.mode,
                        );
                        weapon_data.last_shot = time.elapsed_seconds();
                    }

                    let now = time.elapsed_seconds();
                    if skill_pressed {
                        // Bow Mode Switch is instant/tactical, maybe small cooldown?
                        // Let's add small cooldown to prevent accidental double taps
                        if now - weapon_data.last_skill_use >= bow::MODE_SWITCH_COOLDOWN {
                            perform_skill(
                                &mut commands,
                                weapon_type,
                                hand_pos,
                                cursor_pos,
                                player_entity,
                                &*magic_loadout,
                                &mut sword_state,
                                &mut bow_state,
                                &mut meshes,
                                &mut materials,
                                &projectile_query,
                                &mut player_transform,
                            );
                            weapon_data.last_skill_use = now;
                        }
                    }
                }
                _ => {
                    // Standard Weapons (Sword, Shuriken)
                    let now = time.elapsed_seconds();
                    if is_just_pressed && now - weapon_data.last_shot >= weapon_data.cooldown {
                        fire_weapon(
                            &mut commands,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            &mut meshes,
                            &mut materials,
                            sword_state.mode, // Pass Mode
                            bow_state.mode,   // Pass Bow Mode
                        );
                        weapon_data.last_shot = now;
                    }
                    if skill_pressed
                        && now - weapon_data.last_skill_use >= weapon_data.skill_cooldown
                    {
                        perform_skill(
                            &mut commands,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            &*magic_loadout,
                            &mut sword_state, // Pass State
                            &mut bow_state,   // Pass Bow State
                            &mut meshes,
                            &mut materials,
                            &projectile_query,
                            &mut player_transform,
                        );
                        weapon_data.last_skill_use = now;
                    }
                }
            }
        }
    }
}

fn cast_spell(
    commands: &mut Commands,
    spell: SpellType,
    player_entity: Entity,
    player_transform: &mut Transform,
    cursor_pos: Vec2,
    spawn_pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    enemy_query: &mut Query<(Entity, &Transform, &mut Enemy), Without<Player>>, // Mutable for damage
) {
    let direction = (cursor_pos - spawn_pos).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    match spell {
        SpellType::EnergyBolt => {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Circle::new(8.0))).into(),
                    material: materials.add(Color::from(PURPLE)),
                    transform: Transform::from_translation(spawn_pos.extend(0.0)),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(4.0),
                Sensor,
                GravityScale(0.0),
                Velocity {
                    linvel: direction * energy_bolt::SPEED,
                    angvel: 0.0,
                },
                Projectile {
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
                    damage: energy_bolt::DAMAGE, // Using same damage for explosion for now
                },
            ));
        }
        SpellType::Laser => {
            // Raycast / Long box
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Rectangle::new(1000.0, 4.0))).into(),
                    material: materials.add(Color::from(AQUA)),
                    transform: Transform::from_translation(
                        (spawn_pos + direction * (laser::LENGTH / 2.0)).extend(0.0), // Center it
                    )
                    .with_rotation(Quat::from_rotation_z(angle)),
                    ..default()
                },
                Sensor,
                Collider::cuboid(laser::LENGTH / 2.0, laser::WIDTH / 2.0), // Half-extents
                Projectile {
                    damage: laser::DAMAGE,
                    speed: 0.0,
                    direction,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(laser::LIFETIME, TimerMode::Once),
                },
            ));
        }
        SpellType::Nova => {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Circle::new(nova::RADIUS))).into(),
                    material: materials.add(Color::srgb(1.0, 0.0, 1.0).with_alpha(0.4)),
                    transform: Transform::from_translation(player_transform.translation),
                    ..default()
                },
                Sensor,
                Collider::ball(nova::RADIUS),
                Projectile {
                    damage: nova::DAMAGE,
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(nova::LIFETIME, TimerMode::Once),
                },
            ));
        }
        SpellType::Blink => {
            player_transform.translation = cursor_pos.extend(0.0);
            println!("Blink!");
        }
        SpellType::Global => {
            println!("Global Spell Used!");
            // Global is now a massive Nova
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Circle::new(global::RADIUS))).into(),
                    material: materials.add(Color::srgb(1.0, 1.0, 1.0).with_alpha(0.1)), // White flash
                    transform: Transform::from_translation(player_transform.translation),
                    ..default()
                },
                Sensor,
                Collider::ball(global::RADIUS),
                Projectile {
                    damage: global::DAMAGE, // Back to reasonable damage (single hit)
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(global::LIFETIME, TimerMode::Once),
                },
            ));
        }
    }
}

// ... (Keep existing perform_skill and fire_weapon functions, they are fine)
fn perform_skill(
    commands: &mut Commands,
    weapon_type: WeaponType,
    spawn_pos: Vec2,
    cursor_pos: Vec2,
    player_entity: Entity,
    _magic_loadout: &MagicLoadout,
    sword_state: &mut SwordState,
    bow_state: &mut BowState,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    projectile_query: &Query<(Entity, &GlobalTransform, &Projectile), Without<Player>>,
    player_transform: &mut Transform,
) {
    match weapon_type {
        WeaponType::Shuriken => {
            // Teleport to closest projectile
            let mut closest_proj: Option<(Entity, Vec3)> = None;
            let mut min_dist_sq = shuriken::TELEPORT_RANGE_SQ; // Max teleport range check? Or just find any.

            for (entity, proj_tf, proj) in projectile_query.iter() {
                if proj.owner_entity == player_entity {
                    // Check if it is a Shuriken (hack: speed 600.0 or just purely by being projectile owner)
                    // Ideally we'd have a WeaponType on Projectile, but owner is unique enough for now.
                    let translation = proj_tf.translation();
                    let dist_sq = translation.truncate().distance_squared(cursor_pos);
                    if dist_sq < min_dist_sq {
                        min_dist_sq = dist_sq;
                        closest_proj = Some((entity, translation));
                    }
                }
            }

            if let Some((entity, location)) = closest_proj {
                // Teleport FX (at old position)
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(Circle::new(15.0))).into(),
                        material: materials.add(Color::srgba(0.0, 1.0, 1.0, 0.5)),
                        transform: Transform::from_translation(player_transform.translation),
                        ..default()
                    },
                    Lifetime {
                        timer: Timer::from_seconds(0.2, TimerMode::Once),
                    },
                ));

                // Move Player
                player_transform.translation = location;
                println!("Skill: Shuriken Teleport to {:?}", location);

                // Teleport FX (at new position)
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(Circle::new(15.0))).into(),
                        material: materials.add(Color::srgba(0.0, 1.0, 1.0, 0.5)),
                        transform: Transform::from_translation(location),
                        ..default()
                    },
                    Lifetime {
                        timer: Timer::from_seconds(0.2, TimerMode::Once),
                    },
                ));

                // Despawn the projectile used as anchor
                commands.entity(entity).despawn_recursive();
            } else {
                println!("Skill: No Shuriken found to teleport to!");
            }
        }
        WeaponType::Sword => {
            // Toggle Mode
            match sword_state.mode {
                SwordMode::Normal => {
                    sword_state.mode = SwordMode::Shattered;
                    println!("Sword Mode: Shattered!");
                }
                SwordMode::Shattered => {
                    sword_state.mode = SwordMode::Normal;
                    println!("Sword Mode: Normal!");
                }
            }
        }
        WeaponType::Bow => {
            // Toggle Bow Mode
            match bow_state.mode {
                BowMode::Single => {
                    bow_state.mode = BowMode::Multishot;
                    println!("Bow Mode: Multishot (Triple Shot)!");
                }
                BowMode::Multishot => {
                    bow_state.mode = BowMode::Rapid;
                    println!("Bow Mode: Rapid Fire (Machine Gun)!");
                }
                BowMode::Rapid => {
                    bow_state.mode = BowMode::Single;
                    println!("Bow Mode: Single (Precision)!");
                }
            }
        }
        _ => {}
    }
}

fn fire_weapon(
    commands: &mut Commands,
    weapon_type: WeaponType,
    spawn_pos: Vec2,
    target_pos: Vec2,
    owner: Entity,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    sword_mode: SwordMode, // Added mode
    bow_mode: BowMode,     // Added Bow Mode
) {
    let direction = (target_pos - spawn_pos).normalize_or_zero();
    match weapon_type {
        WeaponType::Shuriken => {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Rectangle::new(10.0, 10.0))).into(),
                    material: materials.add(Color::from(AZURE)),
                    transform: Transform::from_translation(spawn_pos.extend(0.0)),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(5.0),
                Sensor,
                GravityScale(0.0),
                GravityScale(0.0),
                Velocity {
                    linvel: direction * shuriken::SPEED,
                    angvel: 10.0,
                },
                Projectile {
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
                    // Normal 2-Phase Sweep
                    commands
                        .spawn((
                            SpatialBundle {
                                transform: Transform::from_translation(spawn_pos.extend(0.0)),
                                ..default()
                            },
                            SwordSwing {
                                state: SwingState::Windup,
                                timer: Timer::from_seconds(sword::NORMAL_TIMER, TimerMode::Once),
                                base_angle: start_angle,
                                owner_entity: owner,
                                damage: sword::NORMAL_DAMAGE,
                                range: sword::NORMAL_RANGE,
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((MaterialMesh2dBundle {
                                mesh: meshes.add(Mesh::from(Rectangle::new(140.0, 10.0))).into(),
                                material: materials.add(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                                transform: Transform::from_xyz(70.0, 0.0, 0.0),
                                ..default()
                            },));
                        });
                }
                SwordMode::Shattered => {
                    // Shattered 2-Phase Sweep (Skill Visuals/Stats)
                    commands
                        .spawn((
                            SpatialBundle {
                                transform: Transform::from_translation(spawn_pos.extend(0.0)),
                                ..default()
                            },
                            SwordSwing {
                                state: SwingState::Windup,
                                timer: Timer::from_seconds(sword::SHATTERED_TIMER, TimerMode::Once),
                                base_angle: start_angle,
                                owner_entity: owner,
                                damage: sword::SHATTERED_DAMAGE,
                                range: sword::SHATTERED_RANGE,
                            },
                        ))
                        .with_children(|parent| {
                            // Shattered Visuals
                            let mut rng = rand::thread_rng();
                            for _ in 0..40 {
                                let dist = rng.gen_range(50.0..350.0);
                                let jitter_y = rng.gen_range(-15.0..15.0);
                                parent.spawn(MaterialMesh2dBundle {
                                    mesh: meshes
                                        .add(Mesh::from(Circle::new(rng.gen_range(2.0..4.0))))
                                        .into(),
                                    material: materials.add(Color::srgba(1.0, 1.0, 1.0, 0.9)),
                                    transform: Transform::from_xyz(dist, jitter_y, 0.0),
                                    ..default()
                                });
                            }
                        });
                }
            }
        }
        WeaponType::Bow => {
            let base_angle = direction.y.atan2(direction.x);

            // Refactoring to handle dynamic spread or just handle it inside the loop
            let mut projectiles = Vec::new();
            match bow_mode {
                BowMode::Single => projectiles.push((0.0, bow::SINGLE_DAMAGE, bow::SINGLE_SPEED)),
                BowMode::Multishot => {
                    for &s in bow::MULTISHOT_SPREAD {
                        projectiles.push((s, bow::MULTISHOT_DAMAGE, bow::MULTISHOT_SPEED));
                    }
                }
                BowMode::Rapid => {
                    let mut rng = rand::thread_rng();
                    let jitter = rng.gen_range(-bow::RAPID_SPREAD_JITTER..bow::RAPID_SPREAD_JITTER);
                    projectiles.push((jitter, bow::RAPID_DAMAGE, bow::RAPID_SPEED));
                }
            }

            for (offset, damage, speed) in projectiles {
                let angle = base_angle + offset;
                let dir = Vec2::new(angle.cos(), angle.sin());

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(Rectangle::new(20.0, 5.0))).into(),
                        material: materials.add(Color::from(YELLOW)),
                        transform: Transform::from_translation(spawn_pos.extend(0.0))
                            .with_rotation(Quat::from_rotation_z(angle)),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(10.0, 2.5),
                    Sensor,
                    GravityScale(0.0),
                    Velocity {
                        linvel: dir * speed,
                        angvel: 0.0,
                    },
                    Projectile {
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
        _ => {}
    }
}

pub fn resolve_damage(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    projectile_query: Query<(Entity, &Projectile, &Transform)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<Player>>, // Keep mutable for direct damage
    // Shield Logic needs access to Shields
    mut shake: ResMut<crate::resources::polish::ScreenShake>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    exploding_query: Query<&ExplodingProjectile>,
) {
    // 2. Check Projectile vs Enemy
    for (proj_entity, projectile, projectile_transform) in projectile_query.iter() {
        // Skip if projectile dead (from absorb) - ECS despawn is deferred, so we might need manual check or `commands.entity(e).despawn()` works at end of stage.
        // Actually, despawned entities are still iteratable in the same system execution usually? No, but multiple loops might clash.
        // Let's rely on standard intersections.

        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if rapier_context.intersection_pair(proj_entity, enemy_entity) == Some(true)
                && projectile.owner_entity != enemy_entity
            {
                // Don't hit self if reflected?
                enemy.health -= projectile.damage;
                shake.add_trauma(0.1); // Small shake on hit

                // Explosion Logic
                if let Ok(exploding) = exploding_query.get(proj_entity) {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(Circle::new(exploding.radius))).into(),
                            material: materials.add(Color::srgb(1.0, 0.5, 0.0).with_alpha(0.6)),
                            transform: Transform::from_translation(
                                projectile_transform.translation,
                            ),
                            ..default()
                        },
                        Collider::ball(exploding.radius),
                        Sensor,
                        Projectile {
                            damage: exploding.damage,
                            speed: 0.0,
                            direction: Vec2::ZERO,
                            owner_entity: projectile.owner_entity,
                        },
                        Lifetime {
                            timer: Timer::from_seconds(0.1, TimerMode::Once),
                        },
                    ));
                }

                // Despawn projectile after hit (even stationary ones like Nova/Global)
                // This prevents them from dealing damage every frame of their lifetime.
                // Commands are deferred, so it will still finish colliding with other enemies in this frame.
                commands.entity(proj_entity).despawn();

                if enemy.health <= 0.0 {
                    commands.entity(enemy_entity).despawn_recursive();
                    shake.add_trauma(0.3); // Big shake on kill
                    println!("Enemy Killed!");

                    // Spawn Particles
                    let mut rng = rand::thread_rng();
                    for _ in 0..5 {
                        let dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                            .normalize_or_zero();
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes.add(Mesh::from(Circle::new(3.0))).into(),
                                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                                transform: Transform::from_translation(enemy_transform.translation),
                                ..default()
                            },
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
}

pub fn update_sword_mechanics(
    mut commands: Commands,
    time: Res<Time>,
    mut sword_query: Query<(Entity, &mut SwordSwing, &mut Transform)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<SwordSwing>>,
    mut shake: ResMut<crate::resources::polish::ScreenShake>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut swing, mut transform) in sword_query.iter_mut() {
        swing.timer.tick(time.delta());

        match swing.state {
            SwingState::Windup => {
                let start_idx = -std::f32::consts::FRAC_PI_2; // -90 deg
                let current_angle = swing.base_angle + start_idx;
                transform.rotation = Quat::from_rotation_z(current_angle);

                if swing.timer.finished() {
                    swing.state = SwingState::Swinging;
                    swing.timer = Timer::from_seconds(0.2, TimerMode::Once);

                    // Damage Scan
                    let sweep_radius = swing.range;
                    let sweep_arc = std::f32::consts::PI; // 180 degrees

                    for (enemy_entity, enemy_tf, mut enemy) in enemy_query.iter_mut() {
                        let to_enemy =
                            enemy_tf.translation.truncate() - transform.translation.truncate();

                        let dist_sq = to_enemy.length_squared();
                        if dist_sq < sweep_radius * sweep_radius {
                            let angle_to_enemy = to_enemy.y.atan2(to_enemy.x);
                            let mut angle_diff = angle_to_enemy - swing.base_angle;
                            while angle_diff > std::f32::consts::PI {
                                angle_diff -= 2.0 * std::f32::consts::PI;
                            }
                            while angle_diff < -std::f32::consts::PI {
                                angle_diff += 2.0 * std::f32::consts::PI;
                            }

                            if angle_diff.abs() <= sweep_arc / 2.0 {
                                enemy.health -= swing.damage;
                                shake.add_trauma(0.05);

                                let mut rng = rand::thread_rng();
                                for _ in 0..3 {
                                    let dir = Vec2::new(
                                        rng.gen_range(-1.0..1.0),
                                        rng.gen_range(-1.0..1.0),
                                    )
                                    .normalize_or_zero();
                                    commands.spawn((
                                        MaterialMesh2dBundle {
                                            mesh: meshes.add(Mesh::from(Circle::new(2.0))).into(),
                                            material: materials.add(Color::srgb(1.0, 0.5, 0.0)),
                                            transform: Transform::from_translation(
                                                enemy_tf.translation,
                                            ),
                                            ..default()
                                        },
                                        Velocity {
                                            linvel: dir * 150.0,
                                            angvel: 0.0,
                                        },
                                        Lifetime {
                                            timer: Timer::from_seconds(0.3, TimerMode::Once),
                                        },
                                    ));
                                }

                                if enemy.health <= 0.0 {
                                    commands.entity(enemy_entity).despawn_recursive();
                                    shake.add_trauma(0.2);
                                }
                            }
                        }
                    }
                }
            }
            SwingState::Swinging => {
                let progress = 1.0 - swing.timer.fraction_remaining();
                let start_angle = -std::f32::consts::FRAC_PI_2;
                let end_angle = std::f32::consts::FRAC_PI_2;
                let current_angle =
                    swing.base_angle + start_angle + (end_angle - start_angle) * progress;
                transform.rotation = Quat::from_rotation_z(current_angle);

                if swing.timer.finished() {
                    swing.state = SwingState::Recover;
                    swing.timer = Timer::from_seconds(0.1, TimerMode::Once);
                }
            }
            SwingState::Recover => {
                if swing.timer.finished() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn manage_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
