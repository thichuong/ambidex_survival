use super::components::{
    CooldownOverlay, GoldText, HUDIcon, HealthBar, HealthText, RoundText, ShurikenCountText,
};
use crate::components::player::{CombatStats, Currency, Hand, Health, Player};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
use bevy::prelude::*;

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn update_hud_indicators(
    mut icon_query: Query<(&HUDIcon, &mut ImageNode)>,
    hand_query: Query<(
        &Hand,
        &crate::components::weapon::SwordState,
        &crate::components::weapon::GunState,
        &MagicLoadout,
    )>,
    asset_server: Res<AssetServer>,
) -> Result<(), String> {
    for (icon, mut image_node) in &mut icon_query {
        if let Some((hand, sword, gun, magic)) =
            hand_query.iter().find(|(h, _, _, _)| h.side == icon.side)
        {
            let icon_path = match hand.equipped_weapon {
                Some(WeaponType::Shuriken) | None => "ui/icons/shuriken.png", // Default fallback
                Some(WeaponType::Sword) => match sword.mode {
                    crate::components::weapon::SwordMode::Normal => "ui/icons/sword_normal.png",
                    crate::components::weapon::SwordMode::Shattered => {
                        "ui/icons/sword_shattered.png"
                    }
                },
                Some(WeaponType::Gun) => match gun.mode {
                    crate::components::weapon::GunMode::Single => "ui/icons/gun_single.png",
                    crate::components::weapon::GunMode::Shotgun => "ui/icons/gun_shotgun.png",
                    crate::components::weapon::GunMode::Rapid => "ui/icons/gun_rapid.png",
                },
                Some(WeaponType::Magic) => {
                    let spell = if magic.active_slot
                        == crate::components::weapon::ActiveSpellSlot::Primary
                    {
                        magic.primary
                    } else {
                        magic.secondary
                    };
                    match spell {
                        SpellType::EnergyBolt => "ui/icons/magic_bolt.png",
                        SpellType::Laser => "ui/icons/magic_laser.png",
                        SpellType::Nova => "ui/icons/magic_nova.png",
                        SpellType::Blink => "ui/icons/magic_blink.png",
                        SpellType::Global => "ui/icons/magic_global.png",
                    }
                }
            };

            image_node.image = asset_server.load(icon_path);
        }
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
pub fn update_health_ui(
    mut health_bar_query: Query<&mut Node, With<HealthBar>>,
    mut health_text_query: Query<&mut Text, With<HealthText>>,
    player_query: Query<&Health, With<Player>>,
) -> Result<(), String> {
    if let Ok(health) = player_query.single() {
        for mut node in &mut health_bar_query {
            // Player health is 0..max_health
            let percent = (health.current / health.max).clamp(0.0, 1.0) * 100.0;
            node.width = Val::Percent(percent);
        }

        for mut text in &mut health_text_query {
            **text = format!("{:.0} / {:.0}", health.current, health.max);
        }
    }
    Ok(())
}

#[allow(clippy::type_complexity)]
pub fn update_gold_ui(
    mut gold_text_query: Query<&mut Text, With<GoldText>>,
    player_query: Query<&Currency, With<Player>>,
) {
    if let Ok(currency) = player_query.single() {
        for mut text in &mut gold_text_query {
            **text = format!("Gold: {}", currency.gold);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_round_text(
    mut query: Query<&mut Text, With<RoundText>>,
    round_manager: Res<crate::resources::round::RoundManager>,
) {
    for mut text in &mut query {
        text.0 = format!("Round: {}", round_manager.current_round);
    }
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn update_cooldown_indicators(
    mut overlay_query: Query<(&mut Node, &CooldownOverlay)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
    player_query: Query<&CombatStats, With<Player>>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    let combat_stats = player_query.single();

    for (mut node, overlay) in &mut overlay_query {
        if let Some((hand, weapon)) = hand_query.iter().find(|(h, _)| h.side == overlay.side) {
            // Apply CDR only to Magic weapons
            let effective_cooldown = if hand.equipped_weapon == Some(WeaponType::Magic) {
                combat_stats.as_ref().map_or(weapon.cooldown, |stats| {
                    weapon.cooldown * (1.0 - stats.cooldown_reduction)
                })
            } else {
                weapon.cooldown
            };

            // Calculate primary cooldown progress
            let primary_elapsed = now - weapon.last_shot;
            let primary_progress = if effective_cooldown > 0.0 {
                (1.0 - (primary_elapsed / effective_cooldown)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            // Calculate skill cooldown progress
            // Note: Currently skill_cooldown doesn't use CDR in weapon_logic.rs,
            // but we follow the same pattern for consistency if it ever does.
            let skill_elapsed = now - weapon.last_skill_use;
            let skill_progress = if weapon.skill_cooldown > 0.0 {
                (1.0 - (skill_elapsed / weapon.skill_cooldown)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            // Use the maximum of both cooldowns for the overlay
            let max_progress = primary_progress.max(skill_progress);

            if max_progress > 0.0 {
                node.height = Val::Percent(max_progress * 100.0);
                node.display = Display::Flex;
            } else {
                node.height = Val::Percent(0.0);
                node.display = Display::None;
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_shuriken_count_ui(
    mut text_query: Query<(&mut Text, &ShurikenCountText)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
    projectile_query: Query<&crate::components::weapon::Projectile>,
) {
    for (mut text, count_text) in &mut text_query {
        if let Some((hand, _)) = hand_query.iter().find(|(h, _)| h.side == count_text.side) {
            if hand.equipped_weapon == Some(WeaponType::Shuriken) {
                let count = projectile_query
                    .iter()
                    .filter(|p| p.kind == WeaponType::Shuriken)
                    .count();

                if count > 0 {
                    text.0 = format!("{count}");
                } else {
                    text.0 = String::new();
                }
            } else {
                text.0 = String::new();
            }
        }
    }
}
