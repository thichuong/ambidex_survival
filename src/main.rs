use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::{
    RenderPlugin,
    settings::{Backends, RenderCreation, WgpuSettings},
};

mod components;
mod configs;
mod plugins;
mod resources;
mod systems;
mod utils;
mod visuals;

use bevy::window::PrimaryWindow;
use components::player::GameCamera;
use resources::cached_assets::CachedAssets;
use resources::game_state::GameState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ambidex Survival".into(),
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
        .add_plugins((
            plugins::combat::CombatPlugin,
            plugins::physics::PhysicsPlugin,
            plugins::ui::UiPlugin,
            plugins::player::PlayerPlugin,
            plugins::visuals::VisualsPlugin,
        ))
        .init_state::<GameState>()
        .init_resource::<resources::round::RoundManager>()
        .init_resource::<resources::polish::ScreenShake>()
        .init_resource::<components::physics::UniformGrid>()
        .add_systems(Startup, (setup_camera, init_cached_assets))
        .add_systems(Update, maximize_window)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}

fn init_cached_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cached = CachedAssets::new(&mut meshes, &mut materials);
    commands.insert_resource(cached);
}

#[allow(clippy::needless_pass_by_value)]
fn maximize_window(mut windows: Query<&mut Window, With<PrimaryWindow>>, mut frames: Local<usize>) {
    if *frames > 100 {
        return;
    }
    *frames += 1;

    for mut window in &mut windows {
        window.set_maximized(true); // Trying to see if this method works
    }
}
