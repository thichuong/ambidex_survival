use crate::components::enemy::Enemy;
use crate::components::player::{GameCamera, Hand, HandType, Player};
use crate::components::weapon::{Lifetime, Projectile, WeaponType};
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn handle_combat_input(
    mut commands: Commands,
    _time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    hand_query: Query<(&GlobalTransform, &Hand)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    projectile_query: Query<(Entity, &Transform, &Projectile), Without<Player>>,
) {
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

    let left_click = mouse_input.just_pressed(MouseButton::Left);
    let right_click = mouse_input.just_pressed(MouseButton::Right);

    let q_key = key_input.just_pressed(KeyCode::KeyQ);
    let e_key = key_input.just_pressed(KeyCode::KeyE);

    for (hand_transform, hand) in hand_query.iter() {
        let hand_pos = hand_transform.translation().truncate();

        // --- Basic Attacks (Mouse) ---
        let should_attack = match hand.hand_type {
            HandType::Left => left_click,
            HandType::Right => right_click,
        };

        if should_attack {
            if let Some(weapon_type) = hand.equipped_weapon {
                fire_weapon(
                    &mut commands,
                    weapon_type,
                    hand_pos,
                    cursor_pos,
                    player_entity,
                    &mut meshes,
                    &mut materials,
                );
            }
        }

        // --- Skills (Q/E) ---
        let should_use_skill = match hand.hand_type {
            HandType::Left => q_key,
            HandType::Right => e_key,
        };

        if should_use_skill {
            if let Some(weapon_type) = hand.equipped_weapon {
                perform_skill(
                    &mut commands,
                    weapon_type,
                    player_entity,
                    &mut player_transform,
                    cursor_pos,
                    hand_pos,
                    &mut meshes,
                    &mut materials,
                    &projectile_query,
                );
            }
        }
    }
}

fn perform_skill(
    commands: &mut Commands,
    weapon_type: WeaponType,
    player_entity: Entity,
    player_transform: &mut Transform,
    cursor_pos: Vec2,
    spawn_pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    projectile_query: &Query<(Entity, &Transform, &Projectile), Without<Player>>,
) {
    match weapon_type {
        WeaponType::Shuriken => {
            // Skill: Teleport to nearest projectile
            let mut closest_proj: Option<(Entity, Vec3)> = None;
            let mut min_dist_sq = f32::MAX;

            for (entity, proj_tf, proj) in projectile_query.iter() {
                // Ideally check if projectile is a Shuriken (need Projectile data for this later)
                if proj.owner_entity == player_entity {
                    let dist_sq = proj_tf.translation.truncate().distance_squared(cursor_pos);
                    if dist_sq < min_dist_sq {
                        min_dist_sq = dist_sq;
                        closest_proj = Some((entity, proj_tf.translation));
                    }
                }
            }

            if let Some((entity, location)) = closest_proj {
                player_transform.translation = location;
                commands.entity(entity).despawn_recursive();
                println!("Skill: Shuriken Teleport!");
            }
        }
        WeaponType::Sword => {
            // Skill: Spin Attack (360 degrees)
            println!("Skill: Sword Spin!");
            for i in 0..8 {
                let angle = (i as f32) * (std::f32::consts::PI * 2.0 / 8.0);
                let dir = Vec2::new(angle.cos(), angle.sin());

                // Reuse fire logic or custom spawn
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(Rectangle::new(40.0, 60.0))).into(),
                        material: materials.add(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.5))),
                        transform: Transform::from_translation(spawn_pos.extend(1.0))
                            .with_rotation(Quat::from_rotation_z(angle)),
                        ..default()
                    },
                    Sensor,
                    Collider::cuboid(20.0, 30.0),
                    Projectile {
                        damage: 10.0,
                        speed: 200.0, // Moving slash
                        direction: dir,
                        owner_entity: player_entity,
                    },
                    Velocity {
                        linvel: dir * 200.0,
                        angvel: 0.0,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(0.3, TimerMode::Once),
                    },
                ));
            }
        }
        WeaponType::Bow => {
            // Skill: Multishot (Spread)
            println!("Skill: Bow Multishot!");
            let base_dir = (cursor_pos - spawn_pos).normalize_or_zero();
            let base_angle = base_dir.y.atan2(base_dir.x);

            let spread_angles = [-0.3, 0.0, 0.3]; // Radians offset

            for offset in spread_angles.iter() {
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
            let angle = direction.y.atan2(direction.x);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Rectangle::new(40.0, 60.0))).into(),
                    material: materials.add(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.5))),
                    transform: Transform::from_translation(spawn_pos.extend(1.0))
                        .with_rotation(Quat::from_rotation_z(angle)),
                    ..default()
                },
                Sensor,
                Collider::cuboid(20.0, 30.0),
                Projectile {
                    damage: 20.0,
                    speed: 0.0,
                    direction: Vec2::ZERO,
                    owner_entity: owner,
                },
                Lifetime {
                    timer: Timer::from_seconds(0.2, TimerMode::Once),
                },
            ));
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
    mut enemy_query: Query<(Entity, &mut Enemy)>,
) {
    for (proj_entity, projectile, _proj_tf) in projectile_query.iter() {
        for (enemy_entity, mut enemy) in enemy_query.iter_mut() {
            if rapier_context.intersection_pair(proj_entity, enemy_entity) == Some(true) {
                // Hit!
                enemy.health -= projectile.damage;

                if projectile.speed > 0.0 {
                    commands.entity(proj_entity).despawn();
                }

                if enemy.health <= 0.0 {
                    commands.entity(enemy_entity).despawn_recursive();
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
