use super::components::{
    MagicCycleButton, MagicPanel, MenuCDRText, MenuCritText, MenuDamageText, MenuGoldText,
    MenuHealthText, MenuLifestealText, PurchaseEvent, ShopButton, TutorialButton, WeaponButton,
    WeaponDetailPanel, WeaponMenuUI, WeaponStateGroup,
};
use crate::components::player::{
    CombatStats, Currency, Hand, HandType, Health, Player, PlayerStats,
};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};

use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponDescriptionText;

pub fn get_spell_description(spell_type: SpellType) -> String {
    match spell_type {
        SpellType::EnergyBolt => "Energy Bolt: Creates a large explosion on impact.".to_string(),
        SpellType::Laser => "Laser: Instant-hit high-velocity beam.".to_string(),
        SpellType::Nova => "Nova: Radial burst of high area damage.".to_string(),
        SpellType::Blink => "Blink: Teleport to cursor & Invulnerable.".to_string(),
        SpellType::Global => "Global: Massive strike hitting ALL enemies.".to_string(),
    }
}

pub fn get_weapon_description(weapon_type: WeaponType, loadout: Option<&MagicLoadout>) -> String {
    match weapon_type {
        WeaponType::Sword => {
            "Sword (Melee)\n\nNormal Mode: Moderate range, high damage.\nShattered Mode (Skill): Blade fragments cover massive area, lower damage.".to_string()
        }
        WeaponType::Gun => {
            "Gun (Firearm)\n\nModes: Single, Shotgun, Rapid.\nSkill Cycle: Toggle between modes.\nRapid: Hold to spray.".to_string()
        }
        WeaponType::Shuriken => "Shuriken (Utility)\n\nAttack: Throw fast-moving stars (Max 12).\nSkill: Teleport to nearest shuriken.\nGreat for dodging.".to_string(),
        WeaponType::Magic => {
        loadout.map_or_else(
            || "Magic (Spellcasting)\n\nMost customizable weapon.\nTwo spell slots (Primary/Secondary).\nSelect a spell to see details.".to_string(),
            |loadout| {
                let p_desc = get_spell_description(loadout.primary);
                let s_desc = get_spell_description(loadout.secondary);
                format!("Magic (Spellcasting)\n\nPrimary - {p_desc}\n\nSecondary - {s_desc}\n\nSkill: Toggle Spell Slot.\nBenefits from CDR upgrades.")
            },
        )
        }
    }
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn spawn_weapon_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Weapon Selection Menu (Full Screen)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                display: Display::Flex, // Changed from None to Flex as we spawn on enter
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart, // Align to top
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
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
                            width: Val::Percent(30.0),
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

                            // Content Row (Buttons | Description)
                            col.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::FlexStart,
                                ..default()
                            }).with_children(|row| {
                                // Column 1: Buttons
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(35.0),
                                    ..default()
                                }).with_children(|btns| {
                                    spawn_weapon_button(btns, HandType::Left, WeaponType::Shuriken, "Shuriken");
                                    spawn_weapon_button(btns, HandType::Left, WeaponType::Sword, "Sword");
                                    spawn_weapon_button(btns, HandType::Left, WeaponType::Gun, "Gun");
                                    spawn_weapon_button(btns, HandType::Left, WeaponType::Magic, "Magic");
                                });

                                // Column 2: Details & Magic Panel
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(65.0),
                                    ..default()
                                }).with_children(|details| {
                                    // Magic Panel (Initially Hidden, managed by logic)
                                    spawn_magic_panel(details, HandType::Left, &asset_server);
                                    spawn_weapon_detail_panel(details, HandType::Left, &asset_server);
                                });
                            });
                        });

                    // --- CENTER SHOP SECTION ---
                    content
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                width: Val::Percent(40.0),
                                min_width: Val::Px(420.0),
                                height: Val::Auto,
                                flex_grow: 1.0,
                                padding: UiRect::all(Val::Px(16.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            BorderColor::all(Color::srgba(0.4, 0.4, 0.5, 0.8)),
                            BorderRadius::all(Val::Px(16.0)),
                            BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.9)),
                        ))
                        .with_children(|shop_container| {
                            // Shop Title with gradient-like styling
                            shop_container.spawn((
                                Text::new("⚒️ SHOP"),
                                TextFont {
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.9, 0.5)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(16.0)),
                                    ..default()
                                },
                            ));

                            // Cards Grid Container
                            shop_container
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::FlexStart,
                                    align_content: AlignContent::FlexStart,
                                    width: Val::Percent(100.0),
                                    row_gap: Val::Px(8.0),
                                    column_gap: Val::Px(8.0),
                                    ..default()
                                })
                                .with_children(|shop_row| {
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::Heal,
                                        "",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::DamageUp,
                                        "",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::MaxHealthUp,
                                        "",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::CritDamageUp,
                                        "",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::CritChanceUp,
                                        "",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::LifestealUp,
                                        "",
                                    );
                                    spawn_shop_button(
                                        shop_row,
                                        ShopButton::CooldownReductionUp,
                                        "",
                                    );
                                });
                        });

                    // --- RIGHT HAND SECTION ---
                    content
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Percent(30.0),
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

                            // Content Row (Buttons | Description)
                            col.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::FlexStart,
                                ..default()
                            }).with_children(|row| {
                                // Column 1: Buttons
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(35.0),
                                    ..default()
                                }).with_children(|btns| {
                                    spawn_weapon_button(btns, HandType::Right, WeaponType::Shuriken, "Shuriken");
                                    spawn_weapon_button(btns, HandType::Right, WeaponType::Sword, "Sword");
                                    spawn_weapon_button(btns, HandType::Right, WeaponType::Gun, "Gun");
                                    spawn_weapon_button(btns, HandType::Right, WeaponType::Magic, "Magic");
                                });

                                // Column 2: Details & Magic Panel
                                row.spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(65.0),
                                    ..default()
                                }).with_children(|details| {
                                    spawn_magic_panel(details, HandType::Right, &asset_server);
                                    spawn_weapon_detail_panel(details, HandType::Right, &asset_server);
                                });
                            });
                        });
                });

            // Footer Container (Pushed to bottom) - Horizontal layout
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    width: Val::Percent(100.0),
                    margin: UiRect::top(Val::Px(16.0)),
                    padding: UiRect::vertical(Val::Px(16.0)),
                    ..default()
                })
                .with_children(|footer| {
                    // Close Button
                    footer
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(180.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(0.4, 0.8, 0.4)),
                            BorderRadius::all(Val::Px(8.0)),
                            BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
                        ))
                        .observe(
                            |_: On<Pointer<Click>>,
                             mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>,
                             mut round_manager: ResMut<crate::resources::round::RoundManager>| {
                                // Nếu đang ở Shop state thì bắt đầu round mới
                                if round_manager.round_state
                                    == crate::resources::round::RoundState::Shop
                                {
                                    round_manager.current_round += 1;
                                    round_manager.enemies_to_spawn =
                                        crate::configs::enemy::BASE_ENEMY_COUNT
                                            + (round_manager.current_round
                                                * crate::configs::enemy::ENEMY_COUNT_SCALING_PER_ROUND);
                                    #[allow(clippy::cast_precision_loss)]
                                    let exponent = round_manager.current_round as f32;
                                    round_manager.spawn_timer = bevy::time::Timer::from_seconds(
                                        crate::configs::enemy::BASE_SPAWN_INTERVAL
                                            * (crate::configs::enemy::SPAWN_INTERVAL_DECAY)
                                                .powf(exponent),
                                        bevy::time::TimerMode::Repeating,
                                    );
                                    round_manager.round_state =
                                        crate::resources::round::RoundState::Spawning;
                                    println!("Starting Round {}!", round_manager.current_round);
                                }
                                next_state.set(crate::resources::game_state::GameState::Playing);
                            },
                        )
                        .observe(
                            |trigger: On<Pointer<Over>>,
                             mut color: Query<&mut BackgroundColor>| {
                                if let Ok(mut color) = color.get_mut(trigger.entity) {
                                    *color = BackgroundColor(Color::srgba(0.2, 0.6, 0.2, 1.0));
                                }
                            },
                        )
                        .observe(
                            |trigger: On<Pointer<Out>>,
                             mut color: Query<&mut BackgroundColor>| {
                                if let Ok(mut color) = color.get_mut(trigger.entity) {
                                    *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0));
                                }
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

                    // Tutorial Button
                    footer
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(180.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(0.0, 0.8, 1.0)),
                            BorderRadius::all(Val::Px(8.0)),
                            BackgroundColor(Color::srgba(0.1, 0.2, 0.3, 1.0)),
                            TutorialButton,
                        ))
                        .observe(
                            |_: On<Pointer<Click>>,
                             mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>| {
                                next_state
                                    .set(crate::resources::game_state::GameState::Tutorial);
                            },
                        )
                        .observe(
                            |trigger: On<Pointer<Over>>,
                             mut color: Query<&mut BackgroundColor>| {
                                if let Ok(mut color) = color.get_mut(trigger.entity) {
                                    *color = BackgroundColor(Color::srgba(0.2, 0.4, 0.6, 1.0));
                                }
                            },
                        )
                        .observe(
                            |trigger: On<Pointer<Out>>,
                             mut color: Query<&mut BackgroundColor>| {
                                if let Ok(mut color) = color.get_mut(trigger.entity) {
                                    *color = BackgroundColor(Color::srgba(0.1, 0.2, 0.3, 1.0));
                                }
                            },
                        )
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("TUTORIAL"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn despawn_weapon_menu(mut commands: Commands, query: Query<Entity, With<WeaponMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[allow(clippy::too_many_lines)]
fn spawn_shop_button(parent: &mut ChildSpawnerCommands, btn_type: ShopButton, _label: &str) {
    // Determine card type: BLUE (Advanced) or WHITE (Basic)
    let is_blue = matches!(
        btn_type,
        ShopButton::CritChanceUp | ShopButton::LifestealUp | ShopButton::CooldownReductionUp
    );

    // Get icon, title, description, and price for each upgrade type
    let (icon, title, desc, price) = match btn_type {
        ShopButton::Heal => ("❤️", "Heal", "+30 HP", "50G"),
        ShopButton::DamageUp => ("⚔️", "Damage", "+10%", "100G"),
        ShopButton::MaxHealthUp => ("💖", "Max HP", "+20", "150G"),
        ShopButton::CritDamageUp => ("💥", "Crit Dmg", "+50%", "200G"),
        ShopButton::CritChanceUp => ("🎯", "Crit Rate", "+10%", "250G"),
        ShopButton::LifestealUp => ("🩸", "Lifesteal", "+10%", "300G"),
        ShopButton::CooldownReductionUp => ("⏱️", "CDR", "+10%", "350G"),
    };

    // Card colors based on type
    let (border_color, bg_color, bg_hover, text_accent) = if is_blue {
        (
            Color::srgb(0.0, 0.7, 1.0),          // Bright cyan border
            Color::srgba(0.0, 0.12, 0.25, 0.95), // Dark blue background
            Color::srgba(0.0, 0.2, 0.4, 1.0),    // Hover blue
            Color::srgb(0.4, 0.9, 1.0),          // Cyan accent text
        )
    } else {
        (
            Color::srgb(0.9, 0.9, 0.95),         // Soft white border
            Color::srgba(0.15, 0.15, 0.2, 0.95), // Dark gray background
            Color::srgba(0.25, 0.25, 0.3, 1.0),  // Hover gray
            Color::srgb(1.0, 1.0, 1.0),          // White accent text
        )
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(130.0),
                min_width: Val::Px(100.0),
                height: Val::Px(160.0),
                min_height: Val::Px(140.0),
                margin: UiRect::all(Val::Px(8.0)),
                padding: UiRect::all(Val::Px(12.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(12.0)),
            BackgroundColor(bg_color),
            btn_type,
        ))
        .observe(
            |trigger: On<Pointer<Click>>,
             btn_query: Query<&ShopButton>,
             mut events: MessageWriter<PurchaseEvent>| {
                if let Ok(btn_type) = btn_query.get(trigger.entity) {
                    events.write(PurchaseEvent {
                        btn_type: *btn_type,
                        entity: trigger.entity,
                    });
                }
            },
        )
        .observe(
            move |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(bg_hover);
                }
            },
        )
        .observe(
            move |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(bg_color);
                }
            },
        )
        .with_children(|card| {
            // Icon (large emoji at top)
            card.spawn((
                Text::new(icon),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                },
            ));

            // Title (upgrade name)
            card.spawn((
                Text::new(title),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(text_accent),
            ));

            // Description (effect)
            card.spawn((
                Text::new(desc),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Price container
            card.spawn((
                Node {
                    margin: UiRect::top(Val::Px(8.0)),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                BorderRadius::all(Val::Px(6.0)),
            ))
            .with_children(|price_box| {
                price_box.spawn((
                    Text::new(format!("💰 {price}")),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.85, 0.0)), // Gold color
                ));
            });
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
             mut hand_query: Query<(&mut Hand, &mut crate::components::weapon::Weapon)>| {
                if let Ok(button_data) = button_query.get(trigger.entity) {
                    // Update internal state
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
        // Removed Observers that fight with system based highlighting
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
fn spawn_magic_panel(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(10.0)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.0, 0.2, 0.8)),
            MagicPanel { side },
        ))
        .with_children(|panel| {
            // Label "Select Spell"
            panel.spawn((
                Text::new("Select Spell"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.5, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

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

            // Icons Container
            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|icons| {
                    // Primary Icon (Left)
                    icons
                        .spawn((
                            ImageNode::new(asset_server.load("ui/icons/magic_bolt.png")), // Default
                            Node {
                                width: Val::Px(80.0),
                                height: Val::Px(80.0),
                                margin: UiRect::right(Val::Px(10.0)),
                                ..default()
                            },
                            MagicCycleButton {
                                side,
                                is_primary: true,
                            }, // Re-using this component for tracking, but maybe we need separate one for icon?
                               // Actually, let's keep it simple. The button below controls it.
                               // But I need to update this icon.
                               // I'll attach a marker to this logic.
                        ))
                        .insert(MagicCycleButton {
                            side,
                            is_primary: true,
                        });

                    // Secondary Icon (Right)
                    icons.spawn((
                        ImageNode::new(asset_server.load("ui/icons/magic_blink.png")), // Default
                        Node {
                            width: Val::Px(60.0),
                            height: Val::Px(60.0), // Slightly smaller
                            ..default()
                        },
                        MagicCycleButton {
                            side,
                            is_primary: false,
                        },
                    ));
                });

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

fn spawn_weapon_detail_panel(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(10.0)),
                display: Display::None,
                ..default()
            },
            WeaponDetailPanel { side },
        ))
        .with_children(|panel| {
            // Description Text
            panel.spawn((
                Text::new("Description..."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    max_width: Val::Px(250.0), // Limit width for wrapping
                    ..default()
                },
                WeaponDescriptionText,
            ));

            spawn_sword_group(panel, side, asset_server);
            spawn_gun_group(panel, side, asset_server);
            spawn_shuriken_group(panel, side, asset_server);
        });
}

fn spawn_sword_group(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None, // Hidden by default
                ..default()
            },
            WeaponStateGroup {
                side,
                weapon_type: WeaponType::Sword,
            },
        ))
        .with_children(|group| {
            // Normal
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/sword_normal.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
            // Shattered
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/sword_shattered.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_gun_group(parent: &mut ChildSpawnerCommands, side: HandType, asset_server: &AssetServer) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            WeaponStateGroup {
                side,
                weapon_type: WeaponType::Gun,
            },
        ))
        .with_children(|group| {
            // Single
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/gun_single.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
            // Shotgun
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/gun_shotgun.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
            // Rapid
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/gun_rapid.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    margin: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_shuriken_group(
    parent: &mut ChildSpawnerCommands,
    side: HandType,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            WeaponStateGroup {
                side,
                weapon_type: WeaponType::Shuriken,
            },
        ))
        .with_children(|group| {
            group.spawn((
                ImageNode::new(asset_server.load("ui/icons/shuriken.png")),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(80.0),
                    ..default()
                },
            ));
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
pub fn update_menu_magic_ui(
    mut panel_query: Query<(&mut Node, &MagicPanel)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
    button_node_query: Query<(&Children, &MagicCycleButton), With<Button>>,
    mut icon_query: Query<(&mut ImageNode, &MagicCycleButton), Without<Button>>,
    mut text_query: Query<&mut Text>,
    loadout_query: Query<&MagicLoadout>,
    asset_server: Res<AssetServer>,
) {
    // 1. Update Panel Visibility
    for (mut node, panel) in &mut panel_query {
        let show = hand_query
            .iter()
            .any(|(h, weapon)| h.side == panel.side && weapon.kind == WeaponType::Magic);
        node.display = if show { Display::Flex } else { Display::None };
    }

    // 2. Update Icons and Text
    for (hand, _) in &hand_query {
        // Find corresponding loadout
        // Note: MagicLoadout is likely on the same entity as Hand? Or Player?
        // Checking components/weapon.rs: MagicLoadout is a component.
        // Assuming Hand entity has MagicLoadout.
        // But the previous code used `hand_query.iter().zip(loadout_query.iter())`.
        // This relies on matching order, which is risky if they aren't on same entity or query isn't aligned.
        // Best to query together if possible.
        // Let's assume they are separate entities for now matching previous logic, OR check if I can join them.
        // Previous logic:
        /*
        if let Some((_, loadout)) = hand_query
            .iter()
            .zip(loadout_query.iter())
            .find(|(hand, _)| hand.side == btn_data.side)
        */
        // This implies 1-to-1 match.

        // Let's use two loops matching by side.

        // Find loadout for this hand
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
            // Update Buttons Text
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

            // Update Icons
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

#[allow(clippy::needless_pass_by_value)]
pub fn update_menu_weapon_details_ui(
    mut panel_query: Query<(&mut Node, &Children, &WeaponDetailPanel)>,
    mut group_query: Query<(&mut Node, &WeaponStateGroup), Without<WeaponDetailPanel>>,
    mut text_query: Query<&mut Text, With<WeaponDescriptionText>>,
    hand_query: Query<(
        &Hand,
        &crate::components::weapon::Weapon,
        Option<&MagicLoadout>,
    )>,
) {
    for (mut panel_node, children, panel) in &mut panel_query {
        // Find active weapon and loadout for this side
        let active_data = hand_query.iter().find(|(h, _, _)| h.side == panel.side);

        if let Some((_, weapon, loadout_opt)) = active_data {
            // Always show panel if weapon is selected
            panel_node.display = Display::Flex;
            let weapon_kind = weapon.kind;

            // Update Description
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

            // Show matching group, hide others
            // Note: Magic has no "group" in this logic (no specific icons in detail panel except maybe generic?)
            // If Magic, we hide all weapon groups.
            for (mut group_node, group) in &mut group_query {
                if group.side == panel.side {
                    if group.weapon_type == weapon_kind {
                        group_node.display = Display::Flex;
                    } else {
                        group_node.display = Display::None;
                    }
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
        text.0 = format!("Life Steal: {life:.0}%");
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

pub fn update_menu_weapon_buttons(
    mut button_query: Query<(&WeaponButton, &mut BackgroundColor)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
) {
    // Iterate all buttons and check if they match the equipped weapon for their side
    for (button, mut color) in &mut button_query {
        // Find the hand matching the button's side
        let is_active = hand_query
            .iter()
            .any(|(h, weapon)| h.side == button.side && weapon.kind == button.kind);

        if is_active {
            *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0)); // Green for active
        } else {
            *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)); // Gray for inactive
        }
    }
}
