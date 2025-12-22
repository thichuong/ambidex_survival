use super::super::components::{PurchaseEvent, ShopButton};
use bevy::prelude::*;

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

fn get_shop_button_colors(btn_type: ShopButton) -> (Color, Color, Color, Color) {
    let is_blue = matches!(
        btn_type,
        ShopButton::CritChanceUp | ShopButton::LifestealUp | ShopButton::CooldownReductionUp
    );

    if is_blue {
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
    }
}

fn get_shop_button_content(btn_type: ShopButton) -> (&'static str, &'static str, &'static str) {
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

#[allow(clippy::too_many_lines)]
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
            ShopButton::Heal => {
                let cross_color = BackgroundColor(Color::srgb(1.0, 0.2, 0.2));
                icon.spawn((
                    Node {
                        width: Val::Px(12.0),
                        height: Val::Px(36.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    cross_color,
                ));
                icon.spawn((
                    Node {
                        width: Val::Px(36.0),
                        height: Val::Px(12.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    cross_color,
                ));
            }
            ShopButton::DamageUp => {
                icon.spawn((
                    Node {
                        width: Val::Px(8.0),
                        height: Val::Px(35.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.1, 0.1)),
                ));
                icon.spawn((
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(6.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(15.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.6, 0.0)),
                ));
            }
            ShopButton::MaxHealthUp => {
                icon.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(15.0)),
                    BorderColor::all(Color::srgb(1.0, 0.4, 0.6)),
                ));
            }
            ShopButton::CritDamageUp => {
                for i in 0..4 {
                    #[allow(clippy::cast_precision_loss)]
                    let angle = (i as f32 * 45.0).to_radians();
                    icon.spawn((
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(6.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Transform::from_rotation(Quat::from_rotation_z(angle)),
                        BackgroundColor(Color::srgb(1.0, 0.8, 0.0)),
                    ));
                }
            }
            ShopButton::CritChanceUp => {
                icon.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(15.0)),
                    BorderColor::all(Color::srgb(0.4, 0.9, 1.0)),
                ));
                icon.spawn((
                    Node {
                        width: Val::Px(2.0),
                        height: Val::Px(40.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ));
                icon.spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(2.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ));
            }
            ShopButton::LifestealUp => {
                icon.spawn((
                    Node {
                        width: Val::Px(25.0),
                        height: Val::Px(35.0),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(12.0)),
                    BackgroundColor(Color::srgb(0.7, 0.1, 0.1)),
                ));
            }
            ShopButton::CooldownReductionUp => {
                icon.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(15.0)),
                    BorderColor::all(Color::srgb(0.2, 0.7, 1.0)),
                ));
                icon.spawn((
                    Node {
                        width: Val::Px(2.0),
                        height: Val::Px(12.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(25.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ));
            }
        });
}
