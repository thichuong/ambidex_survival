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
    _key_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    hand_query: Query<(&GlobalTransform, &Hand)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    let (player_entity, _player_transform) = if let Ok(p) = player_query.get_single() {
        p
    } else {
        return;
    };

    let left_attack = mouse_input.just_pressed(MouseButton::Left);
    let right_attack = mouse_input.just_pressed(MouseButton::Right);

    for (hand_transform, hand) in hand_query.iter() {
        let should_attack = match hand.hand_type {
            HandType::Left => left_attack,
            HandType::Right => right_attack,
        };

        if should_attack {
            if let Some(weapon_type) = hand.equipped_weapon {
                fire_weapon(
                    &mut commands,
                    weapon_type,
                    hand_transform.translation().truncate(),
                    cursor_pos,
                    player_entity,
                    &mut meshes,
                    &mut materials,
                );
            }
        }
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
            // Shuriken is square or diamond
            // Mesh::from(Rectangle::new(10.0, 10.0))
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Rectangle::new(10.0, 10.0))).into(),
                    material: materials.add(Color::from(AZURE)), // Light Blue
                    transform: Transform::from_translation(spawn_pos.extend(0.0)),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(5.0),
                Sensor,
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
            // Sword Slash Visual
            // Use a Rectangle or Ellipse to approximate slash arc? Or just a Rectangle.
            let angle = direction.y.atan2(direction.x);

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Rectangle::new(40.0, 60.0))).into(), // Slash area
                    material: materials.add(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.5))),
                    transform: Transform::from_translation(spawn_pos.extend(1.0))
                        .with_rotation(Quat::from_rotation_z(angle)), // Align with direction
                    ..default()
                },
                Sensor,
                Collider::cuboid(20.0, 30.0), // Half extents
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
                println!("Hit! Enemy HP: {}", enemy.health);

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
