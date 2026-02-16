use crate::components::physics::{Collider, Velocity};
use crate::components::weapon::{ExplodingProjectile, Faction, Lifetime, Projectile, WeaponType};
use crate::configs::spells::energy_bolt;
use crate::systems::combat::{CombatContext, CombatInputParams};
use crate::visuals::world::spawn_energy_bolt_visuals;
use bevy::prelude::*;

pub fn spawn_energy_bolt(
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
            Collider::ball(4.0),
            Velocity {
                linvel: direction * energy_bolt::SPEED,
                angvel: 0.0,
            },
            Projectile {
                kind: WeaponType::Magic,
                damage: energy_bolt::DAMAGE * ctx.damage_multiplier,
                speed: energy_bolt::SPEED,
                direction,
                owner_entity: ctx.owner_entity,
                is_aoe: false, // Initial hit is single-target, explosion is AOE
                faction: Faction::Player,
                crit_chance: ctx.combat_stats.crit_chance,
                crit_damage: ctx.combat_stats.crit_damage,
                lifesteal_efficiency: 1.0,
            },
            Lifetime {
                timer: Timer::from_seconds(energy_bolt::LIFETIME, TimerMode::Once),
            },
            ExplodingProjectile {
                radius: energy_bolt::EXPLOSION_RADIUS,
                damage: energy_bolt::DAMAGE * ctx.damage_multiplier,
            },
        ))
        .with_children(|parent| {
            spawn_energy_bolt_visuals(parent, &params.cached_assets);
        });
}
