// use crate::components::enemy::Enemy; // Removed unused import
use crate::components::physics::{Collider, IgnoreGrid, Velocity};
use crate::components::status::UnitStatus;
use crate::components::weapon::{
    AoEProjectile, Faction, ForcePull, ForcePush, Lifetime, Projectile, WeaponType,
};
use crate::configs::spells::force;
use crate::systems::combat::{CollisionEvent, CombatContext, CombatInputParams};
use crate::visuals::world::{spawn_force_pull_visuals, spawn_force_push_visuals};
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
                lifesteal_efficiency: 0.5,
            },
            Lifetime {
                timer: Timer::from_seconds(force::LIFETIME, TimerMode::Once),
            },
            AoEProjectile::default(),
            IgnoreGrid,
            ForcePush,
            crate::components::weapon::DistanceDamageBonus {
                max_bonus: force::DAMAGE_BONUS_MAX,
                optimal_distance: 0.0,
                radius: force::RADIUS,
            },
        ))
        .with_children(|parent| {
            spawn_force_push_visuals(parent, &params.cached_assets);
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
                lifesteal_efficiency: 0.5,
            },
            Lifetime {
                timer: Timer::from_seconds(force::LIFETIME, TimerMode::Once),
            },
            AoEProjectile::default(),
            IgnoreGrid,
            ForcePull,
            crate::components::weapon::DistanceDamageBonus {
                max_bonus: force::DAMAGE_BONUS_MAX,
                optimal_distance: 1.0,
                radius: force::RADIUS,
            },
        ))
        .with_children(|parent| {
            spawn_force_pull_visuals(parent, &params.cached_assets);
        });
}

#[allow(clippy::needless_pass_by_value)]
pub fn force_effect_observer(
    trigger: On<CollisionEvent>,
    projectile_query: Query<(Entity, Option<&ForcePush>, Option<&ForcePull>, &Transform)>,
    mut target_query: Query<(&Transform, &mut Velocity, &mut UnitStatus)>,
) {
    let event = trigger.event();

    let Ok((_entity, push, pull, proj_transform)) = projectile_query.get(event.projectile) else {
        return;
    };

    let Ok((target_transform, _velocity, mut status)) = target_query.get_mut(event.target) else {
        return;
    };

    let target_pos = target_transform.translation.truncate();
    let caster_pos = proj_transform.translation.truncate(); // Projectile center
    let to_target = target_pos - caster_pos;
    let distance = to_target.length();
    let dir = to_target.normalize_or_zero();

    // Apply Rooted status for duration so they don't fight the push/pull
    // Actually ForcedMovement implies no control, but let's be explicit if wanted.
    // However, status system handles velocity override now.

    if push.is_some() {
        // Push away:
        // Formula: (Range - Distance) * ScalingFactor
        // Closer to caster = Stronger push
        let speed = (force::RADIUS - distance).max(0.0) * force::PUSH_SPEED_FACTOR;

        status.add(crate::components::status::StatusEffect::ForcedMovement {
            timer: Timer::from_seconds(force::PUSH_DURATION, TimerMode::Once),
            direction: dir,
            speed,
            move_type: crate::components::status::ForceType::Push,
        });
    } else if pull.is_some() {
        // Pull towards:
        // Formula: Distance * ScalingFactor
        // Farther from caster = Stronger pull
        let speed = distance * force::PULL_SPEED_FACTOR;

        status.add(crate::components::status::StatusEffect::ForcedMovement {
            timer: Timer::from_seconds(force::PULL_DURATION, TimerMode::Once),
            direction: -dir, // Pull towards caster
            speed,
            move_type: crate::components::status::ForceType::Pull,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::enemy::Enemy;
    use crate::components::physics::Velocity;
    use crate::components::status::StatusEffect;

    #[test]
    fn test_force_push_logic() {
        let mut app = App::new();
        app.add_message::<CollisionEvent>();

        // Mock the observer registration manually since we don't have the full plugin here
        app.add_observer(force_effect_observer);

        // Spawn Enemy
        let enemy = app
            .world_mut()
            .spawn((
                Transform::from_xyz(100.0, 0.0, 0.0),
                Velocity::default(),
                UnitStatus::default(),
                Enemy::default(),
            ))
            .id();

        // Spawn Projectile (ForcePush) at Origin
        let projectile = app
            .world_mut()
            .spawn((Transform::from_xyz(0.0, 0.0, 0.0), ForcePush))
            .id();

        // Trigger Collision
        app.world_mut().trigger(CollisionEvent {
            projectile,
            target: enemy,
            position: Vec2::new(100.0, 0.0), // Hit position
        });

        // Check Enemy Status
        let status = app.world().get::<UnitStatus>(enemy).unwrap();

        // Should have ForcedMovement
        let effect = status.effects.last().unwrap();
        if let StatusEffect::ForcedMovement {
            direction,
            speed,
            move_type,
            ..
        } = effect
        {
            assert_eq!(move_type, &crate::components::status::ForceType::Push);
            assert!(direction.x > 0.0); // Pushed right
            assert!(direction.x > 0.0); // Pushed right
            // Distance 100, Radius 800.
            // Speed = (800 - 100) * 1.0 = 700
            assert!(speed > &650.0 && speed < &750.0);
        } else {
            panic!("Expected ForcedMovement status");
        }
    }

    #[test]
    fn test_force_pull_logic() {
        let mut app = App::new();
        app.add_message::<CollisionEvent>();
        app.add_observer(force_effect_observer);

        // Spawn Enemy at (100, 0)
        let enemy = app
            .world_mut()
            .spawn((
                Transform::from_xyz(100.0, 0.0, 0.0),
                Velocity::default(),
                UnitStatus::default(),
                Enemy::default(),
            ))
            .id();

        // Spawn Projectile (ForcePull) at Origin
        let projectile = app
            .world_mut()
            .spawn((Transform::from_xyz(0.0, 0.0, 0.0), ForcePull))
            .id();

        // Trigger Collision
        app.world_mut().trigger(CollisionEvent {
            projectile,
            target: enemy,
            position: Vec2::new(100.0, 0.0),
        });

        // Check Enemy Status
        let status = app.world().get::<UnitStatus>(enemy).unwrap();

        let effect = status.effects.last().unwrap();
        if let StatusEffect::ForcedMovement {
            direction,
            speed,
            move_type,
            ..
        } = effect
        {
            assert_eq!(move_type, &crate::components::status::ForceType::Pull);
            assert!(direction.x < 0.0); // Pulled left
            assert!(direction.x < 0.0); // Pulled left
            // Distance 100, Speed Factor 1.4.
            // Speed = 100 * 1.4 = 140
            assert!(speed > &135.0 && speed < &145.0);
        } else {
            panic!("Expected ForcedMovement status");
        }
    }
}
