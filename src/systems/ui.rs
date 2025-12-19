use crate::components::player::{Hand, HandType};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            WeaponMenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("WEAPON SELECTION"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    // Left Column
                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::right(Val::Px(50.0)),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn((
                            Text::new("LEFT HAND"),
                            TextFont::default(),
                            TextColor(Color::WHITE),
                        ));
                        spawn_weapon_button(col, HandType::Left, WeaponType::Shuriken, "Shuriken ❄");
                        spawn_weapon_button(col, HandType::Left, WeaponType::Sword, "Sword 🗡");
                        spawn_weapon_button(col, HandType::Left, WeaponType::Gun, "Gun 🔫");
                        spawn_weapon_button(col, HandType::Left, WeaponType::Magic, "Magic 🔮");
                    });

                    // Right Column
                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::left(Val::Px(50.0)),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn((
                            Text::new("RIGHT HAND"),
                            TextFont::default(),
                            TextColor(Color::WHITE),
                        ));
                        spawn_weapon_button(col, HandType::Right, WeaponType::Shuriken, "Shuriken ❄");
                        spawn_weapon_button(col, HandType::Right, WeaponType::Sword, "Sword 🗡");
                        spawn_weapon_button(col, HandType::Right, WeaponType::Gun, "Gun 🔫");
                        spawn_weapon_button(col, HandType::Right, WeaponType::Magic, "Magic 🔮");
                    });
                });

            // Magic Loadout in Menu
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_magic_panel(row, HandType::Left);
                    spawn_magic_panel(row, HandType::Right);
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
                .observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>| {
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

    // Shop Menu (Initially Hidden)
    commands
        .spawn((
            Node {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        ))
        .insert(ShopMenu)
        .with_children(|shop| {
            shop.spawn((
                Text::new("--- SHOP ---"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_shop_button(shop, ShopButton::Heal, "Heal (+30 HP)");
            spawn_shop_button(shop, ShopButton::DamageUp, "Damage Up (+10%)");
            spawn_shop_button(shop, ShopButton::NextRound, "Start Next Round");
        });
}

fn spawn_shop_button(parent: &mut ChildSpawnerCommands, btn_type: ShopButton, label: &str) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
            btn_type,
        ))
        .observe(
            |trigger: On<Pointer<Click>>,
             btn_query: Query<&ShopButton>,
             mut round_manager: ResMut<crate::resources::round::RoundManager>,
             mut player_query: Query<&mut crate::components::player::Player>,
             mut color_query: Query<&mut BackgroundColor>| {
                if let Ok(btn_type) = btn_query.get(trigger.entity) {
                    if let Ok(mut color) = color_query.get_mut(trigger.entity) {
                        *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0));
                    }
                    match btn_type {
                        ShopButton::Heal => {
                            if let Ok(mut player) = player_query.single_mut() {
                                player.health = (player.health + 30.0).min(100.0);
                                println!("Healed! Health: {}", player.health);
                            }
                        }
                        ShopButton::DamageUp => {
                            println!("Damage Upgraded! (Placeholder)");
                        }
                        ShopButton::NextRound => {
                            round_manager
                                .round_timer
                                .set_duration(std::time::Duration::from_secs(0));
                            round_manager.round_timer.reset();
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
                    *color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
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
                width: Val::Px(600.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
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
                        width: Val::Px(180.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
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
                        width: Val::Px(180.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
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
pub fn update_ui_visibility(
    mut shop_query: Query<&mut Node, (With<ShopMenu>, Without<WeaponMenuUI>)>,
    mut weapon_menu_query: Query<&mut Node, (With<WeaponMenuUI>, Without<ShopMenu>)>,
    game_state: Res<State<crate::resources::game_state::GameState>>,
    round_manager: Res<crate::resources::round::RoundManager>,
) -> Result<(), String> {
    // 1. Shop Visibility
    for mut node in &mut shop_query {
        node.display = if *game_state.get() == crate::resources::game_state::GameState::Shop
            || round_manager.round_state == crate::resources::round::RoundState::Shop
        {
            Display::Flex
        } else {
            Display::None
        };
    }

    // 2. Weapon Menu Visibility
    for mut node in &mut weapon_menu_query {
        node.display = if *game_state.get() == crate::resources::game_state::GameState::WeaponMenu {
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
