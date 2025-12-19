use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]

pub enum GameState {
    AssetLoading,
    Menu,
    Playing,
    Shop,
    #[default]
    WeaponMenu,
    GameOver,
}
