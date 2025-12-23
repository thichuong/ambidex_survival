use crate::components::physics::{Collider, IgnoreGrid};
use crate::components::weapon::{AoEProjectile, Faction, Lifetime, Projectile, WeaponType};
use crate::configs::spells::global;
use crate::systems::combat::CombatInputParams;
use crate::systems::weapon_visuals::spawn_global_visuals;
use bevy::prelude::*;

pub fn spawn_global_spell(
    params: &mut CombatInputParams,
    player_entity: Entity,
    player_center: Vec3,
    damage_multiplier: f32,
    crit_chance: f32,
    crit_damage: f32,
) {
    params
        .commands
        .spawn((
            Transform::from_translation(player_center),
            Visibility::Visible,
            Collider::ball(global::RADIUS),
            Projectile {
                kind: WeaponType::Magic,
                damage: global::DAMAGE * damage_multiplier,
                speed: 0.0,
                direction: Vec2::ZERO,
                owner_entity: player_entity,
                is_aoe: true,
                faction: Faction::Player,
                crit_chance,
                crit_damage,
            },
            Lifetime {
                timer: Timer::from_seconds(global::LIFETIME, TimerMode::Once),
            },
            AoEProjectile::default(),
            IgnoreGrid,
        ))
        .with_children(|parent| {
            spawn_global_visuals(parent, &params.cached_assets);
        });
}
