use crate::components::player::{Hand, HandType};
use crate::components::weapon::WeaponType;
use bevy::prelude::*;

#[derive(Component)]
pub struct WeaponButton {
    pub hand_type: HandType,
    pub weapon_type: WeaponType,
}

pub fn setup_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Root UI Node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd, // Align to bottom
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Weapon Select Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                    ..default()
                })
                .with_children(|panel| {
                    // Text Label
                    panel.spawn(TextBundle::from_section(
                        "Left Hand (Q) [Click to Set]:",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Buttons for Left Hand
                    spawn_weapon_button(panel, HandType::Left, WeaponType::Shuriken, "Shuriken");
                    spawn_weapon_button(panel, HandType::Left, WeaponType::Sword, "Sword");
                    spawn_weapon_button(panel, HandType::Left, WeaponType::Bow, "Bow");

                    // Spacer
                    panel.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    });

                    // Text Label
                    panel.spawn(TextBundle::from_section(
                        "Right Hand (E) [Click to Set]:",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Buttons for Right Hand
                    spawn_weapon_button(panel, HandType::Right, WeaponType::Shuriken, "Shuriken");
                    spawn_weapon_button(panel, HandType::Right, WeaponType::Sword, "Sword");
                    spawn_weapon_button(panel, HandType::Right, WeaponType::Bow, "Bow");
                });
        });
}

fn spawn_weapon_button(parent: &mut ChildBuilder, hand: HandType, weapon: WeaponType, label: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
            ..default()
        })
        .insert(WeaponButton {
            hand_type: hand,
            weapon_type: weapon,
        })
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 14.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

pub fn weapon_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &WeaponButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut hand_query: Query<&mut Hand>,
) {
    for (interaction, mut color, button_data) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.2, 0.8, 0.2, 1.0)); // Green clicked

                // Update Player Hand
                for mut hand in hand_query.iter_mut() {
                    if hand.hand_type == button_data.hand_type {
                        hand.equipped_weapon = Some(button_data.weapon_type);
                        println!(
                            "Equipped {:?} to {:?}",
                            button_data.weapon_type, button_data.hand_type
                        );
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 1.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0));
            }
        }
    }
}
