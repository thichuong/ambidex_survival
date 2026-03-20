// use bevy::prelude::*; // Was: use bevy::color::palettes::css::AQUA; - Removed unused import
use bevy::prelude::*;

use crate::components::physics::Velocity;
use crate::components::player::{Hand, HandType, Player, PlayerStats};
use crate::components::status::UnitStatus;
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
    virtual_input: Res<crate::resources::input_settings::VirtualInput>,
    mut player: Single<(&mut Velocity, &PlayerStats, &UnitStatus), With<Player>>,
) {
    let (ref mut velocity, stats, unit_status) = *player;

    if unit_status.is_rooted() {
        velocity.linvel = Vec2::ZERO;
        return;
    }

    let direction = virtual_input.axis;

    velocity.linvel = direction * stats.speed;
}

#[allow(clippy::needless_pass_by_value)]
pub fn aim_player(
    virtual_input: Res<crate::resources::input_settings::VirtualInput>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let world_position = virtual_input.cursor_world;
    let diff = world_position - player.translation.truncate();
    let angle = diff.y.atan2(diff.x);
    player.rotation = Quat::from_rotation_z(angle);
}
