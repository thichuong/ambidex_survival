use super::components::{Action, RebindButton, RebindState, SettingsBackButton, SettingsUI};
use crate::resources::game_state::GameState;
use crate::resources::input_settings::{ActionInput, InputSettings};
use bevy::prelude::*;

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn spawn_settings_menu(mut commands: Commands, input_settings: Res<InputSettings>) {
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
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.07, 0.95)),
            SettingsUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Movement Section
            spawn_section_header(parent, "MOVEMENT");
            spawn_rebind_row(
                parent,
                "Up",
                Action::MoveUp,
                format!("{0:?}", input_settings.move_up),
            );
            spawn_rebind_row(
                parent,
                "Down",
                Action::MoveDown,
                format!("{0:?}", input_settings.move_down),
            );
            spawn_rebind_row(
                parent,
                "Left",
                Action::MoveLeft,
                format!("{0:?}", input_settings.move_left),
            );
            spawn_rebind_row(
                parent,
                "Right",
                Action::MoveRight,
                format!("{0:?}", input_settings.move_right),
            );

            // Combat Section
            spawn_section_header(parent, "COMBAT/SKILLS");
            spawn_rebind_row(
                parent,
                "Left Fire",
                Action::LeftFire,
                format_action(input_settings.left_fire),
            );
            spawn_rebind_row(
                parent,
                "Right Fire",
                Action::RightFire,
                format_action(input_settings.right_fire),
            );
            spawn_rebind_row(
                parent,
                "Left Skill",
                Action::LeftSkill,
                format_action(input_settings.left_skill),
            );
            spawn_rebind_row(
                parent,
                "Right Skill",
                Action::RightSkill,
                format_action(input_settings.right_skill),
            );

            // Back Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(40.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                    SettingsBackButton,
                ))
                .observe(
                    |_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::MainMenu);
                    },
                )
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("BACK"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn spawn_section_header(parent: &mut ChildSpawnerCommands, title: &str) {
    parent.spawn((
        Text::new(title),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            margin: UiRect::vertical(Val::Px(10.0)),
            ..default()
        },
    ));
}

fn spawn_rebind_row(parent: &mut ChildSpawnerCommands, label: &str, action: Action, value: String) {
    parent
        .spawn(Node {
            width: Val::Px(400.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(5.0)),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));

            row.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 1.0)),
                RebindButton(action),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(value),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn format_action(action: ActionInput) -> String {
    match action {
        ActionInput::Keyboard(k) => format!("{k:?}"),
        ActionInput::Mouse(m) => format!("{m:?}"),
    }
}

pub fn despawn_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn handle_rebind_clicks(
    mut commands: Commands,
    query: Query<(Entity, &RebindButton), Added<RebindButton>>,
) {
    for (entity, rebind) in &query {
        let action = rebind.0;
        commands.entity(entity).observe(
            move |_: On<Pointer<Click>>, mut rebind_state: ResMut<RebindState>| {
                rebind_state.active_action = Some(action);
            },
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn rebind_system(
    mut rebind_state: ResMut<RebindState>,
    mut input_settings: ResMut<InputSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Some(action) = rebind_state.active_action else {
        return;
    };

    // Find the first pressed key or mouse button
    let mut new_input = keys
        .get_just_pressed()
        .find(|&&key| key != KeyCode::Escape)
        .map(|&key| ActionInput::Keyboard(key));

    if new_input.is_none() {
        if let Some(m_btn) = mouse.get_just_pressed().next() {
            new_input = Some(ActionInput::Mouse(*m_btn));
        }
    }

    if let Some(input) = new_input {
        match action {
            Action::MoveUp => {
                if let ActionInput::Keyboard(k) = input {
                    input_settings.move_up = k;
                }
            }
            Action::MoveDown => {
                if let ActionInput::Keyboard(k) = input {
                    input_settings.move_down = k;
                }
            }
            Action::MoveLeft => {
                if let ActionInput::Keyboard(k) = input {
                    input_settings.move_left = k;
                }
            }
            Action::MoveRight => {
                if let ActionInput::Keyboard(k) = input {
                    input_settings.move_right = k;
                }
            }
            Action::LeftFire => input_settings.left_fire = input,
            Action::RightFire => input_settings.right_fire = input,
            Action::LeftSkill => input_settings.left_skill = input,
            Action::RightSkill => input_settings.right_skill = input,
        }
        rebind_state.active_action = None;
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn update_rebind_ui(
    rebind_state: Res<RebindState>,
    input_settings: Res<InputSettings>,
    mut query: Query<(&RebindButton, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (rebind, children) in &mut query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        if rebind_state.active_action == Some(rebind.0) {
            text.0 = "...".to_string();
        } else {
            text.0 = match rebind.0 {
                Action::MoveUp => format!("{0:?}", input_settings.move_up),
                Action::MoveDown => format!("{0:?}", input_settings.move_down),
                Action::MoveLeft => format!("{0:?}", input_settings.move_left),
                Action::MoveRight => format!("{0:?}", input_settings.move_right),
                Action::LeftFire => format_action(input_settings.left_fire),
                Action::RightFire => format_action(input_settings.right_fire),
                Action::LeftSkill => format_action(input_settings.left_skill),
                Action::RightSkill => format_action(input_settings.right_skill),
            };
        }
    }
}
