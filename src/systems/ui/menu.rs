use super::components::{
    MagicCycleButton, MagicPanel, MenuCDRText, MenuCritText, MenuDamageText, MenuGoldText,
    MenuHealthText, MenuLifestealText, PurchaseEvent, ShopButton, WeaponButton, WeaponMenuUI,
};
use crate::components::player::{
    CombatStats, Currency, Hand, HandType, Health, Player, PlayerStats,
};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
use bevy::prelude::*;

#[allow(clippy::too_many_lines)]
pub fn spawn_weapon_menu(commands: &mut Commands) {
    // Weapon Selection Menu (Full Screen)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                display: Display::None,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
            WeaponMenuUI,
        ))
        .with_children(|parent| {
            // Status Panel (Top)
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)), // Status bar background
                ))
                .with_children(|status_bar| {
                    // Health
                    status_bar.spawn((
                        Text::new("HP: 100/100"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.0)),
                        MenuHealthText,
                    ));
                    // Gold
                    status_bar.spawn((
                        Text::new("Gold: 0"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                        MenuGoldText,
                    ));
                    // Damage Bonus
                    status_bar.spawn((
                        Text::new("Dmg: +0%"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.5, 0.0)),
                        MenuDamageText,
                    ));
                    // Crit
                    status_bar.spawn((
                        Text::new("Crit: 0% (x2.0)"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.2, 0.2)),
                        MenuCritText,
                    ));
                    // Lifesteal
                    status_bar.spawn((
                        Text::new("Life: 0%"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.0, 1.0)),
                        MenuLifestealText,
                    ));
                    // CDR
                    status_bar.spawn((
                        Text::new("CDR: 0%"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 1.0, 1.0)),
                        MenuCDRText,
                    ));
                });

            // Title
            parent.spawn((
                Text::new("WEAPON SELECTION"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Content Container (Weapons + Shop)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::horizontal(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|content| {
                    // --- LEFT HAND SECTION ---
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Percent(25.0),
                            ..default()
                        })
                        .with_children(|col| {
                            col.spawn((
                                Text::new("LEFT HAND"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            // Separator
                            col.spawn(Node {
                                height: Val::Px(10.0),
                                ..default()
                            });

                            spawn_weapon_button(col, HandType::Left, WeaponType::Shuriken, "Shuriken ❄");
                            spawn_weapon_button(col, HandType::Left, WeaponType::Sword, "Sword 🗡");
                            spawn_weapon_button(col, HandType::Left, WeaponType::Gun, "Gun 🔫");
                            spawn_weapon_button(col, HandType::Left, WeaponType::Magic, "Magic 🔮");

                            // Magic Panel (Initially Hidden, managed by logic)
                            // We place it here so it appears under the left column when active
                            spawn_magic_panel(col, HandType::Left);
                        });

                    // --- CENTER SHOP SECTION ---
                    content
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                width: Val::Percent(40.0),
                                height: Val::Percent(100.0), // Fill height
                                padding: UiRect::all(Val::Px(20.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgba(0.5, 0.5, 0.5, 0.5)),
                            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                        ))
                        .with_children(|shop_container| {
                            shop_container.spawn((
                                Text::new("SHOP"),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    margin: UiRect::bottom(Val::Px(20.0)),
                                    ..default()
                                },
                            ));

                            shop_container
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::FlexStart,
                                    width: Val::Percent(100.0),
                                    ..default()
                                })
                                .with_children(|shop_row| {
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::Heal,
                                        "Heal\n(+30 HP)\n50G",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::DamageUp,
                                        "Damage Up\n(+10%)\n100G",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::MaxHealthUp,
                                        "Max HP Up\n(+20)\n150G",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::CritDamageUp,
                                        "Crit Damage\n(+50%)\n200G",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::CritChanceUp,
                                        "Crit Chance\n(+10%)\n250G",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::LifestealUp,
                                        "Lifesteal\n(+10%)\n300G",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::CooldownReductionUp,
                                        "Magic CDR\n(+10%)\n350G",
                                    );
                                });
                        });

                    // --- RIGHT HAND SECTION ---
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Percent(25.0),
                            ..default()
                        })
                        .with_children(|col| {
                            col.spawn((
                                Text::new("RIGHT HAND"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            // Separator
                            col.spawn(Node {
                                height: Val::Px(10.0),
                                ..default()
                            });

                            spawn_weapon_button(col, HandType::Right, WeaponType::Shuriken, "Shuriken ❄");
                            spawn_weapon_button(col, HandType::Right, WeaponType::Sword, "Sword 🗡");
                            spawn_weapon_button(col, HandType::Right, WeaponType::Gun, "Gun 🔫");
                            spawn_weapon_button(
                                col,
                                HandType::Right,
                                WeaponType::Magic,
                                "Magic 🔮",
                            );

                            // Magic Panel
                            spawn_magic_panel(col, HandType::Right);
                        });
                });

            // Close Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        margin: UiRect::top(Val::Px(40.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
                ))
                .observe(
                    |_: On<Pointer<Click>>,
                     mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>,
                     mut round_manager: ResMut<crate::resources::round::RoundManager>| {
                        // Nếu đang ở Shop state thì bắt đầu round mới
                        if round_manager.round_state == crate::resources::round::RoundState::Shop {
                            round_manager.current_round += 1;
                            round_manager.enemies_to_spawn = crate::configs::enemy::BASE_ENEMY_COUNT
                                + (round_manager.current_round
                                    * crate::configs::enemy::ENEMY_COUNT_SCALING_PER_ROUND);
                            #[allow(clippy::cast_precision_loss)]
                            let exponent = round_manager.current_round as f32;
                            round_manager.spawn_timer = bevy::time::Timer::from_seconds(
                                crate::configs::enemy::BASE_SPAWN_INTERVAL
                                    * (crate::configs::enemy::SPAWN_INTERVAL_DECAY).powf(exponent),
                                bevy::time::TimerMode::Repeating,
                            );
                            round_manager.round_state = crate::resources::round::RoundState::Spawning;
                            println!("Starting Round {}!", round_manager.current_round);
                        }
                        next_state.set(crate::resources::game_state::GameState::Playing);
                    },
                )
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("BACK TO GAME"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

#[allow(clippy::too_many_lines)]
fn spawn_shop_button(parent: &mut ChildSpawnerCommands, btn_type: ShopButton, label: &str) {
    let is_blue = matches!(
        btn_type,
        ShopButton::CritChanceUp | ShopButton::LifestealUp | ShopButton::CooldownReductionUp
    );

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(140.0), // Square-ish card
                height: Val::Px(140.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column, // Stack text if needed
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            if is_blue {
                BorderColor::all(Color::srgb(0.0, 0.5, 1.0))
            } else {
                BorderColor::all(Color::WHITE)
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
            btn_type,
        ))
        .observe(
            |trigger: On<Pointer<Click>>,
             btn_query: Query<&ShopButton>,
             mut purchase_events: MessageWriter<PurchaseEvent>| {
                if let Ok(btn_type) = btn_query.get(trigger.entity) {
                    purchase_events.write(PurchaseEvent {
                        btn_type: *btn_type,
                        entity: trigger.entity,
                    });
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 1.0));
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_weapon_button(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    kind: WeaponType,
    label: &str,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
            WeaponButton { side, kind },
        ))
        .observe(
            |trigger: On<Pointer<Click>>,
             button_query: Query<&WeaponButton>,
             mut hand_query: Query<(&mut Hand, &mut crate::components::weapon::Weapon)>,
             mut color_query: Query<&mut BackgroundColor>| {
                if let Ok(button_data) = button_query.get(trigger.entity) {
                    if let Ok(mut color) = color_query.get_mut(trigger.entity) {
                        *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
                    }

                    for (mut hand, mut weapon) in &mut hand_query {
                        if hand.side == button_data.side {
                            hand.equipped_weapon = Some(button_data.kind);
                            weapon.kind = button_data.kind;

                            match button_data.kind {
                                WeaponType::Magic => {
                                    weapon.cooldown = 0.8;
                                    weapon.damage = 0.0;
                                }
                                WeaponType::Gun => {
                                    weapon.cooldown = 0.5;
                                    weapon.damage = 0.0;
                                }
                                WeaponType::Shuriken => {
                                    weapon.cooldown = crate::configs::weapons::shuriken::COOLDOWN;
                                    weapon.skill_cooldown =
                                        crate::configs::weapons::shuriken::SKILL_COOLDOWN;
                                }
                                WeaponType::Sword => {
                                    weapon.cooldown = 0.5;
                                }
                            }
                            println!("Equipped {:?} to {:?}", button_data.kind, button_data.side);
                        }
                    }
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 1.0));
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

#[allow(clippy::too_many_lines)]
fn spawn_magic_panel(parent: &mut ChildSpawnerCommands, side: HandType) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0), // Responsive to column width
                height: Val::Auto,          // Adjust to content
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(10.0)),
                display: Display::None, // Hidden by default
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.0, 0.2, 0.8)),
            MagicPanel { side },
        ))
        .with_children(|panel| {
            let label = match side {
                HandType::Left => "Left Magic (Q): ",
                HandType::Right => "Right Magic (E): ",
            };
            panel.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Primary Cycle Button
            panel
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0)),
                    MagicCycleButton {
                        side,
                        is_primary: true,
                    },
                ))
                .observe(magic_button_observer)
                .observe(
                    |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                        if let Ok(mut color) = color.get_mut(trigger.entity) {
                            *color = BackgroundColor(Color::srgba(0.5, 0.0, 0.5, 1.0));
                        }
                    },
                )
                .observe(
                    |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                        if let Ok(mut color) = color.get_mut(trigger.entity) {
                            *color = BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0));
                        }
                    },
                )
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Primary: Bolt"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Secondary Cycle Button
            panel
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0)),
                    MagicCycleButton {
                        side,
                        is_primary: false,
                    },
                ))
                .observe(magic_button_observer)
                .observe(
                    |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                        if let Ok(mut color) = color.get_mut(trigger.entity) {
                            *color = BackgroundColor(Color::srgba(0.5, 0.0, 0.5, 1.0));
                        }
                    },
                )
                .observe(
                    |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                        if let Ok(mut color) = color.get_mut(trigger.entity) {
                            *color = BackgroundColor(Color::srgba(0.4, 0.0, 0.4, 1.0));
                        }
                    },
                )
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Secondary: Blink"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

#[allow(clippy::needless_pass_by_value)]
fn magic_button_observer(
    trigger: On<Pointer<Click>>,
    btn_query: Query<&MagicCycleButton>,
    mut loadout_query: Query<(&Hand, &mut MagicLoadout)>,
) {
    if let Ok(btn_data) = btn_query.get(trigger.entity) {
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
}

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
