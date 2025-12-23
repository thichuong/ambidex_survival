use crate::components::physics::{Collider, IgnoreGrid};
use crate::components::weapon::{AoEProjectile, Faction, Lifetime, Projectile, WeaponType};
use crate::configs::spells::laser;
use crate::systems::combat::CombatInputParams;
use crate::systems::weapon_visuals::spawn_laser_visuals;
use bevy::prelude::*;

pub fn spawn_laser(
    params: &mut CombatInputParams,
    player_entity: Entity,
    spawn_pos: Vec2,
    direction: Vec2,
    angle: f32,
    damage_multiplier: f32,
    crit_chance: f32,
    crit_damage: f32,
) {
    params
        .commands
        .spawn((
            Transform::from_translation(spawn_pos.extend(0.0))
                .with_rotation(Quat::from_rotation_z(angle)),
            Visibility::Visible,
            Collider::line(direction, laser::LENGTH, laser::WIDTH / 2.0),
            Projectile {
                kind: WeaponType::Magic,
                damage: laser::DAMAGE * damage_multiplier,
                speed: 0.0,
                direction,
                owner_entity: player_entity,
                is_aoe: true,
                faction: Faction::Player,
                crit_chance,
                crit_damage,
            },
            Lifetime {
                timer: Timer::from_seconds(laser::LIFETIME, TimerMode::Once),
            },
            AoEProjectile::default(),
            IgnoreGrid,
        ))
        .with_children(|parent| {
            spawn_laser_visuals(parent, &params.cached_assets);
        });
}
