use super::super::components::{
    InfinitySymbol, SelectCardEvent, ShopButton, ShopCardCount, ShopCardCurrentCount, ShopCardLimit,
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
                width: Val::Px(160.0),
                min_width: Val::Px(140.0),
                height: Val::Px(200.0),
                min_height: Val::Px(180.0),
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
            |trigger: On<Pointer<Click>>, btn_query: Query<&ShopButton>, mut commands: Commands| {
                if let Ok(btn_type) = btn_query.get(trigger.entity) {
                    commands.trigger(SelectCardEvent {
                        btn_type: *btn_type,
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
            crate::visuals::ui_icons::spawn_shop_icon(card, btn_type);

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
                    Text::new(price.clone()),
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
                        crate::visuals::ui_icons::spawn_infinity_symbol(infinity);
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

pub const fn get_shop_button_colors(btn_type: ShopButton) -> (Color, Color, Color, Color) {
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
        ShopButton::NovaCore => (
            Color::srgb(0.9, 0.4, 1.0),          // Magenta border
            Color::srgba(0.15, 0.05, 0.2, 0.95), // Dark purple background
            Color::srgba(0.25, 0.1, 0.3, 1.0),   // Hover purple
            Color::srgb(0.9, 0.7, 1.0),          // Light purple accent
        ),
    }
}

pub fn get_shop_button_content(btn_type: ShopButton) -> (String, String, String) {
    let config = crate::configs::shop::get_card_config(btn_type);
    let title = config.name.to_string();
    let price = format!("{}G", config.price);

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let desc = match btn_type {
        ShopButton::Heal => format!("+{} HP", config.value),
        ShopButton::DamageUp | ShopButton::CritChanceUp | ShopButton::CooldownReductionUp => {
            format!("+{}%", (config.value * 100.0) as u32)
        }
        ShopButton::MaxHealthUp => format!("+{}", config.value as u32),
        ShopButton::CritDamageUp => format!("+{}%", (config.value * 100.0) as u32),
        ShopButton::LifestealUp => {
            format!(
                "+{}%|AOE:+{}%",
                (config.value * 100.0) as u32,
                (config.value * 50.0) as u32
            )
        }
        ShopButton::NovaCore => config.description.to_string(),
    };

    (title, desc, price)
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
            ShopButton::NovaCore => progression.nova_core,
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
