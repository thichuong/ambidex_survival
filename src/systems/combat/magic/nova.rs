use crate::components::weapon::{AoEProjectile, Lifetime, Projectile, WeaponType};
use crate::configs::spells::nova;
use crate::systems::combat::CombatInputParams;
use crate::systems::weapon_visuals::spawn_nova_visuals;
use bevy::prelude::*;

pub fn spawn_nova(
    params: &mut CombatInputParams,
    player_entity: Entity,
    player_center: Vec3,
    damage_multiplier: f32,
) {
    params
        .commands
        .spawn((
            Transform::from_translation(player_center),
            Visibility::Visible,
            crate::components::physics::Collider::ball(nova::RADIUS),
            Projectile {
                kind: WeaponType::Magic,
                damage: nova::DAMAGE * damage_multiplier,
                speed: 0.0,
                direction: Vec2::ZERO,
                owner_entity: player_entity,
                is_aoe: true,
            },
            Lifetime {
                timer: Timer::from_seconds(nova::LIFETIME, TimerMode::Once),
            },
            AoEProjectile::default(),
        ))
        .with_children(|parent| {
            spawn_nova_visuals(parent, &params.cached_assets);
        });
}
