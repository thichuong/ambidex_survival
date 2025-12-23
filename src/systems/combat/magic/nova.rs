use crate::components::weapon::{AoEProjectile, Faction, Lifetime, Projectile, WeaponType};
use crate::configs::spells::nova;
use crate::systems::combat::{CombatContext, CombatInputParams};
use crate::systems::weapon_visuals::spawn_nova_visuals;
use bevy::prelude::*;

pub fn spawn_nova(params: &mut CombatInputParams, ctx: &CombatContext, explosion_pos: Vec3) {
    params
        .commands
        .spawn((
            Transform::from_translation(explosion_pos),
            Visibility::Visible,
            crate::components::physics::Collider::ball(nova::RADIUS),
            Projectile {
                kind: WeaponType::Magic,
                damage: nova::DAMAGE * ctx.damage_multiplier,
                speed: 0.0,
                direction: Vec2::ZERO,
                owner_entity: ctx.owner_entity,
                is_aoe: true,
                faction: Faction::Player,
                crit_chance: ctx.combat_stats.crit_chance,
                crit_damage: ctx.combat_stats.crit_damage,
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
