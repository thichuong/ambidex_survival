use super::{CombatResources, DamageEvent};
use crate::components::enemy::Enemy;
use crate::components::physics::Velocity;
use crate::components::player::{Hand, Player};
use crate::components::weapon::{Lifetime, SwingState, SwordSwing};
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
    mut res: CombatResources,
    mut damage_events: MessageWriter<DamageEvent>,
    mut player_query: Query<&mut Player>,
) -> Result<(), String> {
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
                                enemy.health -= swing.damage;
                                damage_events.write(DamageEvent {
                                    damage: swing.damage,
                                    position: enemy_tf.translation.truncate(),
                                });
                                res.shake.add_trauma(0.2);
                                if enemy.health <= 0.0 {
                                    if let Ok(mut player) = player_query.single_mut() {
                                        player.gold += 10;
                                    }
                                    commands.entity(enemy_entity).despawn();

                                    let mut rng = rand::thread_rng();
                                    for _ in 0..5 {
                                        let dir = Vec2::new(
                                            rng.gen_range(-1.0..1.0),
                                            rng.gen_range(-1.0..1.0),
                                        )
                                        .normalize_or_zero();
                                        commands.spawn((
                                            Mesh2d(res.meshes.add(Circle::new(3.0))),
                                            MeshMaterial2d(
                                                res.materials.add(Color::srgb(1.0, 0.0, 0.0)),
                                            ),
                                            Transform::from_translation(enemy_tf.translation),
                                            Velocity {
                                                linvel: dir * 100.0,
                                                angvel: 0.0,
                                            },
                                            Lifetime {
                                                timer: Timer::from_seconds(0.5, TimerMode::Once),
                                            },
                                        ));
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
    Ok(())
}
