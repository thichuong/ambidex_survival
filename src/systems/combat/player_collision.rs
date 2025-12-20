use super::CombatResources;
use crate::components::enemy::Enemy;
use crate::components::physics::{Collider, UniformGrid, check_collision};
use crate::components::player::Player;
use crate::resources::game_state::GameState;
use bevy::prelude::*;

const COLLISION_PUSH_STRENGTH: f32 = 200.0;

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn handle_player_collision(
    mut player_query: Query<(&mut Player, &mut Transform, &Collider), Without<Enemy>>,
    mut enemy_query: Query<(Entity, &mut Transform, &Enemy, &Collider), Without<Player>>,
    grid: Res<UniformGrid>,
    time: Res<Time>,
    mut res: CombatResources,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result<(), String> {
    if let Ok((mut player, mut player_transform, player_collider)) = player_query.single_mut() {
        player.invulnerability_timer.tick(time.delta());

        let player_pos = player_transform.translation.truncate();

        // Query nearby enemies using spatial grid instead of iterating all
        let nearby_entities = grid.query_nearby(player_pos);

        for enemy_entity in nearby_entities {
            if let Ok((_, mut enemy_transform, enemy, enemy_collider)) =
                enemy_query.get_mut(enemy_entity)
            {
                let enemy_pos = enemy_transform.translation.truncate();

                if check_collision(player_pos, player_collider, enemy_pos, enemy_collider) {
                    // Calculate push direction and apply separation
                    let diff = player_pos - enemy_pos;
                    let distance = diff.length();

                    if distance > 0.0 {
                        let push_dir = diff / distance;
                        let push_amount = COLLISION_PUSH_STRENGTH * time.delta_secs();

                        // Push both player and enemy apart
                        player_transform.translation.x += push_dir.x * push_amount;
                        player_transform.translation.y += push_dir.y * push_amount;
                        enemy_transform.translation.x -= push_dir.x * push_amount;
                        enemy_transform.translation.y -= push_dir.y * push_amount;
                    }

                    // Apply damage only if not invulnerable
                    if player.invulnerability_timer.is_finished() {
                        player.health -= enemy.damage;
                        player.invulnerability_timer.reset();
                        res.shake.add_trauma(0.5);

                        if player.health <= 0.0 {
                            player.health = 0.0;
                            next_state.set(GameState::GameOver);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
