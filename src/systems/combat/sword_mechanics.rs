use super::DamageEvent;
use crate::components::enemy::Enemy;
use crate::components::player::{CombatStats, Hand, Health, Player};
use crate::components::weapon::{SwingState, SwordSwing};
use bevy::prelude::*;
use rand::Rng;

#[allow(
    clippy::unnecessary_wraps,
    clippy::needless_pass_by_value,
    clippy::too_many_arguments
)]
pub fn update_sword_mechanics(
    mut commands: Commands,
    time: Res<Time>,
    mut sword_query: Query<(Entity, &mut SwordSwing, &mut Transform)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<SwordSwing>>,
    hand_query: Query<&GlobalTransform, With<Hand>>,
    mut player_query: Query<(&mut Health, &CombatStats), With<Player>>,
) {
    for (entity, mut swing, mut transform) in &mut sword_query {
        if let Ok(hand_transform) = hand_query.get(swing.hand_entity) {
            transform.translation = hand_transform.translation().truncate().extend(0.0);
        }

        swing.timer.tick(time.delta());

        match swing.state {
            SwingState::Swinging => {
                let progress = swing.timer.fraction();
                // If direction is 1.0 (CCW), goes from -PI/2 to +PI/2
                // If direction is -1.0 (CW), should go from +PI/2 to -PI/2?
                // Wait, (progress - 0.5) is [-0.5, 0.5].
                // * PI => [-PI/2, PI/2].
                // If we want reversed, we negate this offset.
                let offset = (progress - 0.5) * std::f32::consts::PI * swing.swing_direction;
                let current_angle = swing.base_angle + offset;
                transform.rotation = Quat::from_rotation_z(current_angle);

                if !swing.damage_dealt {
                    let sweep_radius = swing.range;

                    for (enemy_entity, enemy_tf, mut enemy) in &mut enemy_query {
                        let to_enemy =
                            enemy_tf.translation.truncate() - transform.translation.truncate();
                        let distance = to_enemy.length();

                        if distance <= sweep_radius && distance > 0.0 {
                            let enemy_direction = to_enemy / distance;
                            let base_direction =
                                Vec2::new(swing.base_angle.cos(), swing.base_angle.sin());
                            let dot = enemy_direction.dot(base_direction);

                            if dot > 0.0 {
                                let mut final_damage = swing.damage;
                                let mut is_crit = false;

                                if let Some((mut health, stats)) = player_query.iter_mut().next() {
                                    // Crit Check
                                    let mut rng = rand::thread_rng();
                                    if rng.gen_range(0.0..1.0) < stats.crit_chance {
                                        final_damage *= stats.crit_damage;
                                        is_crit = true;
                                    }

                                    // Lifesteal
                                    if stats.lifesteal > 0.0 {
                                        let heal = final_damage * stats.lifesteal;
                                        health.current = (health.current + heal).min(health.max);
                                    }
                                }

                                enemy.health -= final_damage;

                                commands.trigger(DamageEvent {
                                    damage: final_damage,
                                    position: enemy_tf.translation.truncate(),
                                    is_crit,
                                });

                                if enemy.health <= 0.0 {
                                    commands.trigger(crate::systems::combat::EnemyDeathEvent {
                                        entity: enemy_entity,
                                        position: enemy_tf.translation.truncate(),
                                    });
                                }
                            }
                        }
                    }
                    swing.damage_dealt = true;
                }

                if swing.timer.is_finished() {
                    swing.state = SwingState::Recover;
                    swing
                        .timer
                        .set_duration(std::time::Duration::from_secs_f32(0.1));
                    swing.timer.reset();
                }
            }
            SwingState::Recover => {
                if swing.timer.is_finished() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
