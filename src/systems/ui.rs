use crate::components::player::{Hand, HandType};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};

use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponButton {
    pub side: HandType,
    pub kind: WeaponType,
}

// ShopMenu component removed
#[derive(Component)]
pub enum ShopButton {
    Heal,
    DamageUp,
}

#[derive(Component)]
pub struct WeaponMenuUI;

#[derive(Component)]
pub struct HUDHandIndicator {
    #[allow(dead_code)]
    pub side: HandType,
}

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct MagicPanel {
    #[allow(dead_code)]
    pub side: HandType,
}

#[derive(Component)]
pub struct MagicCycleButton {
    pub side: HandType,
    pub is_primary: bool, // true = primary, false = secondary
}

#[derive(Component)]
pub struct HUDIcon {
    pub side: HandType,
}

#[derive(Component)]
pub struct CooldownOverlay {
    pub side: HandType,
}

#[derive(Component)]
pub struct ShurikenCountText {
    pub side: HandType,
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct MenuGoldText;

#[derive(Component)]
pub struct RoundText;

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.99)),
            WeaponMenuUI,
        ))
        .with_children(|parent| {
            // Title and Gold Row
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(50.0)),
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("WEAPON SELECTION"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    header.spawn((
                        Text::new("Gold: 0"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.843, 0.0)),
                        MenuGoldText,
                    ));
                });

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

    // Shop Menu (Removed - merged into WeaponMenuUI)
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
                     mut player_query: Query<(
                        &mut crate::components::player::Player,
                        &mut Transform,
                    )>,
                     mut round_manager: ResMut<crate::resources::round::RoundManager>,
                     enemy_query: Query<Entity, With<crate::components::enemy::Enemy>>,
                     projectile_query: Query<
                        Entity,
                        With<crate::components::weapon::Projectile>,
                    >,
                     mut commands: Commands| {
                        // Reset Player
                        if let Some((mut player, mut transform)) = player_query.iter_mut().next() {
                            player.health = 100.0;
                            player.gold = 0; // Reset Gold
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

fn spawn_shop_button(parent: &mut ChildSpawnerCommands, btn_type: ShopButton, label: &str) {
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
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
            btn_type,
        ))
        .observe(
            |trigger: On<Pointer<Click>>,
             btn_query: Query<&ShopButton>,
             mut player_query: Query<&mut crate::components::player::Player>,
             mut color_query: Query<&mut BackgroundColor>,
             children_query: Query<&Children>,
             mut text_query: Query<&mut Text>| {
                if let Ok(btn_type) = btn_query.get(trigger.entity) {
                    let mut success = false;

                    match btn_type {
                        ShopButton::Heal => {
                            if let Ok(mut player) = player_query.single_mut() {
                                if player.gold >= 50 && player.health < 100.0 {
                                    player.gold -= 50;
                                    player.health = (player.health + 30.0).min(100.0);
                                    println!(
                                        "Healed! Health: {}, Gold: {}",
                                        player.health, player.gold
                                    );
                                    success = true;
                                } else {
                                    println!("Not enough gold or full health!");
                                }
                            }
                        }
                        ShopButton::DamageUp => {
                            if let Ok(mut player) = player_query.single_mut() {
                                if player.gold >= 100 {
                                    player.gold -= 100;
                                    player.damage_multiplier += 0.1;
                                    println!("Damage Upgraded! Gold: {}", player.gold);
                                    success = true;

                                    // Update button text to show current multiplier
                                    if let Ok(children) = children_query.get(trigger.entity) {
                                        for &child in children {
                                            if let Ok(mut text) = text_query.get_mut(child) {
                                                text.0 = format!(
                                                    "Damage Up\n(+10%)\n100G\n(Current: {:.0}%)",
                                                    player.damage_multiplier * 100.0
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    println!("Not enough gold!");
                                }
                            }
                        }
                    }

                    if success {
                        if let Ok(mut color) = color_query.get_mut(trigger.entity) {
                            *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
                        }
                    } else if let Ok(mut color) = color_query.get_mut(trigger.entity) {
                        *color = BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 1.0)); // Flash red on fail
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
                    *color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
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

#[allow(
    clippy::unnecessary_wraps,
    clippy::needless_pass_by_value,
    clippy::type_complexity
)]
pub fn update_ui_visibility(
    // Removed ShopMenu query as it's no longer used separatedly
    mut weapon_menu_query: Query<&mut Node, (With<WeaponMenuUI>, Without<GameOverUI>)>,
    mut game_over_query: Query<&mut Node, (With<GameOverUI>, Without<WeaponMenuUI>)>,
    game_state: Res<State<crate::resources::game_state::GameState>>,
    round_manager: Res<crate::resources::round::RoundManager>,
) -> Result<(), String> {
    // 1. Weapon Menu / Shop Visibility
    // Show if manually opened (GameState::WeaponMenu) OR if it's Shop phase (RoundState::Shop)
    for mut node in &mut weapon_menu_query {
        node.display = if *game_state.get() == crate::resources::game_state::GameState::WeaponMenu
            || round_manager.round_state == crate::resources::round::RoundState::Shop
        {
            Display::Flex
        } else {
            Display::None
        };
    }

    // 2. Game Over Visibility
    for mut node in &mut game_over_query {
        node.display = if *game_state.get() == crate::resources::game_state::GameState::GameOver {
            Display::Flex
        } else {
            Display::None
        };
    }

    Ok(())
}

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
pub fn update_magic_ui(
    mut panel_query: Query<(&mut Node, &MagicPanel)>,
    hand_query: Query<&Hand>,
    button_node_query: Query<(&Children, &MagicCycleButton)>,
    mut text_query: Query<&mut Text>,
    loadout_query: Query<&MagicLoadout>,
) -> Result<(), String> {
    // 1. Update Panel Visibility (Always visible in Menu)
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

#[allow(clippy::unnecessary_wraps)]
pub fn update_health_ui(
    mut health_bar_query: Query<&mut Node, With<HealthBar>>,
    mut health_text_query: Query<&mut Text, With<HealthText>>,
    player_query: Query<&crate::components::player::Player>,
) -> Result<(), String> {
    if let Ok(player) = player_query.single() {
        for mut node in &mut health_bar_query {
            // Player health is 0..100
            let percent = (player.health / 100.0).clamp(0.0, 1.0) * 100.0;
            node.width = Val::Percent(percent);
        }

        for mut text in &mut health_text_query {
            **text = format!("{:.0} / 100", player.health);
        }
    }
    Ok(())
}

#[allow(clippy::type_complexity)]
pub fn update_gold_ui(
    mut gold_text_query: Query<&mut Text, With<GoldText>>,
    player_query: Query<&crate::components::player::Player>,
) {
    if let Some(player) = player_query.iter().next() {
        for mut text in &mut gold_text_query {
            **text = format!("Gold: {}", player.gold);
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

pub fn update_menu_gold_text(
    mut query: Query<&mut Text, With<MenuGoldText>>,
    player_query: Query<&crate::components::player::Player>,
) {
    if let Some(player) = player_query.iter().next() {
        for mut text in &mut query {
            text.0 = format!("Gold: {}", player.gold);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_cooldown_indicators(
    mut overlay_query: Query<(&mut Node, &CooldownOverlay)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    for (mut node, overlay) in &mut overlay_query {
        if let Some((_, weapon)) = hand_query.iter().find(|(h, _)| h.side == overlay.side) {
            // Calculate primary cooldown progress
            let primary_elapsed = now - weapon.last_shot;
            let primary_progress = if weapon.cooldown > 0.0 {
                (1.0 - (primary_elapsed / weapon.cooldown)).clamp(0.0, 1.0)
            } else {
                0.0
            };

            // Calculate skill cooldown progress
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
