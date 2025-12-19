use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    settings::{Backends, RenderCreation, WgpuSettings},
};
use bevy::window::WindowResolution;

mod components;
mod configs;
mod resources;
mod systems;
mod utils;

use bevy::window::PrimaryWindow;
use components::player::GameCamera;
use resources::game_state::GameState;
use systems::player::{aim_player, move_player, spawn_player};
use utils::log_error;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ambidex Survival".into(),
                        resolution: WindowResolution::new(1280, 720),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::all()),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_message::<systems::combat::DamageEvent>()
        .init_state::<GameState>()
        .init_resource::<resources::round::RoundManager>()
        .init_resource::<resources::polish::ScreenShake>()
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, maximize_window)
        .add_systems(
            Update,
            (
                systems::physics::apply_velocity.pipe(log_error),
                move_player.pipe(log_error),
                aim_player.pipe(log_error),
                resources::polish::update_camera_shake,
                resources::polish::spawn_trails,
                systems::combat::handle_combat_input.pipe(log_error),
                systems::combat::manage_lifetime.pipe(log_error),
                systems::combat::resolve_damage.pipe(log_error),
                systems::combat::update_sword_mechanics.pipe(log_error),
                systems::combat::handle_player_collision.pipe(log_error),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                systems::enemy::enemy_chase_player.pipe(log_error),
                systems::enemy::spawn_waves.pipe(log_error),
                systems::damage_text::spawn_damage_text.pipe(log_error),
                systems::damage_text::update_damage_text.pipe(log_error),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                systems::ui::update_ui_visibility.pipe(log_error),
                systems::ui::update_hud_indicators.pipe(log_error),
                systems::ui::update_magic_ui.pipe(log_error),
                systems::ui::update_health_ui.pipe(log_error),
            ),
        )
        .add_systems(Startup, systems::ui::setup_ui)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}

#[allow(clippy::needless_pass_by_value)]
fn maximize_window(
    winit_windows: Option<NonSend<bevy::winit::WinitWindows>>,
    windows: Query<Entity, With<PrimaryWindow>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    if let Some(winit_windows) = winit_windows {
        for entity in &windows {
            if let Some(winit_window) = winit_windows.get_window(entity) {
                winit_window.set_maximized(true);
                *done = true;
            }
        }
    }
}
