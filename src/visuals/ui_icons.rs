//! UI Icon spawning functions
//! Contains functions to spawn UI nodes representing various icons (sword, shield, cross, etc.)
//! These are procedurally drawn using Bevy UI Nodes.

use crate::systems::ui::components::ShopButton;
use bevy::prelude::*;

/// Dispatch function to spawn the appropriate icon based on the button type
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
            ShopButton::NovaCore => spawn_nova_core_icon(icon),
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

fn spawn_nova_core_icon(parent: &mut ChildSpawnerCommands) {
    // Glowing orb
    parent.spawn((
        Node {
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.3, 0.8, 1.0, 0.8)),
        BorderRadius::all(Val::Px(15.0)),
    ));
    // Core
    parent.spawn((
        Node {
            width: Val::Px(12.0),
            height: Val::Px(12.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::WHITE),
        BorderRadius::all(Val::Px(6.0)),
    ));
    // Ring
    parent.spawn((
        Node {
            width: Val::Px(42.0),
            height: Val::Px(42.0),
            position_type: PositionType::Absolute,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BorderColor::all(Color::srgba(0.9, 0.5, 1.0, 1.0)),
        BorderRadius::all(Val::Px(21.0)),
    ));
}

/// Draws an infinity symbol (∞) using two overlapping circles
pub fn spawn_infinity_symbol(parent: &mut ChildSpawnerCommands) {
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
