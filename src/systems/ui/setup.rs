use super::components::{GoldText, RoundText, HealthBar, HealthText, HUDHandIndicator, HUDIcon, ShurikenCountText, CooldownOverlay, MenuButton, WeaponMenuUI, MenuHealthText, MenuGoldText, MenuDamageText, MenuCritText, MenuLifestealText, MenuCDRText, ShopButton, GameOverUI, NewGameButton, PurchaseEvent, WeaponButton, MagicPanel, MagicCycleButton};
use crate::components::player::{
    CombatStats, Currency, Hand, HandType, Health, Player, Progression,
};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
use bevy::prelude::*;

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root UI Node (HUD)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            // Gold Display (Top Left)
            parent.spawn((
                Text::new("Gold: 0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.843, 0.0)), // Gold Color
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                },
                GoldText,
            ));

            // Round Display (Top Right)
            parent.spawn((
                Text::new("Round: 1"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..default()
                },
                RoundText,
            ));

            // Health Bar (Top Center)
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Percent(50.0),
                    margin: UiRect::left(Val::Px(-100.0)), // Center alignment (half of width)
                    width: Val::Px(200.0),
                    height: Val::Px(20.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                })
                .insert(BorderColor::all(Color::WHITE))
                .insert(BackgroundColor(Color::BLACK))
                .with_children(|bar| {
                    bar.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
                        HealthBar,
                    ));
                });

            // Health Text
            parent.spawn((
                Text::new("100 / 100"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(22.0), // Slightly below top align to center with bar visually or just beside
                    left: Val::Percent(50.0),
                    margin: UiRect::left(Val::Px(110.0)), // Offset to the right of the bar (100px half width + padding)
                    ..default()
                },
                HealthText,
            ));
            // Left Hand Indicator
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(120.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                    BorderColor::all(Color::WHITE),
                    HUDHandIndicator {
                        side: HandType::Left,
                    },
                ))
                .with_children(|btn| {
                    btn.spawn((
                        ImageNode::new(asset_server.load("ui/icons/shuriken.png")),
                        Node {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            ..default()
                        },
                        HUDIcon { side: HandType::Left },
                    ));

                    // Shuriken Count Text
                    btn.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(5.0),
                            right: Val::Px(10.0),
                            ..default()
                        },
                        ShurikenCountText {
                            side: HandType::Left,
                        },
                    ));

                    // Cooldown Overlay
                    btn.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(0.0), // Start empty
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                        CooldownOverlay {
                            side: HandType::Left,
                        },
                    ));
                });

            // Center Menu Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
                    MenuButton,
                ))
                .observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>| {
                    next_state.set(crate::resources::game_state::GameState::WeaponMenu);
                })
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("MENU"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Right Hand Indicator
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(120.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                    BorderColor::all(Color::WHITE),
                    HUDHandIndicator {
                        side: HandType::Right,
                    },
                ))
                .with_children(|btn| {
                    btn.spawn((
                        ImageNode::new(asset_server.load("ui/icons/shuriken.png")),
                        Node {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            ..default()
                        },
                        HUDIcon { side: HandType::Right },
                    ));

                    // Shuriken Count Text
                    btn.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(5.0),
                            right: Val::Px(10.0),
                            ..default()
                        },
                        ShurikenCountText {
                            side: HandType::Right,
                        },
                    ));

                    // Cooldown Overlay
                    btn.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(0.0), // Start empty
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                        CooldownOverlay {
                            side: HandType::Right,
                        },
                    ));
                });
        });

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

                            spawn_weapon_button(
                                col,
                                HandType::Left,
                                WeaponType::Shuriken,
                                "Shuriken ❄",
                            );
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

                            spawn_weapon_button(
                                col,
                                HandType::Right,
                                WeaponType::Shuriken,
                                "Shuriken ❄",
                            );
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
                .observe(|_: On<Pointer<Click>>,
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
                })
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

    // Game Over Menu (Initially Hidden)
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(240.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
                    NewGameButton,
                ))
                .observe(
                    |_trigger: On<Pointer<Click>>,
                     mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>,
                     mut player_query: Query<
                        (
                            &mut Health,
                            &mut Currency,
                            &mut CombatStats,
                            &mut Progression,
                            &mut Transform,
                        ),
                        With<Player>,
                    >,
                     mut round_manager: ResMut<crate::resources::round::RoundManager>,
                     enemy_query: Query<Entity, With<crate::components::enemy::Enemy>>,
                     projectile_query: Query<
                        Entity,
                        With<crate::components::weapon::Projectile>,
                    >,
                     mut commands: Commands| {
                        // Reset Player
                        if let Ok((
                            mut health,
                            mut currency,
                            mut combat,
                            mut progression,
                            mut transform,
                        )) = player_query.single_mut()
                        {
                            *health = Health::default();
                            *currency = Currency::default();
                            *combat = CombatStats::default();
                            *progression = Progression::default();
                            transform.translation = Vec3::ZERO;
                        }

                        // Reset Round
                        *round_manager = crate::resources::round::RoundManager::default();

                        // Despawn Enemies
                        for entity in &enemy_query {
                            commands.entity(entity).despawn();
                        }

                        // Despawn Projectiles
                        for entity in &projectile_query {
                            commands.entity(entity).despawn();
                        }

                        // Restart Game
                        next_state.set(crate::resources::game_state::GameState::Playing);
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
                        Text::new("NEW GAME"),
                        TextFont {
                            font_size: 32.0,
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
