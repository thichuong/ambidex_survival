use super::components::{
    ArsenalButton, MagicCycleButton, MagicPanel, MenuCDRText, MenuCritText, MenuDamageText,
    MenuGoldText, MenuHealthText, MenuLifestealText, WeaponDetailPanel, WeaponStateGroup,
};
use crate::components::player::{CombatStats, Currency, Hand, Health, Player, PlayerStats};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponDescriptionText;

#[allow(clippy::needless_pass_by_value)]
pub fn magic_button_observer(
    trigger: On<Pointer<Click>>,
    btn_query: Query<&MagicCycleButton>,
    mut loadout_query: Query<(&Hand, &mut MagicLoadout)>,
) {
    if let Ok(btn_data) = btn_query.get(trigger.entity) {
        for (hand, mut loadout) in &mut loadout_query {
            if hand.side == btn_data.side {
                let current = if btn_data.is_primary {
                    loadout.primary
                } else {
                    loadout.secondary
                };
                let next = match current {
                    SpellType::EnergyBolt => SpellType::Laser,
                    SpellType::Laser => SpellType::Nova,
                    SpellType::Nova => SpellType::Blink,
                    SpellType::Blink => SpellType::Global,
                    SpellType::Global => SpellType::EnergyBolt,
                };

                if btn_data.is_primary {
                    loadout.primary = next;
                } else {
                    loadout.secondary = next;
                }
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_magic_ui(
    mut panel_query: Query<(&mut Node, &MagicPanel)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
    button_node_query: Query<(&Children, &MagicCycleButton), With<Button>>,
    mut icon_query: Query<(&mut ImageNode, &MagicCycleButton), Without<Button>>,
    mut text_query: Query<&mut Text>,
    loadout_query: Query<&MagicLoadout>,
    asset_server: Res<AssetServer>,
    active_side: Res<super::resources::ActiveDescriptionSide>,
) {
    for (mut node, panel) in &mut panel_query {
        let is_active_side = panel.side == active_side.0;
        let show = is_active_side
            && hand_query
                .iter()
                .any(|(h, weapon)| h.side == panel.side && weapon.kind == WeaponType::Magic);
        node.display = if show { Display::Flex } else { Display::None };
    }

    for (hand, _) in &hand_query {
        let loadout_opt =
            loadout_query
                .iter()
                .zip(hand_query.iter())
                .find_map(|(loadout, (h, _))| {
                    if h.side == hand.side {
                        Some(loadout)
                    } else {
                        None
                    }
                });

        if let Some(loadout) = loadout_opt {
            for (children, btn_data) in &button_node_query {
                if btn_data.side == hand.side {
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

            for (mut image, icon_data) in &mut icon_query {
                if icon_data.side == hand.side {
                    let spell = if icon_data.is_primary {
                        loadout.primary
                    } else {
                        loadout.secondary
                    };
                    let path = match spell {
                        SpellType::EnergyBolt => "ui/icons/magic_bolt.png",
                        SpellType::Laser => "ui/icons/magic_laser.png",
                        SpellType::Nova => "ui/icons/magic_nova.png",
                        SpellType::Blink => "ui/icons/magic_blink.png",
                        SpellType::Global => "ui/icons/magic_global.png",
                    };
                    image.image = asset_server.load(path);
                }
            }
        }
    }
}

pub fn update_menu_weapon_details_ui(
    mut panel_query: Query<(&mut Node, &Children, &WeaponDetailPanel)>,
    mut group_query: Query<(&mut Node, &WeaponStateGroup), Without<WeaponDetailPanel>>,
    mut text_query: Query<&mut Text, With<WeaponDescriptionText>>,
    hand_query: Query<(
        &Hand,
        &crate::components::weapon::Weapon,
        Option<&MagicLoadout>,
    )>,
    active_side: Res<super::resources::ActiveDescriptionSide>,
) {
    use super::arsenal::get_weapon_description;
    for (mut panel_node, children, panel) in &mut panel_query {
        if panel.side != active_side.0 {
            panel_node.display = Display::None;
            continue;
        }

        let active_data = hand_query.iter().find(|(h, _, _)| h.side == panel.side);

        if let Some((_, weapon, loadout_opt)) = active_data {
            panel_node.display = Display::Flex;
            let weapon_kind = weapon.kind;
            let loadout = if weapon_kind == WeaponType::Magic {
                loadout_opt
            } else {
                None
            };
            let desc = get_weapon_description(weapon_kind, loadout);

            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0.clone_from(&desc);
                }
            }

            for (mut group_node, group) in &mut group_query {
                if group.side == panel.side {
                    group_node.display = if group.weapon_type == weapon_kind {
                        Display::Flex
                    } else {
                        Display::None
                    };
                }
            }
        } else {
            panel_node.display = Display::None;
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_gold_text(
    mut query: Query<&mut Text, With<MenuGoldText>>,
    player: Single<&Currency, With<Player>>,
) {
    for mut text in &mut query {
        text.0 = format!("Gold: {}", player.gold);
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_health_text(
    mut query: Query<&mut Text, With<MenuHealthText>>,
    player: Single<&Health, With<Player>>,
) {
    for mut text in &mut query {
        text.0 = format!("HP: {:.0}/{:.0}", player.current, player.max);
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_damage_text(
    mut query: Query<&mut Text, With<MenuDamageText>>,
    player: Single<&PlayerStats, With<Player>>,
) {
    for mut text in &mut query {
        let bonus = (player.damage_multiplier - 1.0) * 100.0;
        text.0 = format!("Dmg: +{bonus:.0}%");
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_crit_text(
    mut query: Query<&mut Text, With<MenuCritText>>,
    player: Single<&CombatStats, With<Player>>,
) {
    for mut text in &mut query {
        let chance = player.crit_chance * 100.0;
        let damage = player.crit_damage;
        text.0 = format!("Crit: {chance:.0}% (x{damage:.1})");
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_lifesteal_text(
    mut query: Query<&mut Text, With<MenuLifestealText>>,
    player: Single<&CombatStats, With<Player>>,
) {
    for mut text in &mut query {
        let life = player.lifesteal * 100.0;
        text.0 = format!("Life Steal: {life:.0}% | AOE: {:.0}%", life / 2.0);
    }
}

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

pub fn update_menu_weapon_buttons(
    mut button_query: Query<(&ArsenalButton, &mut BackgroundColor)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
) {
    for (button, mut color) in &mut button_query {
        let is_active = hand_query
            .iter()
            .any(|(h, weapon)| h.side == button.side && weapon.kind == button.kind);
        if is_active {
            *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
        } else {
            *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0));
        }
    }
}

pub fn update_description_wrapper_visibility(
    mut query: Query<(&mut Node, &super::components::DescriptionWrapper)>,
    active_side: Res<super::resources::ActiveDescriptionSide>,
) {
    for (mut node, wrapper) in &mut query {
        node.display = if wrapper.side == active_side.0 {
            Display::Flex
        } else {
            Display::None
        };
    }
}
