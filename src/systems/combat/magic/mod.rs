use crate::components::player::{CombatStats, Hand, HandType, Player, PlayerStats, Progression};
use crate::components::weapon::{ActiveSpellSlot, MagicLoadout, SpellType, Weapon, WeaponType};
use crate::systems::combat::{CombatContext, CombatInputParams};
use bevy::prelude::*;

pub mod blink;
pub mod energy_bolt;
pub mod global_spell;
pub mod laser;
pub mod nova;

pub fn magic_weapon_system(
    mut params: CombatInputParams,
    mut player: Single<
        (
            Entity,
            &mut Transform,
            &PlayerStats,
            &CombatStats,
            &Progression,
        ),
        With<Player>,
    >,
    mut hand_query: Query<(&GlobalTransform, &Hand, &mut MagicLoadout, &mut Weapon)>,
) {
    let (camera, camera_transform) = *params.camera;
    let window = *params.window;

    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.get_point(0.0).truncate());

    let Some(cursor_pos) = cursor_pos else {
        return;
    };

    let player_entity = player.0;
    let stats = player.2;
    let combat_stats = player.3;
    let progression = player.4;
    let player_transform = &mut *player.1;

    let left_pressed = params.mouse_input.pressed(MouseButton::Left);
    let right_pressed = params.mouse_input.pressed(MouseButton::Right);
    let left_just_pressed = params.mouse_input.just_pressed(MouseButton::Left);
    let right_just_pressed = params.mouse_input.just_pressed(MouseButton::Right);

    let q_just_pressed = params.key_input.just_pressed(KeyCode::KeyQ);
    let e_just_pressed = params.key_input.just_pressed(KeyCode::KeyE);

    for (hand_transform, hand, mut magic_loadout, mut weapon_data) in &mut hand_query {
        if hand.equipped_weapon != Some(WeaponType::Magic) {
            continue;
        }

        let hand_pos = hand_transform.translation().truncate();

        let (_, is_just_pressed, skill_pressed) = match hand.side {
            HandType::Left => (left_pressed, left_just_pressed, q_just_pressed),
            HandType::Right => (right_pressed, right_just_pressed, e_just_pressed),
        };

        // Skill logic (Switch slot)
        if skill_pressed {
            match magic_loadout.active_slot {
                ActiveSpellSlot::Primary => {
                    magic_loadout.active_slot = ActiveSpellSlot::Secondary;
                }
                ActiveSpellSlot::Secondary => {
                    magic_loadout.active_slot = ActiveSpellSlot::Primary;
                }
            }
        }

        let now = params.time.elapsed_secs();
        let effective_cooldown = weapon_data.cooldown * (1.0 - combat_stats.cooldown_reduction);

        // Fire logic
        if is_just_pressed && now - weapon_data.last_shot >= effective_cooldown {
            let spell_to_cast = match magic_loadout.active_slot {
                ActiveSpellSlot::Primary => magic_loadout.primary,
                ActiveSpellSlot::Secondary => magic_loadout.secondary,
            };

            cast_spell(
                &mut params,
                spell_to_cast,
                CombatContext {
                    owner_entity: player_entity,
                    transform: &mut *player_transform,
                    cursor_pos,
                    spawn_pos: hand_pos,
                    damage_multiplier: stats.damage_multiplier,
                    combat_stats,
                    progression,
                },
            );
            weapon_data.last_shot = now;
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn cast_spell(params: &mut CombatInputParams, spell: SpellType, ctx: CombatContext) {
    let direction = (ctx.cursor_pos - ctx.spawn_pos).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    match spell {
        SpellType::EnergyBolt => {
            energy_bolt::spawn_energy_bolt(
                params,
                ctx.owner_entity,
                ctx.spawn_pos,
                direction,
                angle,
                ctx.damage_multiplier,
                ctx.combat_stats.crit_chance,
                ctx.combat_stats.crit_damage,
            );
        }
        SpellType::Laser => {
            laser::spawn_laser(
                params,
                ctx.owner_entity,
                ctx.spawn_pos,
                direction,
                angle,
                ctx.damage_multiplier,
                ctx.combat_stats.crit_chance,
                ctx.combat_stats.crit_damage,
            );
        }
        SpellType::Nova => {
            let explosion_pos = if ctx.progression.nova_core > 0 {
                ctx.cursor_pos.extend(ctx.transform.translation.z)
            } else {
                ctx.transform.translation
            };
            nova::spawn_nova(
                params,
                ctx.owner_entity,
                explosion_pos,
                ctx.damage_multiplier,
                ctx.combat_stats.crit_chance,
                ctx.combat_stats.crit_damage,
            );
        }
        SpellType::Blink => {
            blink::perform_blink(ctx.transform, ctx.cursor_pos);
        }
        SpellType::Global => {
            global_spell::spawn_global_spell(
                params,
                ctx.owner_entity,
                ctx.transform.translation,
                ctx.damage_multiplier,
                ctx.combat_stats.crit_chance,
                ctx.combat_stats.crit_damage,
            );
        }
    }
}
