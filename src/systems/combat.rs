use crate::components::enemy::Enemy;
use crate::components::player::{GameCamera, Hand, HandType, Player};
use crate::components::weapon::{
    ExplodingProjectile, Lifetime, MagicLoadout, Projectile, ShieldCollider, ShieldMode,
    ShieldState, SpellType, SwingState, SwordMode, SwordState, SwordSwing, Weapon, WeaponType,
};
use bevy::color::palettes::css::{AQUA, AZURE, PURPLE, YELLOW};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[allow(clippy::too_many_arguments)]
pub fn handle_combat_input(
    mut commands: Commands,
    _time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    mut hand_query: Query<(
        Entity,
        &GlobalTransform,
        &Hand,
        &mut ShieldState,
        &MagicLoadout,
        &mut SwordState,
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

    for (hand_entity, hand_transform, hand, mut shield_state, magic_loadout, mut sword_state) in
        hand_query.iter_mut()
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
                WeaponType::Shield => {
                    // ... (Shield Logic - Same as before)
                    if is_pressed {
                        if shield_state.shield_entity.is_none() {
                            // Spawn Shield (Simplified copy of existing logic to ensure match)
                            let color = match shield_state.mode {
                                ShieldMode::Absorb => Color::srgb(0.0, 0.0, 1.0).with_alpha(0.5),
                                ShieldMode::Reflect => Color::srgb(1.0, 0.5, 0.0).with_alpha(0.5),
                            };
                            let id = commands
                                .spawn((
                                    MaterialMesh2dBundle {
                                        mesh: meshes
                                            .add(Mesh::from(Rectangle::new(10.0, 60.0)))
                                            .into(),
                                        material: materials.add(color),
                                        transform: Transform::from_xyz(30.0, 0.0, 1.0),
                                        ..default()
                                    },
                                    Collider::cuboid(5.0, 30.0),
                                    Sensor,
                                    ShieldCollider {
                                        owner_hand: hand_entity,
                                    },
                                ))
                                .id();
                            commands.entity(hand_entity).push_children(&[id]);
                            shield_state.shield_entity = Some(id);
                        }
                    } else if let Some(id) = shield_state.shield_entity {
                        commands.entity(id).despawn_recursive();
                        shield_state.shield_entity = None;
                    }
                    if skill_pressed {
                        match shield_state.mode {
                            ShieldMode::Absorb => {
                                shield_state.mode = ShieldMode::Reflect;
                                println!("Shield Mode: Reflect");
                                if shield_state.accumulated_damage > 0.0 {
                                    // Shockwave
                                    commands.spawn((
                                        MaterialMesh2dBundle {
                                            mesh: meshes.add(Mesh::from(Circle::new(50.0))).into(),
                                            material: materials
                                                .add(Color::srgb(0.0, 1.0, 1.0).with_alpha(0.3)),
                                            transform: Transform::from_translation(
                                                player_pos.extend(1.0),
                                            ),
                                            ..default()
                                        },
                                        Collider::ball(50.0),
                                        Sensor,
                                        Projectile {
                                            damage: shield_state.accumulated_damage,
                                            speed: 0.0,
                                            direction: Vec2::ZERO,
                                            owner_entity: player_entity,
                                        },
                                        Lifetime {
                                            timer: Timer::from_seconds(0.1, TimerMode::Once),
                                        },
                                    ));
                                    shield_state.accumulated_damage = 0.0;
                                }
                            }
                            ShieldMode::Reflect => {
                                shield_state.mode = ShieldMode::Absorb;
                                println!("Shield Mode: Absorb");
                            }
                        }
                    }
                }
                WeaponType::Magic => {
                    // Primary Spell (Click)
                    if is_just_pressed {
                        cast_spell(
                            &mut commands,
                            magic_loadout.primary,
                            player_entity,
                            &mut player_transform,
                            cursor_pos,
                            hand_pos,
                            &mut meshes,
                            &mut materials,
                            &mut enemy_query,
                        );
                    }
                    // Secondary Spell (Skill)
                    if skill_pressed {
                        cast_spell(
                            &mut commands,
                            magic_loadout.secondary,
                            player_entity,
                            &mut player_transform,
                            cursor_pos,
                            hand_pos,
                            &mut meshes,
                            &mut materials,
                            &mut enemy_query,
                        );
                    }
                }
                _ => {
                    // Standard Weapons (Sword, Shuriken, Bow)
                    if is_just_pressed {
                        fire_weapon(
                            &mut commands,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            &mut meshes,
                            &mut materials,
                            sword_state.mode, // Pass Mode
                        );
                    }
                    if skill_pressed {
                        perform_skill(
                            &mut commands,
                            weapon_type,
                            hand_pos,
                            cursor_pos,
                            player_entity,
                            &mut shield_state,
                            magic_loadout,
                            &mut sword_state, // Pass State
                            &mut meshes,
                            &mut materials,
                            &projectile_query,
                            &mut player_transform,
                        );
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
                    linvel: direction * 500.0,
                    angvel: 0.0,
                },
                Projectile {
                    damage: 25.0,
                    speed: 500.0,
                    direction,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                },
                ExplodingProjectile {
                    radius: 40.0,
                    damage: 20.0,
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
                        (spawn_pos + direction * 500.0).extend(0.0),
                    )
                    .with_rotation(Quat::from_rotation_z(angle)),
                    ..default()
                },
                Sensor,
                Collider::cuboid(500.0, 2.0),
                Projectile {
                    damage: 10.0,
                    speed: 0.0,
                    direction,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(0.1, TimerMode::Once),
                },
            ));
        }
        SpellType::Nova => {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Circle::new(80.0))).into(),
                    material: materials.add(Color::srgb(1.0, 0.0, 1.0).with_alpha(0.4)),
                    transform: Transform::from_translation(player_transform.translation),
                    ..default()
                },
                Sensor,
                Collider::ball(80.0),
                Projectile {
                    damage: 40.0,
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: player_entity,
                },
                Lifetime {
                    timer: Timer::from_seconds(0.2, TimerMode::Once),
                },
            ));
        }
        SpellType::Blink => {
            player_transform.translation = cursor_pos.extend(0.0);
            println!("Blink!");
        }
        SpellType::Global => {
            println!("Global Spell Used!");
            for (_, _, mut enemy) in enemy_query.iter_mut() {
                enemy.health -= 5.0; // Low damage to all
            }
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
    _shield_state: &mut ShieldState,
    _magic_loadout: &MagicLoadout,
    sword_state: &mut SwordState,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    projectile_query: &Query<(Entity, &GlobalTransform, &Projectile), Without<Player>>,
    player_transform: &mut Transform,
) {
    match weapon_type {
        WeaponType::Shuriken => {
            // Teleport to closest projectile
            let mut closest_proj: Option<(Entity, Vec3)> = None;
            let mut min_dist_sq = 500.0 * 500.0; // Max teleport range check? Or just find any.

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
            // Multishot
            let base_dir = (cursor_pos - spawn_pos).normalize_or_zero();
            let base_angle = base_dir.y.atan2(base_dir.x);
            let spread = [-0.3, 0.0, 0.3];
            for offset in spread.iter() {
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
                        linvel: dir * 800.0,
                        angvel: 0.0,
                    },
                    Projectile {
                        damage: 15.0,
                        speed: 800.0,
                        direction: dir,
                        owner_entity: player_entity,
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

fn fire_weapon(
    commands: &mut Commands,
    weapon_type: WeaponType,
    spawn_pos: Vec2,
    target_pos: Vec2,
    owner: Entity,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    sword_mode: SwordMode, // Added mode
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
                Velocity {
                    linvel: direction * 600.0,
                    angvel: 10.0,
                },
                Projectile {
                    damage: 10.0,
                    speed: 600.0,
                    direction,
                    owner_entity: owner,
                },
                Lifetime {
                    timer: Timer::from_seconds(2.0, TimerMode::Once),
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
                                timer: Timer::from_seconds(0.15, TimerMode::Once),
                                base_angle: start_angle,
                                owner_entity: owner,
                                damage: 30.0,
                                range: 200.0, // Standard Range
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
                                timer: Timer::from_seconds(0.2, TimerMode::Once),
                                base_angle: start_angle,
                                owner_entity: owner,
                                damage: 60.0, // Double damage
                                range: 350.0, // High range
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
            let angle = direction.y.atan2(direction.x);
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
                    linvel: direction * 800.0,
                    angvel: 0.0,
                },
                Projectile {
                    damage: 15.0,
                    speed: 800.0,
                    direction,
                    owner_entity: owner,
                },
                Lifetime {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                },
            ));
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
    shield_query: Query<(Entity, &ShieldCollider)>,
    mut hand_query: Query<&mut ShieldState>,
    mut velocity_query: Query<&mut Velocity>,
    mut shake: ResMut<crate::resources::polish::ScreenShake>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    exploding_query: Query<&ExplodingProjectile>,
) {
    // 1. Check Projectile vs Shield
    for (proj_entity, projectile, _proj_tf) in projectile_query.iter() {
        for (shield_entity, shield_collider) in shield_query.iter() {
            if rapier_context.intersection_pair(proj_entity, shield_entity) == Some(true) {
                // Hit Shield!
                if let Ok(mut shield_state) = hand_query.get_mut(shield_collider.owner_hand) {
                    match shield_state.mode {
                        ShieldMode::Absorb => {
                            shield_state.accumulated_damage += projectile.damage;
                            println!(
                                "Shield Absorbed! Total: {}",
                                shield_state.accumulated_damage
                            );
                            commands.entity(proj_entity).despawn();
                        }
                        ShieldMode::Reflect => {
                            // Reflect logic: Reverse velocity?
                            if let Ok(mut vel) = velocity_query.get_mut(proj_entity) {
                                vel.linvel = -vel.linvel;
                            }
                            println!("Shield Reflected!");
                            // ownership change? Not easy without mutable query on projectile component logic,
                            // but effectively it flies back.
                        }
                    }
                }
                // Prevent it from hitting enemies/player in same frame?
                // Projectile continues if reflected, destroyed if absorbed.
            }
        }
    }

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

                if projectile.speed > 0.0 {
                    commands.entity(proj_entity).despawn();
                }
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
