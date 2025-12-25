use super::components::{
    ActiveTab, MenuCDRText, MenuCritText, MenuDamageText, MenuGoldText, MenuHealthText,
    MenuLifestealText, TabButton, WeaponMenuRestartButton, WeaponMenuSettingsButton, WeaponMenuTab,
};
use super::interaction::{handle_tab_hover, handle_tab_interaction, handle_tab_out};
use crate::systems::ui::TutorialButton;
use bevy::prelude::*;

pub fn spawn_sidebar(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::vertical(Val::Px(20.0)),
                row_gap: Val::Px(20.0),
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.06, 0.08, 1.0)),
            BorderColor::all(Color::srgba(0.2, 0.2, 0.25, 0.5)),
        ))
        .with_children(|sidebar| {
            // Title / Logo Area in Sidebar
            sidebar.spawn((
                Text::new("MENU"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Card Tab Button
            sidebar
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Px(20.0)),
                        border: UiRect::left(Val::Px(5.0)),
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.15, 0.25, 0.35, 1.0)),
                    TabButton {
                        tab: WeaponMenuTab::Card,
                    },
                    ActiveTab,
                    BorderColor::all(Color::srgb(1.0, 0.8, 0.2)), // Active Gold Border
                    BorderRadius::all(Val::Px(5.0)),
                ))
                .observe(handle_tab_interaction)
                .observe(handle_tab_hover)
                .observe(handle_tab_out)
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("UPGRADES"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Equip Tab Button
            sidebar
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Px(20.0)),
                        border: UiRect::left(Val::Px(5.0)),
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 1.0)),
                    TabButton {
                        tab: WeaponMenuTab::Equip,
                    },
                    BorderColor::all(Color::NONE),
                    BorderRadius::all(Val::Px(5.0)),
                ))
                .observe(handle_tab_interaction)
                .observe(handle_tab_hover)
                .observe(handle_tab_out)
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("EQUIPMENT"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
        });
}

pub fn spawn_header(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.1, 0.95)),
            BorderColor::all(Color::srgba(0.2, 0.2, 0.25, 0.5)),
        ))
        .with_children(|header| {
            header
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|status_bar| {
                    status_bar.spawn((
                        Text::new("HP: 100/100"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.2, 1.0, 0.4)),
                        MenuHealthText,
                    ));
                    status_bar.spawn((
                        Text::new("Gold: 0"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.8, 0.0)),
                        MenuGoldText,
                    ));
                    let stat_font = TextFont {
                        font_size: 16.0,
                        ..default()
                    };
                    status_bar.spawn((
                        Text::new("Dmg: +0%"),
                        stat_font.clone(),
                        TextColor(Color::srgb(1.0, 0.4, 0.1)),
                        MenuDamageText,
                    ));
                    status_bar.spawn((
                        Text::new("Crit: 0%"),
                        stat_font.clone(),
                        TextColor(Color::srgb(1.0, 0.2, 0.2)),
                        MenuCritText,
                    ));
                    status_bar.spawn((
                        Text::new("Life: 0%"),
                        stat_font.clone(),
                        TextColor(Color::srgb(1.0, 0.2, 1.0)),
                        MenuLifestealText,
                    ));
                    status_bar.spawn((
                        Text::new("CDR: 0%"),
                        stat_font,
                        TextColor(Color::srgb(0.2, 0.8, 1.0)),
                        MenuCDRText,
                    ));
                });
        });
}

#[allow(clippy::too_many_lines)]
pub fn spawn_footer(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                border: UiRect::top(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.1, 0.95)),
            BorderColor::all(Color::srgba(0.2, 0.2, 0.25, 0.5)),
        ))
        .with_children(|footer| {
            spawn_battle_button(footer);
            spawn_tutorial_button(footer);
            spawn_settings_button(footer);
            spawn_new_game_button(footer);
        });
}

fn spawn_battle_button(parent: &mut ChildSpawnerCommands) {
    use crate::resources::game_state::GameState;
    use crate::resources::round::{RoundManager, RoundState};

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.3, 0.8, 0.3)),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0)),
        ))
        .observe(
            |_: On<Pointer<Click>>,
             mut next_state: ResMut<NextState<GameState>>,
             mut round_manager: ResMut<RoundManager>| {
                if round_manager.round_state == RoundState::Shop {
                    round_manager.current_round += 1;
                    round_manager.enemies_to_spawn = crate::configs::enemy::BASE_ENEMY_COUNT
                        + (round_manager.current_round
                            * crate::configs::enemy::ENEMY_COUNT_SCALING_PER_ROUND);
                    #[allow(clippy::cast_precision_loss)]
                    let exponent = round_manager.current_round as f32;
                    round_manager.spawn_timer = bevy::time::Timer::from_seconds(
                        crate::configs::enemy::BASE_SPAWN_INTERVAL
                            * (crate::configs::enemy::SPAWN_INTERVAL_DECAY).powf(exponent),
                        bevy::time::TimerMode::Repeating,
                    );
                    round_manager.elites_to_spawn = round_manager.current_round;
                    round_manager.yellow_enemies_to_spawn =
                        u32::from(round_manager.current_round >= 5);
                    round_manager.round_state = RoundState::Spawning;
                }
                round_manager.has_started = true;
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
                Text::new("GO TO BATTLE"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_tutorial_button(parent: &mut ChildSpawnerCommands) {
    use crate::resources::game_state::GameState;

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.2, 0.6, 1.0)),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgba(0.15, 0.2, 0.3, 1.0)),
            TutorialButton,
        ))
        .observe(
            |_: On<Pointer<Click>>,
             mut next_state: ResMut<NextState<GameState>>,
             mut prev_state: ResMut<crate::resources::game_state::PreviousMenuState>| {
                prev_state.0 = GameState::WeaponMenu;
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
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_settings_button(parent: &mut ChildSpawnerCommands) {
    use crate::resources::game_state::GameState;

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.6, 0.6, 0.6)),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
            WeaponMenuSettingsButton,
        ))
        .observe(
            |_: On<Pointer<Click>>,
             mut next_state: ResMut<NextState<GameState>>,
             mut prev_state: ResMut<crate::resources::game_state::PreviousMenuState>| {
                prev_state.0 = GameState::WeaponMenu;
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
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_new_game_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(1.0, 0.3, 0.3)),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(Color::srgba(0.3, 0.15, 0.15, 1.0)),
            WeaponMenuRestartButton,
        ))
        .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
            crate::systems::ui::menu::confirmation::spawn_confirmation_dialog(&mut commands);
        })
        .observe(
            |trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.45, 0.25, 0.25, 1.0));
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                if let Ok(mut color) = color.get_mut(trigger.entity) {
                    *color = BackgroundColor(Color::srgba(0.3, 0.15, 0.15, 1.0));
                }
            },
        )
        .with_children(|btn| {
            btn.spawn((
                Text::new("NEW GAME"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
