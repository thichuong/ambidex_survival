use super::components::{
    MagicCycleButton, MagicPanel, MenuCDRText, MenuCritText, MenuDamageText, MenuGoldText,
    MenuHealthText, MenuLifestealText,
};
use crate::components::player::{CombatStats, Currency, Hand, Health, Player, PlayerStats};
use crate::components::weapon::MagicLoadout;
use crate::components::weapon::SpellType;
use bevy::prelude::*;

#[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
pub fn update_magic_ui(
    mut panel_query: Query<(&mut Node, &MagicPanel)>,
    hand_query: Query<&Hand>,
    button_node_query: Query<(&Children, &MagicCycleButton)>,
    mut text_query: Query<&mut Text>,
    loadout_query: Query<&MagicLoadout>,
) -> Result<(), String> {
    // 1. Update Panel Visibility
    for (mut node, _) in &mut panel_query {
        node.display = Display::Flex;
    }

    // 2. Update Button Text from Loadout
    for (children, btn_data) in button_node_query.iter() {
        if let Some((_, loadout)) = hand_query
            .iter()
            .zip(loadout_query.iter())
            .find(|(hand, _)| hand.side == btn_data.side)
        {
            let spell = if btn_data.is_primary {
                loadout.primary
            } else {
                loadout.secondary
            };
            let spell_name = match spell {
                SpellType::EnergyBolt => "Bolt",
                SpellType::Laser => "Laser",
                SpellType::Nova => "Nova",
                SpellType::Blink => "Blink",
                SpellType::Global => "Global",
            };
            let prefix = if btn_data.is_primary { "Pri" } else { "Sec" };

            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = format!("{prefix}: {spell_name}");
                }
            }
        }
    }
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_gold_text(
    mut query: Query<&mut Text, With<MenuGoldText>>,
    player_query: Query<&Currency, With<Player>>,
) {
    if let Ok(currency) = player_query.single() {
        for mut text in &mut query {
            text.0 = format!("Gold: {}", currency.gold);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_health_text(
    mut query: Query<&mut Text, With<MenuHealthText>>,
    player_query: Query<&Health, With<Player>>,
) {
    if let Ok(health) = player_query.single() {
        for mut text in &mut query {
            text.0 = format!("HP: {:.0}/{:.0}", health.current, health.max);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_damage_text(
    mut query: Query<&mut Text, With<MenuDamageText>>,
    player_query: Query<&PlayerStats, With<Player>>,
) {
    if let Ok(stats) = player_query.single() {
        for mut text in &mut query {
            let bonus = (stats.damage_multiplier - 1.0) * 100.0;
            text.0 = format!("Dmg: +{bonus:.0}%");
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_crit_text(
    mut query: Query<&mut Text, With<MenuCritText>>,
    player_query: Query<&CombatStats, With<Player>>,
) {
    if let Ok(stats) = player_query.single() {
        for mut text in &mut query {
            let chance = stats.crit_chance * 100.0;
            let damage = stats.crit_damage;
            text.0 = format!("Crit: {chance:.0}% (x{damage:.1})");
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_lifesteal_text(
    mut query: Query<&mut Text, With<MenuLifestealText>>,
    player_query: Query<&CombatStats, With<Player>>,
) {
    if let Ok(stats) = player_query.single() {
        for mut text in &mut query {
            let life = stats.lifesteal * 100.0;
            text.0 = format!("Life Steal: {life:.0}%");
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_cdr_text(
    mut query: Query<&mut Text, With<MenuCDRText>>,
    player_query: Query<&CombatStats, With<Player>>,
) {
    if let Ok(stats) = player_query.single() {
        for mut text in &mut query {
            let cdr = stats.cooldown_reduction * 100.0;
            text.0 = format!("CDR: {cdr:.0}%");
        }
    }
}
