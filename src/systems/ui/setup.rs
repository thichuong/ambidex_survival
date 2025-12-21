use super::game_over::spawn_game_over_menu;
use super::hud::spawn_hud;
use super::menu::spawn_weapon_menu;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_hud(&mut commands, &asset_server);
    spawn_weapon_menu(&mut commands);
    spawn_game_over_menu(&mut commands);
}
