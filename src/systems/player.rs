use bevy::color::palettes::css::{AQUA, ORANGE, YELLOW};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::components::player::{GameCamera, Hand, HandType, Player};
use crate::components::weapon::WeaponType;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_id = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(Circle::new(20.0))).into(),
                material: materials.add(Color::from(AQUA)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(20.0),
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            Player::default(),
        ))
        .id();

    // Spawn Hands
    let hand_mesh = meshes.add(Mesh::from(Circle::new(5.0)));

    // Left Hand - YELLOW
    let left_hand_mat = materials.add(Color::from(YELLOW));
    let left_hand = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: hand_mesh.clone().into(),
                material: left_hand_mat,
                transform: Transform::from_xyz(25.0, 10.0, -0.1), // Child transform relative to parent
                ..default()
            },
            Hand {
                hand_type: HandType::Left,
                offset: Vec3::new(25.0, 10.0, 0.0),
                equipped_weapon: Some(WeaponType::Shuriken),
            },
        ))
        .id();

    // Right Hand - ORANGE
    let right_hand_mat = materials.add(Color::from(ORANGE));
    let right_hand = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: hand_mesh.into(),
                material: right_hand_mat,
                transform: Transform::from_xyz(25.0, -10.0, -0.1),
                ..default()
            },
            Hand {
                hand_type: HandType::Right,
                offset: Vec3::new(25.0, -10.0, 0.0),
                equipped_weapon: Some(WeaponType::Sword),
            },
        ))
        .id();

    commands
        .entity(player_id)
        .push_children(&[left_hand, right_hand]);
}

pub fn move_player(
    _time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        velocity.linvel = direction * player.speed;
    }
}

pub fn aim_player(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
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

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for mut player_transform in player_query.iter_mut() {
            let diff = world_position - player_transform.translation.truncate();
            let angle = diff.y.atan2(diff.x);
            player_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
