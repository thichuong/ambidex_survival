use crate::components::player::{Hand, HandType};

use crate::components::weapon::WeaponType;
use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponButton {
    pub side: HandType,
    pub kind: WeaponType,
}

#[derive(Component)]
pub struct ShopMenu;

#[derive(Component)]
pub enum ShopButton {
    Heal,
    DamageUp,
    NextRound,
}

#[derive(Component)]
pub struct MagicPanel {
    pub side: HandType,
}

#[derive(Component)]
pub struct MagicCycleButton {
    pub side: HandType,
    pub is_primary: bool, // true = primary, false = secondary
}

use crate::components::weapon::{MagicLoadout, SpellType};

pub fn setup_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Root UI Node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd, // Align to bottom
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Weapon Select Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                    ..default()
                })
                .with_children(|panel| {
                    // Text Label
                    panel.spawn(TextBundle::from_section(
                        "Left Hand (Q) [Click to Set]:",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Buttons for Left Hand
                    spawn_weapon_button(panel, HandType::Left, WeaponType::Shuriken, "Shuriken");
                    spawn_weapon_button(panel, HandType::Left, WeaponType::Sword, "Sword");
                    spawn_weapon_button(panel, HandType::Left, WeaponType::Gun, "Gun");
                    spawn_weapon_button(panel, HandType::Left, WeaponType::Magic, "Magic");

                    // Spacer
                    panel.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    });

                    // Text Label
                    panel.spawn(TextBundle::from_section(
                        "Right Hand (E) [Click to Set]:",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Buttons for Right Hand
                    spawn_weapon_button(panel, HandType::Right, WeaponType::Shuriken, "Shuriken");
                    spawn_weapon_button(panel, HandType::Right, WeaponType::Sword, "Sword");
                    spawn_weapon_button(panel, HandType::Right, WeaponType::Gun, "Gun");
                    spawn_weapon_button(panel, HandType::Right, WeaponType::Magic, "Magic");
                });

            // Magic Loadout Panels (Dynamic)
            spawn_magic_panel(parent, HandType::Left);
            spawn_magic_panel(parent, HandType::Right);

            // Shop Menu (Initially Hidden)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(300.0),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(50.0),              // Center X
                        top: Val::Percent(30.0),               // Center Y
                        margin: UiRect::left(Val::Px(-200.0)), // Offset by half width
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::None, // Hidden by default
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                    ..default()
                })
                .insert(ShopMenu)
                .with_children(|shop| {
                    shop.spawn(TextBundle::from_section(
                        "--- SHOP ---",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    spawn_shop_button(shop, ShopButton::Heal, "Heal (+30 HP)");
                    spawn_shop_button(shop, ShopButton::DamageUp, "Damage Up (+10%)");
                    spawn_shop_button(shop, ShopButton::NextRound, "Start Next Round");
                });
        });
}

fn spawn_shop_button(parent: &mut ChildBuilder, btn_type: ShopButton, label: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
            ..default()
        })
        .insert(btn_type)
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn spawn_weapon_button(parent: &mut ChildBuilder, side: HandType, kind: WeaponType, label: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
            ..default()
        })
        .insert(WeaponButton { side, kind })
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 14.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn spawn_magic_panel(parent: &mut ChildBuilder, side: HandType) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(600.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                display: Display::None, // Hidden by default
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.2, 0.0, 0.2, 0.8)),
            ..default()
        })
        .insert(MagicPanel { side })
        .with_children(|panel| {
            let label = match side {
                HandType::Left => "Left Magic (Q): ",
                HandType::Right => "Right Magic (E): ",
            };
            panel.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Primary Cycle Button
            panel
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0)),
                    ..default()
                })
                .insert(MagicCycleButton {
                    side,
                    is_primary: true,
                })
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "Primary: Bolt",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Secondary Cycle Button
            panel
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0)),
                    ..default()
                })
                .insert(MagicCycleButton {
                    side,
                    is_primary: false,
                })
                .with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "Secondary: Blink",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

type WeaponButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static WeaponButton,
    ),
    (Changed<Interaction>, With<Button>),
>;

pub fn weapon_button_interaction(
    mut interaction_query: WeaponButtonQuery,
    mut hand_query: Query<(&mut Hand, &mut crate::components::weapon::Weapon)>,
) {
    for (interaction, mut color, button_data) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0)); // Green clicked

                // Update Player Hand
                for (mut hand, mut weapon) in &mut hand_query {
                    if hand.side == button_data.side {
                        hand.equipped_weapon = Some(button_data.kind);

                        // Update Weapon Stats based on Type
                        weapon.kind = button_data.kind;
                        match button_data.kind {
                            WeaponType::Magic => {
                                weapon.cooldown = 0.8; // Slower magic (User Request)
                                weapon.damage = 0.0; // Handled by spell projectile
                            }
                            WeaponType::Gun => {
                                weapon.cooldown = 0.5;
                                weapon.damage = 0.0; // Handled by projectile
                            }
                            WeaponType::Shuriken => {
                                weapon.cooldown = crate::configs::weapons::shuriken::COOLDOWN;
                                weapon.skill_cooldown =
                                    crate::configs::weapons::shuriken::SKILL_COOLDOWN;
                            }
                            WeaponType::Sword => {
                                weapon.cooldown = 0.5; // Default
                            }
                        }

                        println!("Equipped {:?} to {:?}", button_data.kind, button_data.side);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 1.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0));
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_shop_visibility(
    mut shop_query: Query<&mut Style, With<ShopMenu>>,
    round_manager: Res<crate::resources::round::RoundManager>,
) {
    for mut style in &mut shop_query {
        match round_manager.round_state {
            crate::resources::round::RoundState::Shop => {
                style.display = Display::Flex;
            }
            _ => {
                style.display = Display::None;
            }
        }
    }
}

type ShopButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static ShopButton,
    ),
    (Changed<Interaction>, With<Button>),
>;

pub fn shop_button_interaction(
    mut interaction_query: ShopButtonQuery,
    mut round_manager: ResMut<crate::resources::round::RoundManager>,
    mut player_query: Query<&mut crate::components::player::Player>,
) {
    for (interaction, mut color, btn_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
                match btn_type {
                    ShopButton::Heal => {
                        if let Ok(mut player) = player_query.get_single_mut() {
                            player.health = (player.health + 30.0).min(100.0);
                            println!("Healed! Health: {}", player.health);
                        }
                    }
                    ShopButton::DamageUp => {
                        // Needs global damage modifier or per-weapon?
                        // For now, print placeholder
                        println!("Damage Upgraded! (Placeholder)");
                    }
                    ShopButton::NextRound => {
                        // Force next round
                        round_manager
                            .round_timer
                            .set_duration(std::time::Duration::from_secs(0));
                        round_manager.round_timer.reset();
                        // The spawn_waves system checks finished() on this timer to switch state
                    }
                }
            }
            Interaction::Hovered => *color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 1.0)),
            Interaction::None => *color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_magic_ui(
    mut panel_query: Query<(&mut Style, &MagicPanel)>,
    hand_query: Query<&Hand>,
    // button_query: Query<(&mut Text, &MagicCycleButton), Without<MagicPanel>>,
    button_node_query: Query<(&Children, &MagicCycleButton)>,
    mut text_query: Query<&mut Text>,
    loadout_query: Query<&MagicLoadout>,
) {
    // 1. Update Panel Visibility
    for (mut style, panel) in &mut panel_query {
        let mut is_magic = false;
        for hand in hand_query.iter() {
            if hand.side == panel.side && hand.equipped_weapon == Some(WeaponType::Magic) {
                is_magic = true;
            }
        }
        if is_magic {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }

    // 2. Update Button Text from Loadout
    for (children, btn_data) in button_node_query.iter() {
        // Find Loadout for this hand
        let mut current_loadout = None;
        for (hand, loadout) in hand_query.iter().zip(loadout_query.iter()) {
            if hand.side == btn_data.side {
                current_loadout = Some(loadout);
            }
        }

        if let Some(loadout) = current_loadout {
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
            let prefix = if btn_data.is_primary {
                "Primary"
            } else {
                "Secondary"
            };

            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.sections[0].value = format!("{prefix}: {spell_name}");
                }
            }
        }
    }
}

type MagicButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static MagicCycleButton,
    ),
    (Changed<Interaction>, With<Button>),
>;

pub fn magic_button_interaction(
    mut interaction_query: MagicButtonQuery,
    mut loadout_query: Query<(&Hand, &mut MagicLoadout)>,
) {
    for (interaction, mut color, btn_data) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.6, 0.0, 0.6, 1.0));

                // Cycle Spell
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
            Interaction::Hovered => *color = BackgroundColor(Color::srgba(0.5, 0.0, 0.5, 1.0)),
            Interaction::None => *color = BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0)),
        }
    }
}
