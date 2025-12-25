use crate::components::player::HandType;
use bevy::prelude::*;

#[derive(Component)]
pub struct ConfirmationDialogUI;

#[derive(Component)]
pub struct HUDHandIndicator {
    #[allow(dead_code)]
    pub side: HandType,
}

#[derive(Component)]
pub struct HUDIcon {
    pub side: HandType,
}

#[derive(Component)]
pub struct CooldownOverlay {
    pub side: HandType,
}

#[derive(Component)]
pub struct ShurikenCountText {
    pub side: HandType,
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct RoundText;

#[derive(Component)]
pub struct MagicSlotIndicator {
    pub slot: crate::components::weapon::ActiveSpellSlot,
}

#[derive(Component)]
pub struct TutorialUI;

#[derive(Component)]
pub struct TutorialButton;

#[derive(Component)]
pub struct HUDRoot;

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct SettingsBackButton;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    LeftFire,
    RightFire,
    LeftSkill,
    RightSkill,
}

#[derive(Component)]
pub struct RebindButton(pub Action);

#[derive(Resource, Default)]
pub struct RebindState {
    pub active_action: Option<Action>,
}
