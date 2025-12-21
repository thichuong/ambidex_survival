use crate::components::player::{CombatStats, Hand, HandType, Player, PlayerStats};
use crate::components::weapon::{ActiveSpellSlot, MagicLoadout, SpellType, Weapon, WeaponType};
use crate::systems::combat::CombatInputParams;
use bevy::prelude::*;

pub mod blink;
pub mod energy_bolt;
pub mod global_spell;
pub mod laser;
pub mod nova;

pub fn magic_weapon_system(
    mut params: CombatInputParams,
    mut player_query: Query<(Entity, &mut Transform, &PlayerStats, &CombatStats), With<Player>>,
    mut hand_query: Query<(&GlobalTransform, &Hand, &mut MagicLoadout, &mut Weapon)>,
) -> Result<(), String> {
    let (camera, camera_transform) = params
        .camera_query
        .single()
        .map_err(|e| format!("Camera not found: {e:?}"))?;
    let window = params
        .window_query
        .single()
        .map_err(|e| format!("Window not found: {e:?}"))?;

    let cursor_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
        .ok_or_else(|| "Cursor position not available".to_string())?;

    let (player_entity, mut player_transform, stats, combat_stats) = player_query
        .single_mut()
        .map_err(|e| format!("Player not found: {e:?}"))?;

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
                player_entity,
                &mut player_transform,
                cursor_pos,
                hand_pos,
                stats.damage_multiplier,
            );
            weapon_data.last_shot = now;
        }
    }

    Ok(())
}

fn cast_spell(
    params: &mut CombatInputParams,
    spell: SpellType,
    player_entity: Entity,
    player_transform: &mut Transform,
    cursor_pos: Vec2,
    spawn_pos: Vec2,
    damage_multiplier: f32,
) {
    let direction = (cursor_pos - spawn_pos).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    match spell {
        SpellType::EnergyBolt => {
            energy_bolt::spawn_energy_bolt(
                params,
                player_entity,
                spawn_pos,
                direction,
                angle,
                damage_multiplier,
            );
        }
        SpellType::Laser => {
            laser::spawn_laser(
                params,
                player_entity,
                spawn_pos,
                direction,
                angle,
                damage_multiplier,
            );
        }
        SpellType::Nova => {
            nova::spawn_nova(
                params,
                player_entity,
                player_transform.translation,
                damage_multiplier,
            );
        }
        SpellType::Blink => {
            blink::perform_blink(player_transform, cursor_pos);
        }
        SpellType::Global => {
            global_spell::spawn_global_spell(
                params,
                player_entity,
                player_transform.translation,
                damage_multiplier,
            );
        }
    }
}
