use bevy::color::palettes::css::AQUA;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::components::player::{GameCamera, Hand, HandType, Player};
use crate::components::weapon::{GunState, MagicLoadout, SwordState, Weapon, WeaponType};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
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
        .with_children(|parent| {
            // Left Hand
            parent.spawn((
                Hand {
                    side: HandType::Left,
                    offset: Vec3::new(-20.0, 20.0, 1.0),
                    equipped_weapon: Some(WeaponType::Shuriken), // Default Left
                },
                Weapon {
                    kind: WeaponType::Shuriken,
                    cooldown: crate::configs::weapons::shuriken::COOLDOWN,
                    skill_cooldown: crate::configs::weapons::shuriken::SKILL_COOLDOWN,
                    ..default()
                },
                MagicLoadout::default(),
                SwordState::default(),
                GunState::default(),
                SpatialBundle::default(),
            ));

            // Right Hand
            parent.spawn((
                Hand {
                    side: HandType::Right,
                    offset: Vec3::new(20.0, 20.0, 1.0),
                    equipped_weapon: Some(WeaponType::Sword), // Default Right
                },
                Weapon {
                    kind: WeaponType::Sword,
                    ..default()
                },
                MagicLoadout::default(),
                SwordState::default(),
                GunState::default(),
                SpatialBundle::default(),
            ));
        });
}

#[allow(clippy::needless_pass_by_value)]
pub fn move_player(
    _time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in &mut query {
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

#[allow(clippy::needless_pass_by_value)]
pub fn aim_player(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let Ok(window) = window_query.get_single() else {
        return;
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for mut player_transform in &mut player_query {
            let diff = world_position - player_transform.translation.truncate();
            let angle = diff.y.atan2(diff.x);
            player_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
