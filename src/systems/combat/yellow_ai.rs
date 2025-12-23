use crate::components::enemy::{YellowAi, YellowEnemy};
use crate::components::player::{CombatStats, Player, PlayerStats, Progression};
use crate::components::weapon::Faction;
use crate::configs::enemy;
use crate::systems::combat::magic::{blink, global_spell};
use crate::systems::combat::{CombatContext, CombatInputParams};
use bevy::prelude::*;
use rand::Rng;

type YellowEnemyQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Transform,
        &'static mut YellowAi,
        &'static CombatStats,
        &'static PlayerStats,
    ),
    (With<YellowEnemy>, Without<Player>),
>;

#[allow(clippy::needless_pass_by_value)]
pub fn yellow_ai_system(
    mut params: CombatInputParams,
    player: Single<(&Transform, &PlayerStats, &Progression), With<Player>>,
    mut yellow_query: YellowEnemyQuery,
) {
    let (player_transform, _player_stats, progression) = *player;
    let player_pos = player_transform.translation.truncate();

    for (enemy_entity, mut enemy_transform, mut ai, combat_stats, enemy_stats) in &mut yellow_query
    {
        ai.blink_timer.tick(params.time.delta());
        ai.global_timer.tick(params.time.delta());

        let enemy_pos = enemy_transform.translation.truncate();

        // Blink logic
        if ai.blink_timer.just_finished() {
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let radius = rng.gen_range(200.0..enemy::YELLOW_BLINK_RANGE);
            let offset = Vec2::new(angle.cos() * radius, angle.sin() * radius);
            let target_pos = player_pos + offset;

            let mut ctx = CombatContext {
                owner_entity: enemy_entity,
                transform: &mut enemy_transform,
                cursor_pos: target_pos,
                spawn_pos: enemy_pos,
                damage_multiplier: enemy_stats.damage_multiplier,
                combat_stats,
                progression,
            };
            blink::perform_blink(&mut ctx);
        }

        // Global Spell logic
        if ai.global_timer.just_finished() {
            let ctx = CombatContext {
                owner_entity: enemy_entity,
                transform: &mut enemy_transform,
                cursor_pos: player_pos,
                spawn_pos: enemy_pos,
                damage_multiplier: enemy_stats.damage_multiplier,
                combat_stats,
                progression,
            };
            global_spell::spawn_global_spell(&mut params, &ctx, Faction::Enemy);
        }
    }
}
