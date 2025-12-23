use crate::components::physics::{Collider, IgnoreGrid, Velocity};
use crate::components::weapon::{AoEProjectile, ExplodingProjectile, Lifetime, Projectile};
use crate::systems::combat::{CollisionEvent, PendingDespawn};
use crate::systems::weapon_visuals::spawn_bolt_explosion_visuals;
use bevy::prelude::*;
use rand::Rng;

type ProjectileEffectQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Projectile,
        Option<&'static ExplodingProjectile>,
        Option<&'static AoEProjectile>,
        &'static Transform,
        Option<&'static PendingDespawn>,
    ),
>;

#[allow(clippy::needless_pass_by_value)]
pub fn projectile_effect_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    projectile_query: ProjectileEffectQuery,
    res: Res<crate::resources::cached_assets::CachedAssets>,
) {
    let mut processed_projectiles = Vec::new(); // Still needs mut because we push to it!

    for event in collision_events.read() {
        // Skip already processed projectiles this frame
        if processed_projectiles.contains(&event.projectile) {
            continue;
        }

        if let Ok((projectile, exploding, _aoe, transform, _pending)) =
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

                commands
                    .spawn((
                        Visibility::Visible,
                        spawn_transform,
                        lifetime,
                        Velocity::default(),
                        Projectile {
                            kind: projectile.kind,
                            damage: exploding.damage,
                            speed: 0.0,
                            direction: Vec2::ZERO,
                            owner_entity: projectile.owner_entity,
                            is_aoe: true, // Explosion is AOE
                            faction: projectile.faction,
                            crit_chance: projectile.crit_chance,
                            crit_damage: projectile.crit_damage,
                        },
                        AoEProjectile::default(),         // Reset hit list
                        Collider::ball(exploding.radius), // Set correct explosion size
                        IgnoreGrid,                       // Reliable AOE coverage
                    ))
                    .with_children(|parent| {
                        spawn_bolt_explosion_visuals(parent, &res, exploding.radius);
                    });
            }
            processed_projectiles.push(event.projectile);
        }
    }
}

/// Standalone system to clean up projectiles marked for despawn.
/// This runs after collision detection to ensure projectiles are removed recursively,
/// cleaning up all visual child entities.
pub fn cleanup_pending_despawn(
    mut commands: Commands,
    query: Query<Entity, (With<PendingDespawn>, Without<AoEProjectile>)>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
