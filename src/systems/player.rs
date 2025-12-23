// use bevy::prelude::*; // Was: use bevy::color::palettes::css::AQUA; - Removed unused import
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::physics::Velocity;
use crate::components::player::{GameCamera, Hand, HandType, Player, PlayerStats};
use crate::components::weapon::{Weapon, WeaponType};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(crate::configs::player::RADIUS))),
            MeshMaterial2d(materials.add(crate::configs::player::COLOR)),
            Player,
        ))
        .with_children(|parent| {
            // Left Hand
            parent.spawn((
                Hand {
                    side: HandType::Left,
                    equipped_weapon: Some(WeaponType::Shuriken), // Default Left
                },
                Weapon {
                    kind: WeaponType::Shuriken,
                    cooldown: crate::configs::weapons::shuriken::COOLDOWN,
                    skill_cooldown: crate::configs::weapons::shuriken::SKILL_COOLDOWN,
                    ..default()
                },
            ));

            // Right Hand
            parent.spawn((
                Hand {
                    side: HandType::Right,
                    equipped_weapon: Some(WeaponType::Sword), // Default Right
                },
                Weapon {
                    kind: WeaponType::Sword,
                    ..default()
                },
            ));
        });
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut Velocity, &PlayerStats), With<Player>>,
) {
    let (ref mut velocity, stats) = *player;
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

    velocity.linvel = direction * stats.speed;
}

#[allow(clippy::needless_pass_by_value)]
pub fn aim_player(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let (camera, camera_transform) = *camera;

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        let diff = world_position - player.translation.truncate();
        let angle = diff.y.atan2(diff.x);
        player.rotation = Quat::from_rotation_z(angle);
    }
}
