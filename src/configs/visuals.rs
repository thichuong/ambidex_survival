use bevy::prelude::*;

// Damage Text Settings
pub const DAMAGE_TEXT_SIZE_NORMAL: f32 = 20.0;
pub const DAMAGE_TEXT_SIZE_CRIT: f32 = 30.0;
pub const DAMAGE_TEXT_LIFETIME: f32 = 0.8;
pub const DAMAGE_TEXT_VELOCITY: Vec2 = Vec2::new(0.0, 50.0);
pub const DAMAGE_TEXT_Z_INDEX: f32 = 10.0;

// Magic UI Settings
pub const MAGIC_PANEL_BG: Color = Color::srgba(0.06, 0.06, 0.08, 0.95);
pub const MAGIC_SLOT_BG: Color = Color::srgba(0.12, 0.12, 0.16, 1.0);
pub const MAGIC_SLOT_BG_HOVER: Color = Color::srgba(0.18, 0.18, 0.24, 1.0);
pub const MAGIC_SLOT_BORDER_DEFAULT: Color = Color::srgba(0.3, 0.3, 0.4, 0.5);
pub const MAGIC_SLOT_BORDER_SELECTED: Color = Color::srgba(1.0, 0.84, 0.0, 1.0); // Gold
pub const MAGIC_SLOT_BORDER_HIGHLIGHT: Color = Color::srgba(1.0, 1.0, 1.0, 0.5); // Highlight
pub const MAGIC_SLOT_SIZE: f32 = 80.0;
pub const MAGIC_PALETTE_BG: Color = Color::srgba(0.1, 0.1, 0.12, 0.5);
pub const MAGIC_SELECTION_BG: Color = Color::srgba(0.4, 0.2, 0.6, 1.0); // Selected Spell in Palette
pub const MAGIC_INFO_BG: Color = Color::srgba(0.08, 0.08, 0.1, 0.9);
pub const MAGIC_DIVIDER_COLOR: Color = Color::srgba(0.4, 0.4, 0.5, 0.8);
pub const MAGIC_SPELL_ICON_SIZE: f32 = 64.0;
