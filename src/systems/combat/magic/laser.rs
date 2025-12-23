use crate::components::physics::{Collider, IgnoreGrid};
use crate::components::weapon::{AoEProjectile, Faction, Lifetime, Projectile, WeaponType};
use crate::configs::spells::laser;
use crate::systems::combat::{CombatContext, CombatInputParams};
use crate::visuals::world::spawn_laser_visuals;
use bevy::prelude::*;

pub fn spawn_laser(
    params: &mut CombatInputParams,
    ctx: &CombatContext,
    direction: Vec2,
    angle: f32,
) {
    params
        .commands
        .spawn((
            Transform::from_translation(ctx.spawn_pos.extend(0.0))
                .with_rotation(Quat::from_rotation_z(angle)),
            Visibility::Visible,
            Collider::line(direction, laser::LENGTH, laser::WIDTH / 2.0),
            Projectile {
                kind: WeaponType::Magic,
                damage: laser::DAMAGE * ctx.damage_multiplier,
                speed: 0.0,
                direction,
                owner_entity: ctx.owner_entity,
                is_aoe: true,
                faction: Faction::Player,
                crit_chance: ctx.combat_stats.crit_chance,
                crit_damage: ctx.combat_stats.crit_damage,
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
