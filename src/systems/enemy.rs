use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::components::enemy::Enemy;
use crate::components::player::Player;

pub fn spawn_test_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Circle::new(20.0))).into(),
            material: materials.add(Color::from(RED)), // Use solid red for now as strokes aren't native to Meshes easily without custom shader
            transform: Transform::from_xyz(200.0, 200.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(20.0),
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        },
        Enemy::default(),
    ));
}

pub fn enemy_chase_player(
    mut enemy_query: Query<(&mut Velocity, &Transform, &Enemy)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = if let Ok(p) = player_query.get_single() {
        p
    } else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (mut velocity, transform, enemy) in enemy_query.iter_mut() {
        let pos = transform.translation.truncate();
        let dir = (player_pos - pos).normalize_or_zero();
        velocity.linvel = dir * enemy.speed;
    }
}
