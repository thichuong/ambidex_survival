use super::super::components::{
    MenuCDRText, MenuCritText, MenuDamageText, MenuGoldText, MenuHealthText, MenuLifestealText,
    ShopButton, ShopBuyButton, ShopBuyButtonPrice, ShopBuyButtonText, TutorialButton,
    WeaponMenuRestartButton, WeaponMenuSettingsButton, WeaponMenuUI,
};
use super::arsenal::{spawn_magic_panel, spawn_weapon_button, spawn_weapon_detail_panel};
use super::shop::spawn_shop_button;
use crate::components::player::HandType;
use crate::components::weapon::WeaponType;
use bevy::prelude::*;

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn spawn_weapon_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.04, 0.06, 1.0)),
            WeaponMenuUI,
        ))
        .with_children(|parent| {
            // --- HEADER ---
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(120.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(20.0)),
                        border: UiRect::bottom(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.08, 0.1, 0.95)),
                    BorderColor::all(Color::srgba(0.2, 0.2, 0.25, 0.5)),
                ))
                .with_children(|header| {
                    header.spawn(Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    }).with_children(|status_bar| {
                        status_bar.spawn((
                            Text::new("HP: 100/100"),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::srgb(0.2, 1.0, 0.4)),
                            MenuHealthText,
                        ));
                        status_bar.spawn((
                            Text::new("Gold: 0"),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.8, 0.0)),
                            MenuGoldText,
                        ));
                        let stat_font = TextFont { font_size: 16.0, ..default() };
                        status_bar.spawn((Text::new("Dmg: +0%"), stat_font.clone(), TextColor(Color::srgb(1.0, 0.4, 0.1)), MenuDamageText));
                        status_bar.spawn((Text::new("Crit: 0%"), stat_font.clone(), TextColor(Color::srgb(1.0, 0.2, 0.2)), MenuCritText));
                        status_bar.spawn((Text::new("Life: 0%"), stat_font.clone(), TextColor(Color::srgb(1.0, 0.2, 1.0)), MenuLifestealText));
                        status_bar.spawn((Text::new("CDR: 0%"), stat_font, TextColor(Color::srgb(0.2, 0.8, 1.0)), MenuCDRText));
                    });
                    header.spawn((
                        Text::new("ARSENAL & UPGRADES"),
                        TextFont { font_size: 32.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });

            // --- CONTENT ---
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(20.0)),
                    overflow: Overflow::clip(),
                    ..default()
                })
                .with_children(|content| {
                    // LEFT HAND
                    content.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(28.0),
                        height: Val::Percent(100.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    }).with_children(|col| {
                        col.spawn((
                            Text::new("LEFT HAND"),
                            TextFont { font_size: 22.0, ..default() },
                            TextColor(Color::srgb(0.7, 0.7, 1.0)),
                            Node { margin: UiRect::bottom(Val::Px(15.0)), ..default() },
                        ));
                        col.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..default()
                        }).with_children(|row| {
                            row.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                width: Val::Percent(30.0),
                                ..default()
                            }).with_children(|btns| {
                                spawn_weapon_button(btns, HandType::Left, WeaponType::Shuriken, "Shuriken");
                                spawn_weapon_button(btns, HandType::Left, WeaponType::Sword, "Sword");
                                spawn_weapon_button(btns, HandType::Left, WeaponType::Gun, "Gun");
                                spawn_weapon_button(btns, HandType::Left, WeaponType::Magic, "Magic");
                            });
                            row.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                width: Val::Percent(70.0),
                                padding: UiRect::left(Val::Px(10.0)),
                                ..default()
                            }).with_children(|details| {
                                spawn_magic_panel(details, HandType::Left, &asset_server);
                                spawn_weapon_detail_panel(details, HandType::Left, &asset_server);
                            });
                        });
                    });

                    // CENTER SHOP
                    content.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Percent(40.0),
                            height: Val::Percent(100.0),
                            flex_grow: 1.0,
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            border: UiRect::horizontal(Val::Px(1.0)),
                            overflow: Overflow::scroll_y(),
                            ..default()
                        },
                        BorderColor::all(Color::srgba(0.2, 0.2, 0.3, 0.5)),
                    )).with_children(|shop_scroll| {
                        shop_scroll.spawn((
                            Text::new("SHOP UPGRADES"),
                            TextFont { font_size: 24.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.9, 0.5)),
                            Node { margin: UiRect::bottom(Val::Px(15.0)), ..default() },
                        ));
                        shop_scroll.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::FlexStart,
                            width: Val::Percent(100.0),
                            row_gap: Val::Px(10.0),
                            column_gap: Val::Px(10.0),
                            ..default()
                        }).with_children(|grid| {
                            spawn_shop_button(grid, ShopButton::Heal, "");
                            spawn_shop_button(grid, ShopButton::DamageUp, "");
                            spawn_shop_button(grid, ShopButton::MaxHealthUp, "");
                            spawn_shop_button(grid, ShopButton::CritDamageUp, "");
                            spawn_shop_button(grid, ShopButton::CritChanceUp, "");
                            spawn_shop_button(grid, ShopButton::LifestealUp, "");
                            spawn_shop_button(grid, ShopButton::CooldownReductionUp, "");
                            spawn_shop_button(grid, ShopButton::NovaCore, "");
                        });

                        // === SHOP BUY BUTTON (Large, below cards) ===
                        shop_scroll.spawn((
                            Button,
                            Node {
                                width: Val::Px(280.0),
                                height: Val::Px(80.0),
                                margin: UiRect::vertical(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(4.0),
                                border: UiRect::all(Val::Px(3.0)),
                                display: Display::None, // Hidden by default
                                ..default()
                            },
                            BorderColor::all(Color::srgb(1.0, 0.85, 0.0)), // Gold border
                            BorderRadius::all(Val::Px(12.0)),
                            BackgroundColor(Color::srgba(0.2, 0.15, 0.0, 0.95)),
                            ShopBuyButton,
                        ))
                        .observe(|trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) {
                                *color = BackgroundColor(Color::srgba(0.35, 0.28, 0.0, 1.0));
                            }
                        })
                        .observe(|trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) {
                                *color = BackgroundColor(Color::srgba(0.2, 0.15, 0.0, 0.95));
                            }
                        })
                        .with_children(|buy_btn| {
                            // Title line
                            buy_btn.spawn((
                                Text::new("Select an upgrade"),
                                TextFont { font_size: 22.0, ..default() },
                                TextColor(Color::srgb(1.0, 0.95, 0.7)),
                                ShopBuyButtonText,
                            ));
                            // Price line
                            buy_btn.spawn((
                                Text::new(""),
                                TextFont { font_size: 18.0, ..default() },
                                TextColor(Color::srgb(1.0, 0.85, 0.0)), // Gold color
                                ShopBuyButtonPrice,
                            ));
                        });

                        shop_scroll.spawn(Node { height: Val::Px(20.0), ..default() });
                    });

                    // RIGHT HAND
                    content.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(28.0),
                        height: Val::Percent(100.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    }).with_children(|col| {
                        col.spawn((
                            Text::new("RIGHT HAND"),
                            TextFont { font_size: 22.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.7, 0.7)),
                            Node { margin: UiRect::bottom(Val::Px(15.0)), ..default() },
                        ));
                        col.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..default()
                        }).with_children(|row| {
                            row.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                width: Val::Percent(30.0),
                                ..default()
                            }).with_children(|btns| {
                                spawn_weapon_button(btns, HandType::Right, WeaponType::Shuriken, "Shuriken");
                                spawn_weapon_button(btns, HandType::Right, WeaponType::Sword, "Sword");
                                spawn_weapon_button(btns, HandType::Right, WeaponType::Gun, "Gun");
                                spawn_weapon_button(btns, HandType::Right, WeaponType::Magic, "Magic");
                            });
                            row.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                width: Val::Percent(70.0),
                                padding: UiRect::left(Val::Px(10.0)),
                                ..default()
                            }).with_children(|details| {
                                spawn_magic_panel(details, HandType::Right, &asset_server);
                                spawn_weapon_detail_panel(details, HandType::Right, &asset_server);
                            });
                        });
                    });
                });
            // --- FOOTER ---
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(20.0),
                        border: UiRect::top(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.08, 0.1, 0.95)),
                    BorderColor::all(Color::srgba(0.2, 0.2, 0.25, 0.5)),
                ))
                .with_children(|footer| {
                    use crate::resources::game_state::GameState;
                    use crate::resources::round::{RoundManager, RoundState};



                    // BACK TO BATTLE
                    footer
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(0.3, 0.8, 0.3)),
                            BorderRadius::all(Val::Px(10.0)),
                            BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0)),
                        ))
                        .observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>, mut round_manager: ResMut<RoundManager>| {
                            if round_manager.round_state == RoundState::Shop {
                                round_manager.current_round += 1;
                                round_manager.enemies_to_spawn = crate::configs::enemy::BASE_ENEMY_COUNT + (round_manager.current_round * crate::configs::enemy::ENEMY_COUNT_SCALING_PER_ROUND);
                                #[allow(clippy::cast_precision_loss)]
                                let exponent = round_manager.current_round as f32;
                                round_manager.spawn_timer = bevy::time::Timer::from_seconds(
                                    crate::configs::enemy::BASE_SPAWN_INTERVAL * (crate::configs::enemy::SPAWN_INTERVAL_DECAY).powf(exponent),
                                    bevy::time::TimerMode::Repeating,
                                );
                                round_manager.elites_to_spawn = round_manager.current_round;
                                round_manager.yellow_enemies_to_spawn = u32::from(round_manager.current_round >= 5);
                                round_manager.round_state = RoundState::Spawning;
                            }
                            round_manager.has_started = true;
                            next_state.set(GameState::Playing);
                        })
                        .observe(|trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.25, 0.45, 0.25, 1.0)); }
                        })
                        .observe(|trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0)); }
                        })
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("GO TO BATTLE"),
                                TextFont { font_size: 20.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // TUTORIAL
                    footer
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(0.2, 0.6, 1.0)),
                            BorderRadius::all(Val::Px(10.0)),
                            BackgroundColor(Color::srgba(0.15, 0.2, 0.3, 1.0)),
                            TutorialButton,
                        ))
                        .observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>, mut prev_state: ResMut<crate::resources::game_state::PreviousMenuState>| {
                            prev_state.0 = GameState::WeaponMenu;
                            next_state.set(GameState::Tutorial);
                        })
                        .observe(|trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.25, 0.4, 0.6, 1.0)); }
                        })
                        .observe(|trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.15, 0.2, 0.3, 1.0)); }
                        })
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("TUTORIAL"),
                                TextFont { font_size: 18.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // SETTINGS
                    footer
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
                            BorderRadius::all(Val::Px(10.0)),
                            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                            WeaponMenuSettingsButton,
                        ))
                        .observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>, mut prev_state: ResMut<crate::resources::game_state::PreviousMenuState>| {
                            prev_state.0 = GameState::WeaponMenu;
                            next_state.set(GameState::Settings);
                        })
                        .observe(|trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)); }
                        })
                        .observe(|trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)); }
                        })
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("SETTINGS"),
                                TextFont { font_size: 18.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // NEW GAME
                    footer
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::srgb(1.0, 0.3, 0.3)),
                            BorderRadius::all(Val::Px(10.0)),
                            BackgroundColor(Color::srgba(0.3, 0.15, 0.15, 1.0)),
                            WeaponMenuRestartButton,
                        ))
                        .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                            super::spawn_confirmation_dialog(&mut commands);
                        })
                        .observe(|trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.45, 0.25, 0.25, 1.0)); }
                        })
                        .observe(|trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                            if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.3, 0.15, 0.15, 1.0)); }
                        })
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("NEW GAME"),
                                TextFont { font_size: 18.0, ..default() },
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
