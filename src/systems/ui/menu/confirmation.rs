use crate::components::player::{CombatStats, Currency, Health, Player, Progression};
use crate::resources::game_state::GameState;
use crate::resources::round::RoundManager;
use crate::systems::ui::components::ConfirmationDialogUI;
use bevy::prelude::*;

pub fn spawn_confirmation_dialog(commands: &mut Commands) {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ConfirmationDialogUI,
            ZIndex(100), // Ensure it's on top
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Px(400.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                })
                .insert(BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 1.0)))
                .insert(BorderColor::all(Color::srgb(0.3, 0.3, 0.4)))
                .insert(BorderRadius::all(Val::Px(12.0)))
                .with_children(|modal| {
                    modal.spawn((
                        Text::new("Start a New Game?"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(30.0)),
                            ..default()
                        },
                    ));

                    modal
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            ..default()
                        })
                        .with_children(|buttons| {
                            // OK Button
                            buttons
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(120.0),
                                        height: Val::Px(45.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BorderColor::all(Color::srgb(0.3, 0.8, 0.3)),
                                    BorderRadius::all(Val::Px(8.0)),
                                    BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0)),
                                ))
                                .observe(|_: On<Pointer<Click>>,
                                         mut next_state: ResMut<NextState<GameState>>,
                                         player: Single<(&mut Health, &mut Currency, &mut CombatStats, &mut Progression, &mut Transform), With<Player>>,
                                         mut round_manager: ResMut<RoundManager>,
                                         enemy_query: Query<Entity, With<crate::components::enemy::Enemy>>,
                                         projectile_query: Query<Entity, With<crate::components::weapon::Projectile>>,
                                         dialog_query: Query<Entity, With<ConfirmationDialogUI>>,
                                         mut commands: Commands| {
                                    // Reset Player
                                    let (mut health, mut currency, mut combat, mut progression, mut transform) = player.into_inner();
                                    *health = Health::default();
                                    *currency = Currency::default();
                                    *combat = CombatStats::default();
                                    *progression = Progression::default();
                                    transform.translation = Vec3::ZERO;

                                    // Reset Round
                                    *round_manager = RoundManager::default();
                                    round_manager.has_started = false;

                                    // Despawn Enemies and Projectiles
                                    for entity in &enemy_query { commands.entity(entity).despawn(); }
                                    for entity in &projectile_query { commands.entity(entity).despawn(); }

                                    // Despawn Dialog
                                    for entity in &dialog_query { commands.entity(entity).despawn(); }

                                    // Return to Weapon Menu
                                    next_state.set(GameState::WeaponMenu);
                                })
                                .observe(|trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                                    if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.25, 0.45, 0.25, 1.0)); }
                                })
                                .observe(|trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                                    if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.15, 0.25, 0.15, 1.0)); }
                                })
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("OK"),
                                        TextFont { font_size: 20.0, ..default() },
                                        TextColor(Color::WHITE),
                                    ));
                                });

                            // Cancel Button
                            buttons
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(120.0),
                                        height: Val::Px(45.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BorderColor::all(Color::srgb(0.8, 0.3, 0.3)),
                                    BorderRadius::all(Val::Px(8.0)),
                                    BackgroundColor(Color::srgba(0.25, 0.15, 0.15, 1.0)),
                                ))
                                .observe(|_: On<Pointer<Click>>,
                                         dialog_query: Query<Entity, With<ConfirmationDialogUI>>,
                                         mut commands: Commands| {
                                    for entity in &dialog_query {
                                        commands.entity(entity).despawn();
                                    }
                                })
                                .observe(|trigger: On<Pointer<Over>>, mut color: Query<&mut BackgroundColor>| {
                                    if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.4, 0.2, 0.2, 1.0)); }
                                })
                                .observe(|trigger: On<Pointer<Out>>, mut color: Query<&mut BackgroundColor>| {
                                    if let Ok(mut color) = color.get_mut(trigger.entity) { *color = BackgroundColor(Color::srgba(0.25, 0.15, 0.15, 1.0)); }
                                })
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("CANCEL"),
                                        TextFont { font_size: 20.0, ..default() },
                                        TextColor(Color::WHITE),
                                    ));
                                });
                        });
                });
        });
}
