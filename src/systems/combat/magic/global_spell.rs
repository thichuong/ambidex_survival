use crate::components::physics::{Collider, IgnoreGrid};
use crate::components::weapon::{AoEProjectile, Faction, Lifetime, Projectile, WeaponType};
use crate::configs::spells::global;
use crate::systems::combat::{CombatContext, CombatInputParams};
use crate::systems::weapon_visuals::spawn_global_visuals;
use bevy::prelude::*;

pub fn spawn_global_spell(params: &mut CombatInputParams, ctx: &CombatContext) {
    params
        .commands
        .spawn((
            Transform::from_translation(ctx.transform.translation),
            Visibility::Visible,
            Collider::ball(global::RADIUS),
            Projectile {
                kind: WeaponType::Magic,
                damage: global::DAMAGE * ctx.damage_multiplier,
                speed: 0.0,
                direction: Vec2::ZERO,
                owner_entity: ctx.owner_entity,
                is_aoe: true,
                faction: Faction::Player,
                crit_chance: ctx.combat_stats.crit_chance,
                crit_damage: ctx.combat_stats.crit_damage,
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
