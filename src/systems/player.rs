// use bevy::prelude::*; // Was: use bevy::color::palettes::css::AQUA; - Removed unused import
use bevy::prelude::*;

use crate::components::physics::{Collider, Velocity};
use crate::components::player::{Currency, GameCamera, Hand, HandType, Player, PlayerStats};
use crate::components::weapon::{GunState, MagicLoadout, SwordState, Weapon, WeaponType};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            (
                Mesh2d(meshes.add(Circle::new(crate::configs::player::RADIUS))),
                MeshMaterial2d(materials.add(crate::configs::player::COLOR)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            Collider::ball(crate::configs::player::RADIUS),
            Velocity::zero(),
            Currency {
                gold: crate::configs::player::STARTING_GOLD,
            },
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
                MagicLoadout::default(),
                SwordState::default(),
                GunState::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                Transform::default(),
                GlobalTransform::default(),
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
                MagicLoadout::default(),
                SwordState::default(),
                GunState::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                Transform::default(),
                GlobalTransform::default(),
            ));
        });
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn move_player(
    _time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &PlayerStats), With<Player>>,
) -> Result<(), String> {
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
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
pub fn aim_player(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) -> Result<(), String> {
    let (camera, camera_transform) = camera_query
        .single()
        .map_err(|e| format!("Camera not found: {e:?}"))?;

    let window = window_query
        .single()
        .map_err(|e| format!("Window not found: {e:?}"))?;

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        let mut player_transform = player_query
            .single_mut()
            .map_err(|e| format!("Player not found: {e:?}"))?;
        let diff = world_position - player_transform.translation.truncate();
        let angle = diff.y.atan2(diff.x);
        player_transform.rotation = Quat::from_rotation_z(angle);
    }
    Ok(())
}
