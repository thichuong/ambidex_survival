use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, IgnoreGrid, Velocity};
use crate::components::status::UnitStatus;
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
    projectile_query: Query<(Entity, Option<&ForcePush>, Option<&ForcePull>, &Transform)>,
    mut target_query: Query<(&Transform, &mut Velocity, &mut UnitStatus), With<Enemy>>,
) {
    let event = trigger.event();

    let Ok((_entity, push, pull, proj_transform)) = projectile_query.get(event.projectile) else {
        return;
    };

    let Ok((target_transform, _velocity, mut status)) = target_query.get_mut(event.target)
    else {
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
        // Push away: strength reduces slightly with distance?
        // User said: "càng gần đẩy càng xa" -> Closer = Stronger
        // Formula: Base * (1.0 + (1.0 - dist/Rad)) -> range [1.0, 2.0]
        let factor = 1.0 + (1.0 - (distance / force::RADIUS).min(1.0)).max(0.0);
        let speed = force::BASE_PUSH_SPEED * factor;
        
        status.add(crate::components::status::StatusEffect::ForcedMovement {
            timer: Timer::from_seconds(force::PUSH_DURATION, TimerMode::Once),
            direction: dir,
            speed,
            move_type: crate::components::status::ForceType::Push,
        });
        
    } else if pull.is_some() {
        // Pull towards:
        // User said: "Cách càng xa kéo càng xa" -> Farther = Stronger
        // Formula: Base * (1.0 + dist/Rad) -> range [1.0, 2.0]
        let factor = 1.0 + (distance / force::RADIUS).min(1.0);
        let speed = force::BASE_PULL_SPEED * factor;
        
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
    use crate::components::physics::Velocity;
    use crate::components::status::StatusEffect;

    #[test]
    fn test_force_push_logic() {
        let mut app = App::new();
        app.add_event::<CollisionEvent>();

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
        if let StatusEffect::ForcedMovement { direction, speed, move_type, .. } = effect {
             assert_eq!(move_type, &crate::components::status::ForceType::Push);
             assert!(direction.x > 0.0); // Pushed right
             // Distance 100, Radius 800. Factor = 1 + (1 - 0.125) = 1.875
             // Base 800 * 1.875 = 1500
             assert!(speed > &800.0);
        } else {
            panic!("Expected ForcedMovement status");
        }
    }

    #[test]
    fn test_force_pull_logic() {
        let mut app = App::new();
        app.add_event::<CollisionEvent>();
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
        if let StatusEffect::ForcedMovement { direction, speed, move_type, .. } = effect {
             assert_eq!(move_type, &crate::components::status::ForceType::Pull);
             assert!(direction.x < 0.0); // Pulled left
             // Distance 100, Radius 800. Factor = 1 + 0.125 = 1.125
             // Base 800 * 1.125 = 900
             assert!(speed > &800.0);
        } else {
            panic!("Expected ForcedMovement status");
        }
    }
}

