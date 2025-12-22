use super::super::components::{
    InfinitySymbol, PurchaseEvent, ShopButton, ShopCardCount, ShopCardCurrentCount, ShopCardLimit,
};
use crate::components::player::Progression;
use bevy::prelude::*;

#[allow(clippy::too_many_lines)]
pub fn spawn_shop_button(parent: &mut ChildSpawnerCommands, btn_type: ShopButton, _label: &str) {
    // Determine card type: BLUE (Advanced) or WHITE (Basic)
    let (border_color, bg_color, bg_hover, text_accent) = get_shop_button_colors(btn_type);

    // Get title, description, and price for each upgrade type
    let (title, desc, price) = get_shop_button_content(btn_type);

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
            BorderRadius::all(Val::Px(16.0)),
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
            // Rust-Drawn Icon
            spawn_shop_icon(card, btn_type);

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
                    Text::new(price.to_string()),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.85, 0.0)), // Gold color
                ));
            });
            // Shop Card Count Container - holds "[count / limit]" or "[count / ∞]"
            card.spawn((
                Node {
                    margin: UiRect::top(Val::Px(4.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(2.0),
                    ..default()
                },
                ShopCardCount,
            ))
            .with_children(|count_container| {
                // "[" bracket
                count_container.spawn((
                    Text::new("["),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
                // Count number (will be updated dynamically)
                count_container.spawn((
                    Text::new("0"),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ShopCardCurrentCount,
                ));
                // "/" separator
                count_container.spawn((
                    Text::new(" / "),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
                // Limit text (shown when has limit)
                count_container.spawn((
                    Text::new("0"),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    super::super::components::ShopCardLimit,
                ));
                // Infinity symbol container (shown when no limit)
                count_container
                    .spawn((
                        Node {
                            width: Val::Px(14.0),
                            height: Val::Px(8.0),
                            position_type: PositionType::Relative,
                            display: Display::None, // Hidden by default
                            ..default()
                        },
                        super::super::components::InfinitySymbol,
                    ))
                    .with_children(|infinity| {
                        spawn_infinity_symbol(infinity);
                    });
                // "]" bracket
                count_container.spawn((
                    Text::new("]"),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            });
        });
}

const fn get_shop_button_colors(btn_type: ShopButton) -> (Color, Color, Color, Color) {
    match btn_type {
        ShopButton::Heal => (
            Color::srgb(0.9, 0.9, 0.95),         // Soft white border
            Color::srgba(0.15, 0.15, 0.2, 0.95), // Dark gray background
            Color::srgba(0.25, 0.25, 0.3, 1.0),  // Hover gray
            Color::srgb(1.0, 1.0, 1.0),          // White accent text
        ),
        ShopButton::DamageUp => (
            Color::srgb(1.0, 0.3, 0.3),          // Vibrant red border
            Color::srgba(0.2, 0.05, 0.05, 0.95), // Dark red background
            Color::srgba(0.3, 0.1, 0.1, 1.0),    // Hover red
            Color::srgb(1.0, 0.6, 0.6),          // Red accent text
        ),
        ShopButton::MaxHealthUp => (
            Color::srgb(0.2, 1.0, 0.4),         // Bright emerald border
            Color::srgba(0.05, 0.2, 0.1, 0.95), // Dark green background
            Color::srgba(0.1, 0.3, 0.15, 1.0),  // Hover green
            Color::srgb(0.5, 1.0, 0.6),         // Green accent text
        ),
        ShopButton::CritDamageUp => (
            Color::srgb(1.0, 0.8, 0.0),         // Gold border
            Color::srgba(0.2, 0.15, 0.0, 0.95), // Dark amber background
            Color::srgba(0.3, 0.25, 0.0, 1.0),  // Hover amber
            Color::srgb(1.0, 0.9, 0.4),         // Amber accent text
        ),
        ShopButton::CritChanceUp => (
            Color::srgb(0.8, 0.4, 1.0),          // Violet border
            Color::srgba(0.15, 0.05, 0.2, 0.95), // Dark purple background
            Color::srgba(0.25, 0.1, 0.3, 1.0),   // Hover purple
            Color::srgb(0.9, 0.7, 1.0),          // Purple accent text
        ),
        ShopButton::LifestealUp => (
            Color::srgb(1.0, 0.2, 0.5),         // Crimson border
            Color::srgba(0.2, 0.0, 0.1, 0.95),  // Deep red background
            Color::srgba(0.3, 0.05, 0.15, 1.0), // Hover crimson
            Color::srgb(1.0, 0.5, 0.7),         // Crimson accent text
        ),
        ShopButton::CooldownReductionUp => (
            Color::srgb(0.0, 0.8, 1.0),         // Electric cyan border
            Color::srgba(0.0, 0.15, 0.2, 0.95), // Dark cyan background
            Color::srgba(0.05, 0.25, 0.3, 1.0), // Hover cyan
            Color::srgb(0.4, 0.9, 1.0),         // Cyan accent text
        ),
    }
}

const fn get_shop_button_content(
    btn_type: ShopButton,
) -> (&'static str, &'static str, &'static str) {
    match btn_type {
        ShopButton::Heal => ("Heal", "+30 HP", "50G"),
        ShopButton::DamageUp => ("Damage", "+10%", "100G"),
        ShopButton::MaxHealthUp => ("Max HP", "+20", "150G"),
        ShopButton::CritDamageUp => ("Crit Dmg", "+50%", "200G"),
        ShopButton::CritChanceUp => ("Crit Rate", "+10%", "250G"),
        ShopButton::LifestealUp => ("Lifesteal", "+10%", "300G"),
        ShopButton::CooldownReductionUp => ("CDR", "+10%", "350G"),
    }
}

pub fn spawn_shop_icon(parent: &mut ChildSpawnerCommands, btn_type: ShopButton) {
    let container_node = Node {
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        margin: UiRect::bottom(Val::Px(8.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Relative,
        ..default()
    };

    parent
        .spawn(container_node)
        .with_children(|icon| match btn_type {
            ShopButton::Heal => spawn_heal_icon(icon),
            ShopButton::DamageUp => spawn_damage_icon(icon),
            ShopButton::MaxHealthUp => spawn_max_health_icon(icon),
            ShopButton::CritDamageUp => spawn_crit_damage_icon(icon),
            ShopButton::CritChanceUp => spawn_crit_chance_icon(icon),
            ShopButton::LifestealUp => spawn_lifesteal_icon(icon),
            ShopButton::CooldownReductionUp => spawn_cdr_icon(icon),
        });
}

fn spawn_heal_icon(parent: &mut ChildSpawnerCommands) {
    // Red Medical Cross
    let cross_color = BackgroundColor(Color::srgb(0.9, 0.2, 0.2));
    // Vertical bar
    parent.spawn((
        Node {
            width: Val::Px(14.0),
            height: Val::Px(40.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        cross_color,
        BorderRadius::all(Val::Px(4.0)),
    ));
    // Horizontal bar
    parent.spawn((
        Node {
            width: Val::Px(40.0),
            height: Val::Px(14.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        cross_color,
        BorderRadius::all(Val::Px(4.0)),
    ));
}

fn spawn_damage_icon(parent: &mut ChildSpawnerCommands) {
    // Sword
    // Blade
    parent.spawn((
        Node {
            width: Val::Px(8.0),
            height: Val::Px(50.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgb(0.8, 0.8, 0.9)), // Silver blade
        BorderRadius::top(Val::Px(4.0)),
        Transform::from_rotation(Quat::from_rotation_z(45.0f32.to_radians())),
    ));
    // Hilt/Guard
    parent.spawn((
        Node {
            width: Val::Px(24.0),
            height: Val::Px(4.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgb(0.8, 0.6, 0.2)), // Gold Guard
        Transform::from_xyz(-10.0, -10.0, 0.1)
            .with_rotation(Quat::from_rotation_z(45.0f32.to_radians())),
    ));
    // Handle
    parent.spawn((
        Node {
            width: Val::Px(6.0),
            height: Val::Px(12.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgb(0.4, 0.2, 0.1)), // Brown Handle
        BorderRadius::bottom(Val::Px(3.0)),
        Transform::from_xyz(-13.0, -13.0, 0.0)
            .with_rotation(Quat::from_rotation_z(45.0f32.to_radians())),
    ));
}

fn spawn_max_health_icon(parent: &mut ChildSpawnerCommands) {
    // Shield
    parent
        .spawn((
            Node {
                width: Val::Px(36.0),
                height: Val::Px(42.0),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BorderRadius::bottom(Val::Px(18.0)), // Rounded bottom for shield shape
            BackgroundColor(Color::srgb(0.2, 0.6, 0.3)), // Greenish shield
            BorderColor::all(Color::srgb(0.9, 0.9, 0.9)), // Silver border
        ))
        .with_children(|shield| {
            // Cross symbol on shield
            shield.spawn((
                Node {
                    width: Val::Px(12.0),
                    height: Val::Px(12.0),
                    margin: UiRect::all(Val::Auto), // Center it
                    ..default()
                },
                Text::new("+"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

#[allow(clippy::cast_precision_loss)]
fn spawn_crit_damage_icon(parent: &mut ChildSpawnerCommands) {
    // Impact / Explosion
    for i in 0..8 {
        let angle = (i as f32 * 45.0).to_radians();
        parent.spawn((
            Node {
                width: Val::Px(4.0),
                height: Val::Px(16.0), // Spikes
                position_type: PositionType::Absolute,
                top: Val::Px(2.0),
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.6, 0.0)), // Orange
            Transform::from_rotation(Quat::from_rotation_z(angle)).with_translation(Vec3::new(
                angle.sin() * 12.0,
                angle.cos() * 12.0,
                0.0,
            )),
        ));
    }
    // Core
    parent.spawn((
        Node {
            width: Val::Px(16.0),
            height: Val::Px(16.0),
            ..default()
        },
        BorderRadius::all(Val::Px(8.0)),
        BackgroundColor(Color::srgb(1.0, 0.9, 0.2)), // Yellow core
    ));
}

fn spawn_crit_chance_icon(parent: &mut ChildSpawnerCommands) {
    // Target Reticle
    parent.spawn((
        Node {
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            border: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BorderRadius::all(Val::Px(20.0)),
        BorderColor::all(Color::srgb(1.0, 0.2, 0.2)), // Red ring
        BackgroundColor(Color::NONE),
    ));
    // Crosshairs
    parent.spawn((
        Node {
            width: Val::Px(2.0),
            height: Val::Px(44.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.2, 0.2)),
    ));
    parent.spawn((
        Node {
            width: Val::Px(44.0),
            height: Val::Px(2.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.2, 0.2)),
    ));
}

fn spawn_lifesteal_icon(parent: &mut ChildSpawnerCommands) {
    // Blood Drop
    // A rotated square with rounded corners
    parent.spawn((
        Node {
            width: Val::Px(28.0),
            height: Val::Px(28.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.8, 0.0, 0.0)),
        BorderRadius::top_left(Val::Px(14.0))
            .with_bottom_left(Val::Px(14.0))
            .with_bottom_right(Val::Px(14.0))
            .with_top_right(Val::Px(0.0)), // Pointy top-right
        Transform::from_rotation(Quat::from_rotation_z(45.0f32.to_radians())),
    ));
}

fn spawn_cdr_icon(parent: &mut ChildSpawnerCommands) {
    // Stopwatch
    parent
        .spawn((
            Node {
                width: Val::Px(34.0),
                height: Val::Px(34.0),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(17.0)),
            BorderColor::all(Color::srgb(0.3, 0.8, 1.0)), // Cyan rim
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|clock| {
            // Hands
            clock.spawn((
                Node {
                    width: Val::Px(2.0),
                    height: Val::Px(12.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(14.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE), // Hour hand
            ));
            clock.spawn((
                Node {
                    width: Val::Px(10.0),
                    height: Val::Px(2.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(14.0),
                    left: Val::Px(14.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE), // Minute hand
            ));
        });

    // Top button
    parent.spawn((
        Node {
            width: Val::Px(6.0),
            height: Val::Px(4.0),
            position_type: PositionType::Absolute,
            top: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.8, 1.0)),
    ));
}

/// Draws an infinity symbol (∞) using two overlapping circles
fn spawn_infinity_symbol(parent: &mut ChildSpawnerCommands) {
    let circle_size = Val::Px(7.0);
    let border_width = Val::Px(1.5);
    let border_color = BorderColor::all(Color::srgb(0.6, 0.6, 0.6));

    // Left circle
    parent.spawn((
        Node {
            width: circle_size,
            height: circle_size,
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            border: UiRect::all(border_width),
            ..default()
        },
        border_color,
        BorderRadius::all(Val::Px(4.0)),
        BackgroundColor(Color::NONE),
    ));

    // Right circle
    parent.spawn((
        Node {
            width: circle_size,
            height: circle_size,
            position_type: PositionType::Absolute,
            left: Val::Px(6.0), // Overlap with left circle
            top: Val::Px(0.0),
            border: UiRect::all(border_width),
            ..default()
        },
        border_color,
        BorderRadius::all(Val::Px(4.0)),
        BackgroundColor(Color::NONE),
    ));
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn update_shop_cards_ui(
    progression: Single<&Progression, With<crate::components::player::Player>>,
    mut card_query: Query<(&ShopButton, &mut BackgroundColor, &Children)>,
    count_container_query: Query<&Children, With<ShopCardCount>>,
    mut current_count_query: Query<&mut Text, With<ShopCardCurrentCount>>,
    mut limit_text_query: Query<&mut Text, (With<ShopCardLimit>, Without<ShopCardCurrentCount>)>,
    mut infinity_query: Query<&mut Node, With<InfinitySymbol>>,
) {
    for (btn_type, mut bg_color, card_children) in &mut card_query {
        let config = crate::configs::shop::get_card_config(*btn_type);
        let count = match btn_type {
            ShopButton::Heal => progression.heal_count,
            ShopButton::DamageUp => progression.damage_upgrades,
            ShopButton::MaxHealthUp => progression.max_health_upgrades,
            ShopButton::CritDamageUp => progression.crit_damage_upgrades,
            ShopButton::CritChanceUp => progression.crit_chance_upgrades,
            ShopButton::LifestealUp => progression.lifesteal_upgrades,
            ShopButton::CooldownReductionUp => progression.cdr_upgrades,
        };

        // Find the ShopCardCount container among card's children
        for &child in card_children {
            if let Ok(count_children) = count_container_query.get(child) {
                for &count_child in count_children {
                    // Update current count
                    if let Ok(mut text) = current_count_query.get_mut(count_child) {
                        text.0 = count.to_string();
                    }

                    // Update limit or infinity
                    if let Ok(mut text) = limit_text_query.get_mut(count_child) {
                        if let Some(limit) = config.limit {
                            text.0 = limit.to_string();
                        } else {
                            text.0 = String::new();
                        }
                    }

                    // Toggle infinity symbol visibility
                    if let Ok(mut node) = infinity_query.get_mut(count_child) {
                        node.display = if config.limit.is_some() {
                            Display::None
                        } else {
                            Display::Flex
                        };
                    }
                }
            }
        }

        // Dim if maxed
        if let Some(limit) = config.limit
            && count >= limit
        {
            bg_color.0 = bg_color.0.with_alpha(0.3);
        }
    }
}
