# Project Structure

This document describes the directory and file organization of the **Ambidex Survival** project.

## Directory Layout

```text
ambidex_survival/
├── assets/             # Game assets (images, fonts, UI icons)
├── src/                # Root source directory
│   ├── components/     # ECS Components used for game entities
│   │   ├── attack_effects.rs
│   │   ├── enemy.rs
│   │   ├── physics.rs
│   │   ├── player.rs
│   │   └── weapon.rs
│   ├── configs/        # Configuration constants for gameplay balance
│   │   ├── spells/     # Spell configurations (energy_bolt, laser, nova, blink, global)
│   │   ├── weapons/    # Weapon configurations (gun, shuriken, sword)
│   │   ├── enemy.rs
│   │   ├── player.rs
│   │   ├── shop.rs
│   │   └── visuals.rs
│   ├── plugins/        # Bevy Plugins for modular system registration
│   │   ├── combat.rs
│   │   ├── physics.rs
│   │   ├── player.rs
│   │   ├── ui.rs
│   │   └── visuals.rs
│   ├── resources/      # ECS Resources (RoundManager, CachedAssets, etc.)
│   │   ├── cached_assets.rs
│   │   ├── game_state.rs
│   │   ├── polish.rs
│   │   └── round.rs
│   ├── systems/        # Core game logic partitioned by domain
│   │   ├── combat/     # Weapon firing and skill logic (Modularized)
│   │   │   ├── collision/  # Collision detection and damage pipeline
│   │   │   │   ├── mod.rs
│   │   │   │   ├── damage.rs
│   │   │   │   ├── detection.rs
│   │   │   │   ├── effects.rs
│   │   │   │   └── enemy_death.rs
│   │   │   ├── magic/      # Magic sub-system with individual spell logic
│   │   │   │   ├── blink.rs
│   │   │   │   ├── energy_bolt.rs
│   │   │   │   ├── global_spell.rs
│   │   │   │   ├── laser.rs
│   │   │   │   └── nova.rs
│   │   │   ├── mod.rs          # CombatInputParams, shared types, events
│   │   │   ├── events.rs
│   │   │   ├── elite_ai.rs     # AI for Elite enemies (Teleport + Spread fire)
│   │   │   ├── gun.rs
│   │   │   ├── player_collision.rs
│   │   │   ├── shuriken.rs
│   │   │   ├── sword.rs
│   │   │   └── sword_mechanics.rs
│   │   ├── ui/         # UI layout and update systems
│   │   │   ├── components.rs
│   │   │   ├── game_over.rs
│   │   │   ├── hud.rs
│   │   │   ├── interaction.rs
│   │   │   ├── menu.rs
│   │   │   ├── setup.rs
│   │   │   └── tutorial.rs
│   │   ├── damage_text.rs
│   │   ├── enemy.rs
│   │   ├── physics.rs
│   │   ├── player.rs
│   │   └── weapon_visuals.rs
│   ├── utils/          # Generic utility functions
│   └── main.rs         # Game entry point and plugin registration
├── index.html          # Web entry point
├── Trunk.toml          # Build configuration for Trunk (Wasm)
└── README.md           # Project overview and instructions
```

## Detailed Breakdown

### `src/components/`
Components are pure data structs that attach to entities.
- `player.rs`: Player marker and stat components (Health, Currency, CombatStats).
- `weapon.rs`: Weapon-specific components (MagicLoadout, SwordState, GunState).
- `enemy.rs`: Enemy marker and stat components.
- `physics.rs`: Velocity, Collider labels, sensor markers.
- `attack_effects.rs`: Components for projectiles and damage effects.
- `faction.rs`: Unified `Faction` enum for targeting logic (Player vs Enemy).

### `src/configs/`
Configuration modules for gameplay balancing. All constants in one place.
- `shop.rs`: Shop items configuration (prices, limits, descriptions).
- `player.rs`: Player stat constants (speed, starting gold).
- `visuals.rs`: Global visual constants (damage text, colors).
- `enemy.rs`: Enemy spawning parameters and visuals.
- `weapons/`: Base stats for Sword, Gun, and Shuriken.
- `spells/`: Base stats for Energy Bolt, Laser, Nova, Blink, Global.

### `src/plugins/`
Modular plugins that encapsulate system registration.
- `combat.rs`: Registers all combat-related systems (weapons, collision, enemy AI).
- `physics.rs`: Registers physics and movement systems.
- `player.rs`: Registers player spawning and input handling.
- `ui.rs`: Registers all UI systems (HUD, menus, shop, game over).
- `visuals.rs`: Registers visual effect systems (projectile trails, damage text).

### `src/resources/`
ECS Resources for global game state.
- `cached_assets.rs`: Asset handles cache to avoid redundant loads.
- `game_state.rs`: GameState enum (Playing, Paused, GameOver, WeaponMenu, Tutorial).
- `round.rs`: Wave progression and round management.
- `polish.rs`: Screen shake and particle trail effects.

### `src/systems/`

#### `ui/`
Modularized UI systems:
- `components.rs`: UI component definitions (markers, bundles).
- `setup.rs`: Spawning the UI hierarchy.
- `hud.rs`: Real-time updates for health, gold, and cooldowns.
- `interaction.rs`: Click handling and shop purchases via `PurchaseEvent`.
- `menu.rs`: Weapon selection menu and shop UI logic.
- `game_over.rs`: Game Over screen and restart logic.
- `tutorial.rs`: Interaction guide and skill descriptions.

#### `combat/`
Modularized combat systems following Bevy 0.17 ECS best practices:
- `mod.rs`: `CombatInputParams` SystemParam bundle using `Single<T>` for Camera/Window access.
- `sword.rs` & `sword_mechanics.rs`: Advanced sword logic, swing states, and frame-accurate hit detection.
- `gun.rs`: Multi-mode firearm systems (Single, Shotgun, Rapid) with automatic fire logic.
- `shuriken.rs`: Velocity-based shuriken projectiles and teleportation skill.
- `events.rs`: Unified combat events using `Message` derive for `MessageReader` compatibility.
- `player_collision.rs`: Player-enemy overlap handling and reactive damage reception.
- **Architectural Note**: All systems use `()` return types and communicate via `MessageWriter`/`MessageReader` or reactive `On<E>` observers.

#### `combat/magic/`
Magic spell sub-system:
- `mod.rs`: Spell slot management and input dispatching.
- `energy_bolt.rs`: Projectile spell with explosion on impact.
- `laser.rs`: Instant-hit beam spell.
- `nova.rs`: Radial burst centered on player.
- `blink.rs`: Short-range teleport to cursor.
- `global_spell.rs`: Screen-wide damage.

#### `combat/collision/`
Collision detection and damage processing pipeline:
- `mod.rs`: Shared types (`ProjectileQueryItem`) and re-exports.
- `detection.rs`: Faction-aware spatial collision detection.
- `damage.rs`: Damage calculation, critical hits, lifesteal.
- `effects.rs`: Explosion effects and projectile despawning.
- `enemy_death.rs`: Enemy death particles and gold drops.

#### `systems/` (Combat Root Extensions)
- `elite_ai.rs`: State-machine based AI for elite enemies featuring teleportation, predictive targeting, and damage scaling.

### `src/systems/` (Root Level)
- `player.rs`: Player spawning and movement using `Single<Window>`.
- `enemy.rs`: Enemy AI, movement, and wave spawning logic.
- `physics.rs`: Decoupled movement and velocity integration systems.
- `weapon_visuals.rs`: Visual effects for projectiles and swing animations.
- `damage_text.rs`: Reactive floating numbers triggered by `On<DamageEvent>`.
