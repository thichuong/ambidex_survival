pub const RADIUS: f32 = 800.0;
pub const LIFETIME: f32 = 0.2;

// pub const PUSH_STRENGTH: f32 = 2500.0; // Keep for now if referenced elsewhere, but we might deprecate
// pub const PULL_STRENGTH: f32 = 1800.0;

pub const PUSH_DURATION: f32 = 0.5;
pub const PULL_DURATION: f32 = 0.5;

// Scaling factors for distance-based speed calculation
// Speed = Distance * Factor (or variations based on range)
pub const PUSH_SPEED_FACTOR: f32 = 1.0;
pub const PULL_SPEED_FACTOR: f32 = 1.4;

pub const DAMAGE_BASE: f32 = 5.0;
pub const DAMAGE_BONUS_MAX: f32 = 25.0;
