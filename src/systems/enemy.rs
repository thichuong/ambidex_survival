use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use rand::Rng;

use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, Velocity};
use crate::components::player::Player;
use crate::resources::game_state::GameState;
use crate::resources::round::{RoundManager, RoundState};

#[derive(SystemParam)]
pub struct SpawnWavesParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub round_manager: ResMut<'w, RoundManager>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub enemy_query: Query<'w, 's, &'static Enemy>,
    pub player_query: Query<'w, 's, &'static Transform, With<Player>>,
    pub next_state: ResMut<'w, NextState<GameState>>,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::cast_possible_wrap)]
pub fn spawn_waves(mut params: SpawnWavesParams) -> Result<(), String> {
    let tf = params
        .player_query
        .single()
        .map_err(|e| format!("Player not found: {e:?}"))?;
    let player_pos = tf.translation.truncate();

    match params.round_manager.round_state {
        RoundState::Spawning => {
            params.round_manager.spawn_timer.tick(params.time.delta());
            if params.round_manager.spawn_timer.is_finished() {
                if params.round_manager.enemies_to_spawn > 0 {
                    spawn_random_enemy(
                        &mut params.commands,
                        &mut params.meshes,
                        &mut params.materials,
                        player_pos,
                    );
                    params.round_manager.enemies_to_spawn -= 1;
                } else {
                    params.round_manager.round_state = RoundState::Fighting;
                    println!("Wave Spawning Finished! Fighting...");
                }
            }
        }
        RoundState::Fighting => {
            // Check if all enemies are dead
            let alive_count = params.enemy_query.iter().count();
            if alive_count == 0 {
                println!("Round Cleared! Opening Menu...");
                params.round_manager.round_state = RoundState::Shop;
                // Tự động hiện Menu khi hết round
                params.next_state.set(GameState::WeaponMenu);
            }
        }
        RoundState::Shop => {
            // Không làm gì - chờ người chơi bấm "BACK TO GAME" trong Menu
            // Logic chuyển round đã được xử lý ở nút "BACK TO GAME"
        }
    }
    Ok(())
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
        (
            Mesh2d(meshes.add(Circle::new(15.0))),
            MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::RED))),
            Transform::from_translation(spawn_pos.extend(0.1)),
        ),
        Collider::ball(15.0),
        Velocity::default(),
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
) -> Result<(), String> {
    let player_transform = player_query
        .single()
        .map_err(|e| format!("Player not found: {e:?}"))?;
    let player_pos = player_transform.translation.truncate();

    for (mut velocity, transform, enemy) in &mut enemy_query {
        let pos = transform.translation.truncate();
        let dir = (player_pos - pos).normalize_or_zero();
        velocity.linvel = dir * enemy.speed;
    }
    Ok(())
}
