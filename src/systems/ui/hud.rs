use super::components::{
    CooldownOverlay, GoldText, HUDHandIndicator, HUDIcon, HealthBar, HealthText,
    MagicSlotIndicator, MenuButton, RoundText, ShurikenCountText,
};
use crate::components::player::{CombatStats, Currency, Hand, HandType, Health, Player};
use crate::components::weapon::{MagicLoadout, SpellType, WeaponType};
use bevy::prelude::*;

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn spawn_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
                        HUDIcon {
                            side: HandType::Left,
                        },
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
                .observe(
                    |_: On<Pointer<Click>>,
                     mut next_state: ResMut<NextState<crate::resources::game_state::GameState>>| {
                        next_state.set(crate::resources::game_state::GameState::WeaponMenu);
                    },
                )
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
                        HUDIcon {
                            side: HandType::Right,
                        },
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
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_hud_indicators(
    mut icon_query: Query<(&HUDIcon, &mut ImageNode)>,
    hand_query: Query<(
        &Hand,
        &crate::components::weapon::SwordState,
        &crate::components::weapon::GunState,
        &MagicLoadout,
    )>,
    asset_server: Res<AssetServer>,
) {
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
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_health_ui(
    mut health_bar_query: Query<&mut Node, (With<HealthBar>, Without<HealthText>)>,
    mut health_text_query: Query<&mut Text, (With<HealthText>, Without<HealthBar>)>,
    player: Single<&Health, With<Player>>,
) {
    let health = *player;
    for mut node in &mut health_bar_query {
        // Player health is 0..max_health
        let percent = (health.current / health.max).clamp(0.0, 1.0) * 100.0;
        node.width = Val::Percent(percent);
    }

    for mut text in &mut health_text_query {
        text.0 = format!("{:.0} / {:.0}", health.current, health.max);
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_gold_ui(
    mut gold_text_query: Query<&mut Text, With<GoldText>>,
    player: Single<&Currency, With<Player>>,
) {
    for mut text in &mut gold_text_query {
        text.0 = format!("Gold: {}", player.gold);
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

#[allow(clippy::needless_pass_by_value)]
pub fn update_hud_magic_ui(
    mut query: Query<(&mut ImageNode, &mut BackgroundColor, &MagicSlotIndicator)>,
    player: Single<&MagicLoadout, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let magic = *player;
    for (mut image, mut bg, slot) in &mut query {
        // Highlight active slot
        if magic.active_slot == slot.slot {
            bg.0 = Color::srgba(1.0, 1.0, 1.0, 0.4);
        } else {
            bg.0 = Color::srgba(0.0, 0.0, 0.0, 0.4);
        }

        let spell = match slot.slot {
            crate::components::weapon::ActiveSpellSlot::Primary => magic.primary,
            crate::components::weapon::ActiveSpellSlot::Secondary => magic.secondary,
        };

        // Set icon
        let icon_path = match spell {
            SpellType::EnergyBolt => "ui/icons/magic_bolt.png",
            SpellType::Laser => "ui/icons/magic_laser.png",
            SpellType::Nova => "ui/icons/magic_nova.png",
            SpellType::Blink => "ui/icons/magic_blink.png",
            SpellType::Global => "ui/icons/magic_global.png",
        };
        image.image = asset_server.load(icon_path);
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_cooldown_indicators(
    mut overlay_query: Query<(&mut Node, &CooldownOverlay)>,
    hand_query: Query<(&Hand, &crate::components::weapon::Weapon)>,
    player: Single<&CombatStats, With<Player>>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    let combat_stats = *player;

    for (mut node, overlay) in &mut overlay_query {
        if let Some((hand, weapon)) = hand_query.iter().find(|(h, _)| h.side == overlay.side) {
            // Apply CDR only to Magic weapons
            let effective_cooldown = if hand.equipped_weapon == Some(WeaponType::Magic) {
                weapon.cooldown * (1.0 - combat_stats.cooldown_reduction)
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
