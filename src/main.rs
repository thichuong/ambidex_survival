use bevy::prelude::*;
use bevy::window::WindowResolution;

mod components;
mod configs;
mod resources;
mod systems;
mod utils;

use bevy::window::PrimaryWindow;
use components::player::GameCamera; // Import GameCamera from components
use resources::game_state::GameState;
use systems::player::{aim_player, move_player, spawn_player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ambidex Survival".into(),
                resolution: WindowResolution::new(1280, 720),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_message::<systems::combat::DamageEvent>()
        //.add_plugins(RapierDebugRenderPlugin::default()) // Debug physics
        .init_state::<GameState>()
        .init_resource::<resources::round::RoundManager>()
        .init_resource::<resources::polish::ScreenShake>()
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, maximize_window)
        .add_systems(
            Update,
            (
                systems::physics::apply_velocity,
                move_player,
                aim_player,
                resources::polish::update_camera_shake,
                resources::polish::spawn_trails,
                systems::combat::handle_combat_input,
                systems::combat::manage_lifetime,
                systems::combat::resolve_damage,
                systems::combat::update_sword_mechanics,
            ),
        )
        .add_systems(
            Update,
            (
                systems::enemy::enemy_chase_player,
                systems::enemy::spawn_waves,
                systems::ui::weapon_button_interaction,
                systems::ui::update_shop_visibility,
                systems::ui::shop_button_interaction,
                systems::ui::update_magic_ui,
                systems::ui::magic_button_interaction,
                systems::damage_text::spawn_damage_text,
                systems::damage_text::update_damage_text,
            ),
        )
        // TEMPORARY
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
