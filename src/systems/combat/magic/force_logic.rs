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

    let Ok((target_transform, mut velocity, mut status)) = target_query.get_mut(event.target)
    else {
        return;
    };

    let target_pos = target_transform.translation.truncate();
    let caster_pos = proj_transform.translation.truncate(); // Projectile center
    let to_target = target_pos - caster_pos;
    let dir = to_target.normalize_or_zero();

    // Apply Rooted status for 0.5s so they don't fight the push/pull
    status.root(0.5);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::physics::Velocity;

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
                Enemy,
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

        // Check Enemy Velocity
        let velocity = app.world().get::<Velocity>(enemy).unwrap();

        // Should be pushed right (positive X)
        assert!(velocity.linvel.x > 0.0);
        assert_eq!(velocity.linvel.y, 0.0);

        // Check Rooted status
        let status = app.world().get::<UnitStatus>(enemy).unwrap();
        assert!(status.is_rooted());
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
                Enemy,
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

        // Check Enemy Velocity
        let velocity = app.world().get::<Velocity>(enemy).unwrap();

        // Should be pulled left (negative X) towards origin
        assert!(velocity.linvel.x < 0.0);
        assert_eq!(velocity.linvel.y, 0.0);
    }
}
