//! Projectile visual effects and despawning upon collision

use crate::components::physics::{Collider, IgnoreGrid, Velocity};
use crate::components::weapon::{AoEProjectile, ExplodingProjectile, Lifetime, Projectile};
use crate::systems::combat::{CollisionEvent, CombatResources, PendingDespawn};
use crate::systems::object_pooling::EffectType;
use crate::systems::weapon_visuals::spawn_bolt_explosion_visuals;
use bevy::prelude::*;
use rand::Rng;

/// Handles visual effects and projectile logic (piercing, explosions, despawning) upon collision.
/// Despawns projectiles marked with `PendingDespawn`.
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn projectile_effect_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    projectile_query: Query<(
        &Projectile,
        Option<&ExplodingProjectile>,
        Option<&AoEProjectile>,
        &Transform,
        Option<&PendingDespawn>,
    )>,
    mut res: CombatResources,
) -> Result<(), String> {
    let mut processed_projectiles = Vec::new();

    for event in collision_events.read() {
        // Skip already processed projectiles this frame
        if processed_projectiles.contains(&event.projectile) {
            continue;
        }

        if let Ok((projectile, exploding, aoe, transform, pending)) =
            projectile_query.get(event.projectile)
        {
            // Handle Explosions
            if let Some(exploding) = exploding {
                let mut rng = rand::thread_rng();
                let random_rotation =
                    Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::TAU));
                let spawn_transform = Transform::from_translation(transform.translation)
                    .with_rotation(random_rotation);
                let lifetime = Lifetime {
                    timer: Timer::from_seconds(0.1, TimerMode::Once),
                };

                let req = res.effect_pool.spawn_or_get(
                    &mut commands,
                    EffectType::BoltExplosion,
                    spawn_transform,
                    lifetime,
                );

                // Update the explosion entity components
                commands.entity(req.entity).insert((
                    Velocity::default(),
                    Projectile {
                        kind: projectile.kind,
                        damage: exploding.damage,
                        speed: 0.0,
                        direction: Vec2::ZERO,
                        owner_entity: projectile.owner_entity,
                    },
                    AoEProjectile::default(), // Reset hit list for pooled entities
                    Collider::ball(exploding.radius), // Set correct explosion size
                    IgnoreGrid,               // Reliable AOE coverage
                ));

                if req.is_new {
                    commands.entity(req.entity).with_children(|parent| {
                        spawn_bolt_explosion_visuals(parent, &res.cached_assets, exploding.radius);
                    });
                }
            }

            // Despawn projectiles marked with PendingDespawn (non-AoE projectiles that hit)
            if pending.is_some() && aoe.is_none() {
                commands.entity(event.projectile).despawn();
            }

            processed_projectiles.push(event.projectile);
        }
    }
    Ok(())
}
