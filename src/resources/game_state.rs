use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PreviousMenuState(pub GameState);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Playing,
    #[default]
    WeaponMenu,
    Tutorial,
    GameOver,
    Settings,
}
