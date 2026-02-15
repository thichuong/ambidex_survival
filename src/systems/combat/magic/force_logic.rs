use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, IgnoreGrid, Velocity};
use crate::components::weapon::{
    AoEProjectile, Faction, ForcePull, ForcePush, Lifetime, Projectile, WeaponType,
};
use crate::configs::spells::force;
use crate::systems::combat::{CollisionEvent, CombatContext, CombatInputParams};
use crate::visuals::world::spawn_global_visuals;
use bevy::prelude::*;

pub fn spawn_force_push(params: &mut CombatInputParams, ctx: &CombatContext, faction: Faction) {
    params
        .commands
        .spawn((
            Transform::from_translation(ctx.transform.translation),
            Visibility::Visible,
            Collider::ball(force::RADIUS),
            Projectile {
                kind: WeaponType::Magic,
                damage: force::DAMAGE_BASE * ctx.damage_multiplier,
                speed: 0.0,
                direction: Vec2::ZERO,
                owner_entity: ctx.owner_entity,
                is_aoe: true,
                faction,
                crit_chance: ctx.combat_stats.crit_chance,
                crit_damage: ctx.combat_stats.crit_damage,
            },
            Lifetime {
                timer: Timer::from_seconds(force::LIFETIME, TimerMode::Once),
            },
            AoEProjectile::default(),
            IgnoreGrid,
            ForcePush,
        ))
        .with_children(|parent| {
            // Reusing global visuals for now, maybe customize later
            spawn_global_visuals(parent, &params.cached_assets);
        });
}

pub fn spawn_force_pull(params: &mut CombatInputParams, ctx: &CombatContext, faction: Faction) {
    params
        .commands
        .spawn((
            Transform::from_translation(ctx.transform.translation),
            Visibility::Visible,
            Collider::ball(force::RADIUS),
            Projectile {
                kind: WeaponType::Magic,
                damage: force::DAMAGE_BASE * ctx.damage_multiplier,
                speed: 0.0,
                direction: Vec2::ZERO,
                owner_entity: ctx.owner_entity,
                is_aoe: true,
                faction,
                crit_chance: ctx.combat_stats.crit_chance,
                crit_damage: ctx.combat_stats.crit_damage,
            },
            Lifetime {
                timer: Timer::from_seconds(force::LIFETIME, TimerMode::Once),
            },
            AoEProjectile::default(),
            IgnoreGrid,
            ForcePull,
        ))
        .with_children(|parent| {
            spawn_global_visuals(parent, &params.cached_assets);
        });
}

#[allow(clippy::needless_pass_by_value)]
pub fn force_effect_observer(
    trigger: On<CollisionEvent>,
    projectile_query: Query<(Entity, Option<&ForcePush>, Option<&ForcePull>)>,
    mut target_query: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    let event = trigger.event();

    let Ok((_proj_entity, push, pull)) = projectile_query.get(event.projectile) else {
        return;
    };

    let Ok((target_transform, mut velocity)) = target_query.get_mut(event.target) else {
        return;
    };

    let target_pos = target_transform.translation.truncate();
    let caster_pos = event.position; // Projectile center (where caster was)
    let to_target = target_pos - caster_pos;
    let dir = to_target.normalize_or_zero();

    if push.is_some() {
        // Push away: strength reduces slightly with distance? 
        // User said: "càng đứng gần damage càng lớn". 
        // For physics, let's just apply a strong impulse.
        velocity.linvel += dir * force::PUSH_STRENGTH;
    } else if pull.is_some() {
        // Pull towards: 
        velocity.linvel -= dir * force::PULL_STRENGTH;
    }
}
