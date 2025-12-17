use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::components::enemy::Enemy;
use crate::components::player::Player;
use crate::resources::round::{RoundManager, RoundState};

#[allow(clippy::needless_pass_by_value, clippy::cast_possible_wrap)]
pub fn spawn_waves(
    mut commands: Commands,
    time: Res<Time>,
    mut round_manager: ResMut<RoundManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_query: Query<&Enemy>, // To count alive enemies
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(tf) = player_query.get_single() else {
        return;
    };
    let player_pos = tf.translation.truncate();

    match round_manager.round_state {
        RoundState::Spawning => {
            round_manager.spawn_timer.tick(time.delta());
            if round_manager.spawn_timer.finished() {
                if round_manager.enemies_to_spawn > 0 {
                    spawn_random_enemy(&mut commands, &mut meshes, &mut materials, player_pos);
                    round_manager.enemies_to_spawn -= 1;
                } else {
                    round_manager.round_state = RoundState::Fighting;
                    println!("Wave Spawning Finished! Fighting...");
                }
            }
        }
        RoundState::Fighting => {
            // Check if all enemies are dead
            // Note: enemy_query.iter().count() is O(N), but N is small (<1000 usually)
            let alive_count = enemy_query.iter().count();
            if alive_count == 0 {
                println!("Round Cleared! Entering Shop/Break...");
                round_manager.round_state = RoundState::Shop;
                round_manager.round_timer.reset(); // Start break timer
            }
        }
        RoundState::Shop => {
            round_manager.round_timer.tick(time.delta());
            if round_manager.round_timer.finished() {
                // Next Round
                round_manager.current_round += 1;
                round_manager.enemies_to_spawn = 10 + (round_manager.current_round * 5); // Scale up
                round_manager.spawn_timer = Timer::from_seconds(
                    1.0 * (0.95_f32).powi(round_manager.current_round as i32),
                    TimerMode::Repeating,
                ); // Spawn faster
                round_manager.round_state = RoundState::Spawning;
                println!("Starting Round {}!", round_manager.current_round);
            }
        }
    }
}

fn spawn_random_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_pos: Vec2,
) {
    let mut rng = rand::thread_rng();

    // Random position outside screen? Or just circle around player?
    // Circle around player (radius 500-800)
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let radius = rng.gen_range(500.0..800.0);
    let x = angle.cos() * radius;
    let y = angle.sin() * radius;
    let spawn_pos = player_pos + Vec2::new(x, y);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Circle::new(15.0))).into(),
            material: materials.add(Color::from(bevy::color::palettes::css::RED)),
            transform: Transform::from_translation(spawn_pos.extend(0.1)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(15.0),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED, // Keep upright
        Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        },
        Enemy {
            health: 30.0,
            speed: 150.0,
            #[allow(dead_code)]
            damage: 10.0,
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
pub fn enemy_chase_player(
    mut enemy_query: Query<(&mut Velocity, &Transform, &Enemy)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (mut velocity, transform, enemy) in &mut enemy_query {
        let pos = transform.translation.truncate();
        let dir = (player_pos - pos).normalize_or_zero();
        velocity.linvel = dir * enemy.speed;
    }
}
