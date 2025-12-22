use super::components::TutorialUI;
use crate::resources::game_state::GameState;

use bevy::prelude::*;

#[allow(clippy::too_many_lines)]
pub fn spawn_tutorial_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.95)),
            TutorialUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("TUTORIAL"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.8, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Instructions Box
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::all(Val::Px(30.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                    BorderColor::all(Color::srgba(0.5, 0.5, 0.5, 0.5)),
                ))
                .with_children(|box_node| {
                    spawn_tutorial_line(box_node, "MOVEMENT", "W, A, S, D");
                    spawn_tutorial_line(box_node, "LEFT HAND ATTACK", "Mouse Left");
                    spawn_tutorial_line(box_node, "RIGHT HAND ATTACK", "Mouse Right");
                    spawn_tutorial_line(
                        box_node,
                        "LEFT HAND SKILL",
                        "Q Key (Weapon Specific Ability)",
                    );
                    spawn_tutorial_line(
                        box_node,
                        "RIGHT HAND SKILL",
                        "E Key (Weapon Specific Ability)",
                    );
                    spawn_tutorial_line(box_node, "OPEN MENU", "ESC / Menu Button");

                    box_node.spawn(Node {
                        height: Val::Px(20.0),
                        ..default()
                    });

                    spawn_tutorial_line(
                        box_node,
                        "ECONOMY",
                        "Kill enemies for 10G. Spend in Shop between rounds.",
                    );
                    spawn_tutorial_line(
                        box_node,
                        "AMBIDEX",
                        "Customize each hand with unique weapons & spells!",
                    );
                });

            // Back Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        margin: UiRect::top(Val::Px(40.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                ))
                .observe(
                    |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::WeaponMenu);
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
                        Text::new("BACK"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn spawn_tutorial_line(parent: &mut ChildSpawnerCommands, label: &str, value: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect::vertical(Val::Px(5.0)),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(format!("{label}: ")),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
            row.spawn((
                Text::new(value),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn despawn_tutorial_ui(mut commands: Commands, query: Query<Entity, With<TutorialUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
