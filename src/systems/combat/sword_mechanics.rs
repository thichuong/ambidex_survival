use super::DamageEvent;
use crate::components::enemy::Enemy;
use crate::components::player::{CombatStats, Hand, HandType, Health, Player};
use crate::components::weapon::{Faction, SwingState, SwordSwing};
use crate::configs::weapons::sword::SWORD_SIDE_OFFSET;
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
    hand_query: Query<(&GlobalTransform, &Hand)>,
    player_query: Single<(&mut Health, &CombatStats), With<Player>>,
) {
    let mut player = player_query;
    for (entity, mut swing, mut transform) in &mut sword_query {
        if let Ok((hand_transform, hand)) = hand_query.get(swing.hand_entity) {
            let hand_pos = hand_transform.translation().truncate();
            let hand_side = hand.side;

            // Calculate offset direction (perpendicular to base_angle)
            // If base_angle is forward, side offset is to the right (+90 deg) or left (-90 deg)
            let side_multiplier = match hand_side {
                HandType::Left => -1.0,
                HandType::Right => 1.0,
            };

            let offset_angle =
                std::f32::consts::FRAC_PI_2.mul_add(-side_multiplier, swing.base_angle);
            let offset_vec = Vec2::new(offset_angle.cos(), offset_angle.sin()) * SWORD_SIDE_OFFSET;

            transform.translation = (hand_pos + offset_vec).extend(0.0);

            // Set swing direction based on hand
            swing.swing_direction = side_multiplier;
        }

        swing.timer.tick(time.delta());

        match swing.state {
            SwingState::Swinging => {
                let progress = swing.timer.fraction();
                // Left hand (side_multiplier -1.0): offset goes from PI/2 to -PI/2?
                // Actually, the user wants it to be a semi-circle based on the hand.
                // If we use swing_direction = side_multiplier:
                // Left hand: offset = (progress - 0.5) * PI * -1.0 => [0.5PI, -0.5PI]
                // Right hand: offset = (progress - 0.5) * PI * 1.0 => [-0.5PI, 0.5PI]
                let offset = (progress - 0.5) * std::f32::consts::PI * swing.swing_direction;
                let current_angle = swing.base_angle + offset;
                transform.rotation = Quat::from_rotation_z(current_angle);

                if !swing.damage_dealt {
                    let sweep_radius = swing.range;

                    if swing.faction == Faction::Player {
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

                                    {
                                        let (ref mut health, stats) = *player;
                                        // Crit Check
                                        let mut rng = rand::thread_rng();
                                        if rng.gen_range(0.0..1.0) < stats.crit_chance {
                                            final_damage *= stats.crit_damage;
                                            is_crit = true;
                                        }

                                        // Lifesteal (Sword is AOE, 50% penalty)
                                        if stats.lifesteal > 0.0 {
                                            let heal = final_damage * stats.lifesteal * 0.5;
                                            health.current =
                                                (health.current + heal).min(health.max);
                                        }
                                    }

                                    enemy.health -= final_damage;

                                    commands.trigger(DamageEvent {
                                        entity: enemy_entity,
                                        damage: final_damage,
                                        crit: is_crit,
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
