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
    pub player: Single<'w, 's, &'static Transform, With<Player>>,
    pub next_state: ResMut<'w, NextState<GameState>>,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::cast_possible_wrap)]
pub fn spawn_waves(mut params: SpawnWavesParams) {
    let player_pos = params.player.translation.truncate();

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
                        params.round_manager.current_round,
                    );
                    params.round_manager.enemies_to_spawn -= 1;
                } else if params.round_manager.elites_to_spawn > 0 {
                    spawn_elite_enemy(
                        &mut params.commands,
                        &mut params.meshes,
                        &mut params.materials,
                        player_pos,
                        params.round_manager.current_round,
                    );
                    params.round_manager.elites_to_spawn -= 1;
                } else if params.round_manager.yellow_enemies_to_spawn > 0 {
                    spawn_yellow_enemy(
                        &mut params.commands,
                        &mut params.meshes,
                        &mut params.materials,
                        player_pos,
                        params.round_manager.current_round,
                    );
                    params.round_manager.yellow_enemies_to_spawn -= 1;
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
}

#[allow(clippy::cast_precision_loss)]
fn spawn_random_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_pos: Vec2,
    current_round: u32,
) {
    let mut rng = rand::thread_rng();

    // Random position outside screen? Or just circle around player?
    // Circle around player (radius 500-800)
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let radius = rng.gen_range(
        crate::configs::enemy::SPAWN_RADIUS_MIN..crate::configs::enemy::SPAWN_RADIUS_MAX,
    );
    let x = angle.cos() * radius;
    let y = angle.sin() * radius;
    let spawn_pos = player_pos + Vec2::new(x, y);

    // Scaling Formulas
    // Base HP: 30, +20 per round. Round 1: 50, Round 5: 130
    let health = (current_round as f32).mul_add(
        crate::configs::enemy::HEALTH_SCALING_PER_ROUND,
        crate::configs::enemy::BASE_HEALTH,
    );
    // Base Speed: 150 (Constant)
    let speed = crate::configs::enemy::BASE_SPEED;
    // Base Damage: 10, +5 per round. Round 5: 35
    let damage = (current_round as f32).mul_add(
        crate::configs::enemy::DAMAGE_SCALING_PER_ROUND,
        crate::configs::enemy::BASE_DAMAGE,
    );

    println!("Spawning Enemy (R{current_round}): HP={health}, Spd={speed}, Dmg={damage}");

    commands.spawn((
        (
            Mesh2d(meshes.add(Circle::new(crate::configs::enemy::VISUAL_RADIUS))),
            MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::RED))),
            Transform::from_translation(spawn_pos.extend(crate::configs::enemy::VISUAL_Z_INDEX)),
        ),
        Collider::ball(crate::configs::enemy::COLLIDER_RADIUS),
        Velocity::default(),
        Enemy {
            health,
            speed,
            #[allow(dead_code)]
            damage,
        },
    ));
}

#[allow(clippy::cast_precision_loss)]
fn spawn_elite_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_pos: Vec2,
    current_round: u32,
) {
    let mut rng = rand::thread_rng();

    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let radius = rng.gen_range(
        crate::configs::enemy::SPAWN_RADIUS_MIN..crate::configs::enemy::SPAWN_RADIUS_MAX,
    );
    let x = angle.cos() * radius;
    let y = angle.sin() * radius;
    let spawn_pos = player_pos + Vec2::new(x, y);

    let health = (current_round as f32).mul_add(
        crate::configs::enemy::ELITE_HEALTH_SCALING_PER_ROUND,
        crate::configs::enemy::ELITE_BASE_HEALTH,
    );
    let speed = crate::configs::enemy::ELITE_BASE_SPEED;
    let damage = (current_round as f32).mul_add(
        crate::configs::enemy::DAMAGE_SCALING_PER_ROUND,
        crate::configs::enemy::BASE_DAMAGE, // Elites also do normal contact damage
    );

    println!("Spawning ELITE Enemy (R{current_round}): HP={health}, Spd={speed}");

    commands.spawn((
        (
            Mesh2d(meshes.add(Circle::new(crate::configs::enemy::ELITE_VISUAL_RADIUS))),
            MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::PURPLE))),
            Transform::from_translation(spawn_pos.extend(crate::configs::enemy::VISUAL_Z_INDEX)),
        ),
        Collider::ball(crate::configs::enemy::ELITE_COLLIDER_RADIUS),
        Velocity::default(),
        Enemy {
            health,
            speed,
            #[allow(dead_code)]
            damage,
        },
        crate::components::enemy::EliteEnemy,
        crate::components::enemy::EliteAi {
            shuriken_timer: Timer::from_seconds(
                crate::configs::enemy::ELITE_SHURIKEN_COOLDOWN,
                TimerMode::Repeating,
            ),
            teleport_timer: Timer::from_seconds(
                crate::configs::enemy::ELITE_TELEPORT_COOLDOWN,
                TimerMode::Repeating,
            ),
        },
    ));
}

#[allow(clippy::cast_precision_loss)]
fn spawn_yellow_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_pos: Vec2,
    current_round: u32,
) {
    let mut rng = rand::thread_rng();

    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let radius = rng.gen_range(
        crate::configs::enemy::SPAWN_RADIUS_MIN..crate::configs::enemy::SPAWN_RADIUS_MAX,
    );
    let x = angle.cos() * radius;
    let y = angle.sin() * radius;
    let spawn_pos = player_pos + Vec2::new(x, y);

    let health = (current_round as f32).mul_add(
        crate::configs::enemy::YELLOW_HEALTH_SCALING_PER_ROUND,
        crate::configs::enemy::YELLOW_BASE_HEALTH,
    );
    let speed = crate::configs::enemy::YELLOW_BASE_SPEED;

    // Damage mul: +0.1 * round
    let damage_multiplier = 0.1f32.mul_add(current_round as f32, 1.0);
    // Crit chance: 10% mỗi round = 0.1 * round
    let crit_chance = 0.1 * current_round as f32;
    // Crit damage: 2 + 0.5 * round
    let crit_damage = 0.5f32.mul_add(current_round as f32, 2.0);

    println!(
        "Spawning YELLOW Enemy (R{current_round}): HP={health}, Spd={speed}, DmgMul={damage_multiplier}, Crit={crit_chance}/{crit_damage}"
    );

    commands.spawn((
        (
            Mesh2d(meshes.add(Circle::new(crate::configs::enemy::YELLOW_VISUAL_RADIUS))),
            MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::css::YELLOW))),
            Transform::from_translation(spawn_pos.extend(crate::configs::enemy::VISUAL_Z_INDEX)),
        ),
        Collider::ball(crate::configs::enemy::YELLOW_COLLIDER_RADIUS),
        Velocity::default(),
        Enemy {
            health,
            speed,
            damage: crate::configs::enemy::BASE_DAMAGE * damage_multiplier,
        },
        crate::components::enemy::YellowEnemy,
        crate::components::enemy::YellowAi {
            blink_timer: Timer::from_seconds(
                crate::configs::enemy::YELLOW_BLINK_COOLDOWN,
                TimerMode::Repeating,
            ),
            global_timer: Timer::from_seconds(
                crate::configs::enemy::YELLOW_GLOBAL_COOLDOWN,
                TimerMode::Repeating,
            ),
        },
        crate::components::player::PlayerStats {
            damage_multiplier,
            ..default()
        },
        crate::components::player::CombatStats {
            crit_chance,
            crit_damage,
            ..default()
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
pub fn enemy_chase_player(
    mut enemy_query: Query<(&mut Velocity, &Transform, &Enemy)>,
    player: Single<&Transform, With<Player>>,
) {
    let player_pos = player.translation.truncate();

    for (mut velocity, transform, enemy) in &mut enemy_query {
        let pos = transform.translation.truncate();
        let dir = (player_pos - pos).normalize_or_zero();
        velocity.linvel = dir * enemy.speed;
    }
}
