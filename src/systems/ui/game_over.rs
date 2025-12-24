use super::components::{GameOverUI, NewGameButton};

use bevy::prelude::*;

#[allow(clippy::too_many_lines)]
pub fn spawn_game_over_menu(mut commands: Commands) {
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
                .observe(|_trigger: On<Pointer<Click>>, mut commands: Commands| {
                    crate::systems::ui::menu::spawn_confirmation_dialog(&mut commands);
                })
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

pub fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
