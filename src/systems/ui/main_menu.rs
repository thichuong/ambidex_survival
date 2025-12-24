use super::components::MainMenuUI;
use crate::resources::game_state::GameState;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands) {
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
            BackgroundColor(Color::srgba(0.04, 0.04, 0.06, 1.0)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            spawn_title(parent);
            spawn_start_button(parent);
            spawn_settings_button(parent);
            spawn_tutorial_button(parent);
        });
}

fn spawn_title(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new("AMBIDEX SURVIVAL"),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 1.0)), // Aqua
        Node {
            margin: UiRect::bottom(Val::Px(60.0)),
            ..default()
        },
    ));
}

fn spawn_start_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.3, 0.8, 0.3)),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0)),
        ))
        .observe(
            |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing);
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.25, 0.45, 0.25, 1.0));
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new("START GAME"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_settings_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
        ))
        .observe(
            |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Settings);
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0));
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
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_tutorial_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.2, 0.6, 1.0)),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgba(0.15, 0.2, 0.3, 1.0)),
        ))
        .observe(
            |_: On<Pointer<Click>>,
             mut next_state: ResMut<NextState<GameState>>,
             mut prev_state: ResMut<crate::resources::game_state::PreviousMenuState>| {
                prev_state.0 = GameState::MainMenu;
                next_state.set(GameState::Tutorial);
            },
        )
        .observe(
            |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.25, 0.4, 0.6, 1.0));
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.15, 0.2, 0.3, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new("TUTORIAL"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
