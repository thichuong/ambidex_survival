pub const BASE_HEALTH: f32 = 30.0;
pub const HEALTH_SCALING_PER_ROUND: f32 = 10.0;
pub const BASE_SPEED: f32 = 150.0;
pub const BASE_DAMAGE: f32 = 10.0;
pub const DAMAGE_SCALING_PER_ROUND: f32 = 5.0;

pub const BASE_ENEMY_COUNT: u32 = 10;
pub const ENEMY_COUNT_SCALING_PER_ROUND: u32 = 5;
pub const BASE_SPAWN_INTERVAL: f32 = 1.0;
pub const SPAWN_INTERVAL_DECAY: f32 = 0.95;

pub const SPAWN_RADIUS_MIN: f32 = 500.0;
pub const SPAWN_RADIUS_MAX: f32 = 800.0;

pub const COLLIDER_RADIUS: f32 = 15.0;
pub const VISUAL_RADIUS: f32 = 15.0;
pub const VISUAL_Z_INDEX: f32 = 0.1;

// Elite Enemy Stats
pub const ELITE_BASE_HEALTH: f32 = 200.0;
pub const ELITE_HEALTH_SCALING_PER_ROUND: f32 = 50.0;
pub const ELITE_BASE_SPEED: f32 = 180.0;
pub const ELITE_SHURIKEN_COOLDOWN: f32 = 1.0;
pub const ELITE_TELEPORT_COOLDOWN: f32 = 2.0;
pub const ELITE_TELEPORT_CHANCE: f32 = 0.5;
pub const ELITE_SHURIKEN_SPREAD: f32 = 0.5;
pub const ELITE_VISUAL_RADIUS: f32 = 25.0;
pub const ELITE_COLLIDER_RADIUS: f32 = 25.0;
pub const ELITE_CRIT_CHANCE: f32 = 1.0;
pub const ELITE_CRIT_DAMAGE: f32 = 1.0;

// Yellow Enemy Stats (Mirror Mage)
pub const YELLOW_BASE_HEALTH: f32 = 100.0;
pub const YELLOW_HEALTH_SCALING_PER_ROUND: f32 = 40.0;
pub const YELLOW_BASE_SPEED: f32 = 200.0;
pub const YELLOW_BLINK_COOLDOWN: f32 = 3.0;
pub const YELLOW_GLOBAL_COOLDOWN: f32 = 5.0;
pub const YELLOW_BLINK_RANGE: f32 = 800.0;
pub const YELLOW_VISUAL_RADIUS: f32 = 20.0;
pub const YELLOW_COLLIDER_RADIUS: f32 = 20.0;

// Gold Rewards
pub const GOLD_REWARD: u32 = 10;
pub const ELITE_GOLD_REWARD: u32 = 100;
