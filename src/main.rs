use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod resources;
mod systems;
mod utils;

use components::player::GameCamera; // Import GameCamera from components
use resources::game_state::GameState;
use systems::player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ambidex Survival".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..RapierConfiguration::new(100.0)
        })
        //.add_plugins(RapierDebugRenderPlugin::default()) // Debug physics
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (setup_camera, spawn_player, systems::enemy::spawn_test_enemy),
        )
        .add_systems(
            Update,
            (
                move_player,
                aim_player,
                systems::combat::handle_combat_input,
                systems::combat::manage_lifetime,
                systems::combat::resolve_damage,
                systems::enemy::enemy_chase_player,
                systems::ui::weapon_button_interaction,
            )
                .run_if(in_state(GameState::AssetLoading)),
        ) // TEMPORARY
        .add_systems(Startup, systems::ui::setup_ui)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}
