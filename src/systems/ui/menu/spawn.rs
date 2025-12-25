use super::arsenal::spawn_equipment_panel;
use super::components::WeaponMenuUI;
use super::layout::{spawn_footer, spawn_header, spawn_sidebar};
use super::shop::spawn_shop_panel;
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
                flex_direction: FlexDirection::Row, // Root is Row now
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.04, 0.06, 1.0)),
            WeaponMenuUI,
        ))
        .with_children(|root| {
            // === SIDEBAR (Left) ===
            spawn_sidebar(root);

            // === MAIN CONTENT COLUMN (Right) ===
            root.spawn(Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|main_col| {
                // --- HEADER ---
                spawn_header(main_col);

                // --- CONTENT AREA ---
                main_col
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::all(Val::Px(20.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    })
                    .with_children(|content| {
                        // === SHOP CONTAINER (Card Tab) ===
                        spawn_shop_panel(content);

                        // === EQUIPMENT CONTAINER (Equip Tab) ===
                        spawn_equipment_panel(content, &asset_server);
                    });

                // --- FOOTER ---
                spawn_footer(main_col);
            });
        });
}

pub fn despawn_weapon_menu(mut commands: Commands, query: Query<Entity, With<WeaponMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
