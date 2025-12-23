use bevy::prelude::*;
use rand::Rng;

use crate::components::enemy::{EliteAi, EliteEnemy};
use crate::components::physics::{Collider, Velocity};
use crate::components::player::{Player, PlayerStats};
use crate::components::weapon::{Faction, Lifetime, Projectile, WeaponType};
use crate::configs::weapons::shuriken;
use crate::resources::cached_assets::CachedAssets;
use crate::systems::weapon_visuals::spawn_elite_shuriken_visuals;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn elite_ai_system(
    mut commands: Commands,
    time: Res<Time>,
    mut elite_query: Query<(Entity, &mut Transform, &mut EliteAi), With<EliteEnemy>>,
    player_query: Single<(&Transform, &PlayerStats), (With<Player>, Without<EliteEnemy>)>,
    projectile_query: Query<
        (Entity, &GlobalTransform, &Projectile, &Lifetime),
        Without<EliteEnemy>,
    >,
    cached_assets: Res<CachedAssets>,
) {
    let (player_transform, player_stats) = *player_query;
    let player_pos = player_transform.translation.truncate();

    for (elite_entity, mut elite_transform, mut ai) in &mut elite_query {
        ai.shuriken_timer.tick(time.delta());
        ai.teleport_timer.tick(time.delta());

        let elite_pos = elite_transform.translation.truncate();

        // Fire Shuriken
        if ai.shuriken_timer.just_finished() {
            let mut rng = rand::thread_rng();
            let base_dir = (player_pos - elite_pos).normalize_or_zero();

            // Apply spread (high spread as requested: "có độ lệch cao")
            let spread = rng.gen_range(
                -crate::configs::enemy::ELITE_SHURIKEN_SPREAD
                    ..crate::configs::enemy::ELITE_SHURIKEN_SPREAD,
            );
            let angle = base_dir.y.atan2(base_dir.x) + spread;
            let direction = Vec2::new(angle.cos(), angle.sin());

            // Use the shared fire_shuriken logic to handle limits and spawning
            // We need to wrap our current system context into something that can call fire_shuriken
            // or just use the logic directly. The logic involves projectile_query and commands.
            // Since fire_shuriken is in another module, we might need to make it pub or move it.
            // I'll make it pub in shuriken.rs and call it.

            // However, fire_shuriken takes CombatInputParams which is a massive struct.
            // Elite AI doesn't have all those params.
            // Let's refactor fire_shuriken to be more flexible or just copy the limit logic here.
            // Given the complexity of CombatInputParams, manually implementing the limit here for now
            // is safer unless I refactor fire_shuriken to take smaller pieces.

            // Re-evaluating: I'll just keep the manual spawn here but ADD the limit logic.
            let mut elite_shurikens: Vec<(Entity, f32)> = projectile_query
                .iter()
                .filter(|(_, _, p, _)| {
                    p.kind == WeaponType::Shuriken && p.owner_entity == elite_entity
                })
                .map(|(e, _, _, l)| (e, l.timer.remaining_secs()))
                .collect();

            if elite_shurikens.len() >= shuriken::MAX_COUNT_ELITE {
                elite_shurikens
                    .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                if let Some((oldest_entity, _)) = elite_shurikens.first() {
                    commands.entity(*oldest_entity).despawn();
                }
            }

            commands
                .spawn((
                    Transform::from_translation(elite_pos.extend(0.0)),
                    Visibility::Visible,
                    Collider::ball(shuriken::COLLIDER_RADIUS),
                    Velocity {
                        linvel: direction * shuriken::SPEED,
                        angvel: shuriken::ROTATION_SPEED,
                    },
                    Projectile {
                        kind: WeaponType::Shuriken,
                        damage: shuriken::DAMAGE * player_stats.damage_multiplier,
                        speed: shuriken::SPEED,
                        direction,
                        owner_entity: elite_entity,
                        is_aoe: false,
                        faction: Faction::Enemy,
                    },
                    Lifetime {
                        timer: Timer::from_seconds(shuriken::LIFETIME, TimerMode::Once),
                    },
                ))
                .with_children(|parent| {
                    spawn_elite_shuriken_visuals(parent, &cached_assets);
                });
        }

        // Teleport
        if ai.teleport_timer.just_finished() {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(crate::configs::enemy::ELITE_TELEPORT_CHANCE.into()) {
                // Find shuriken closest to player
                let mut closest_shuriken: Option<(Entity, Vec3)> = None;
                let mut min_dist_sq = f32::MAX;

                for (proj_entity, proj_tf, proj, _) in &projectile_query {
                    if proj.kind == WeaponType::Shuriken && proj.owner_entity == elite_entity {
                        let shuriken_pos = proj_tf.translation();
                        let dist_sq = shuriken_pos.truncate().distance_squared(player_pos);
                        if dist_sq < min_dist_sq {
                            min_dist_sq = dist_sq;
                            closest_shuriken = Some((proj_entity, shuriken_pos));
                        }
                    }
                }

                if let Some((shuriken_entity, shuriken_location)) = closest_shuriken {
                    // Teleport visuals at OLD position
                    commands.spawn((
                        Mesh2d(cached_assets.unit_circle.clone()),
                        MeshMaterial2d(cached_assets.mat_purple_40.clone()),
                        Transform::from_translation(elite_transform.translation)
                            .with_scale(Vec3::splat(shuriken::TELEPORT_VISUAL_SCALE)),
                        Lifetime {
                            timer: Timer::from_seconds(
                                shuriken::TELEPORT_VISUAL_LIFETIME,
                                TimerMode::Once,
                            ),
                        },
                    ));

                    // Update position
                    elite_transform.translation = shuriken_location;

                    // Teleport visuals at NEW position
                    commands.spawn((
                        Mesh2d(cached_assets.unit_circle.clone()),
                        MeshMaterial2d(cached_assets.mat_purple_40.clone()),
                        Transform::from_translation(shuriken_location)
                            .with_scale(Vec3::splat(shuriken::TELEPORT_VISUAL_SCALE)),
                        Lifetime {
                            timer: Timer::from_seconds(
                                shuriken::TELEPORT_VISUAL_LIFETIME,
                                TimerMode::Once,
                            ),
                        },
                    ));

                    // Despawn the shuriken used for teleport
                    commands.entity(shuriken_entity).despawn();
                }
            }
        }
    }
}
