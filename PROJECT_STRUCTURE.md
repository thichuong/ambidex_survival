# Project Structure

This document describes the directory and file organization of the **Ambidex Survival** project.

## Directory Layout

```text
ambidex_survival/
├── assets/             # Game assets (images, fonts, UI icons)
├── src/                # Root source directory
│   ├── components/     # ECS Components used for game entities
│   ├── configs/        # Configuration constants for gameplay balance
│   ├── resources/      # ECS Resources (RoundManager, CachedAssets, etc.)
│   ├── systems/        # Core game logic partitioned by domain
│   │   ├── combat/     # Weapon firing and skill logic (Modularized)
│   │   │   ├── magic/  # Magic sub-system with specific spell logic
│   │   │   ├── gun.rs
│   │   │   ├── shuriken.rs
│   │   │   └── sword.rs
│   │   ├── ui/         # UI layout and update systems (HUD, Shop, Menus)
│   │   └── ...         # Player, Enemy, Physics systems
│   ├── utils/          # Generic utility functions
│   └── main.rs         # Game entry point and system registration
├── index.html          # Web entry point
├── Trunk.toml          # Build configuration for Trunk (Wasm)
└── README.md           # Project overview and instructions
```

## Detailed Breakdown

### `src/components/`
- `player.rs`: Player marker and stat components (Health, Currency, CombatStats).
- `weapon.rs`: Weapon-specific components (MagicLoadout, SwordState, GunState).
- `enemy.rs`: Enemy marker components.
- `physics.rs`: Velocity, Collider, and collision labels.

### `src/configs/`
- `shop.rs`: Central configuration for shop items (prices, limits, descriptions).
- `player.rs`: Player stat constants (speed, starting gold) and visuals.
- `visuals.rs`: Global visual constants (damage text, etc.).
- `enemy.rs`: Enemy spawning parameters (radius, scaling) and visuals.
- `weapons/`: Base stats and physics config for Sword, Gun, and Shuriken.
- `spells/`: Base stats for Energy Bolt, Laser, Nova, etc.

### `src/systems/`

#### `ui/`
Modularized UI systems for better organization:
- `setup.rs`: Spawning the entire UI hierarchy.
- `hud.rs`: Real-time updates for health, gold, and cooldowns.
- `interaction.rs`: Handling clicks and shop purchases via `PurchaseEvent`.
- `menu.rs`: Weapon selection menu and shop UI logic.
- `game_over.rs`: Game Over screen and restart logic.

#### `combat/`
Modularized combat systems following ECS best practices. Input handling and item instantiation are split for each weapon category:
- `shuriken.rs`: Handles Shuriken projectiles and teleport mobility.
- `sword.rs`: Handles Sword swings and mode switching.
- `gun.rs`: Handles Gun firing modes and automatic fire for Rapid mode.
- `magic/`: Sub-module for magic casting and individual spell implementations.
  - `mod.rs`: Orchestration of magic slots and input dispatching.
  - `energy_bolt.rs`, `laser.rs`, `nova.rs`, `blink.rs`, `global_spell.rs`: Individual spell behaviors.
- `collision/`: Sub-module for collision detection and damage processing.
  - `mod.rs`: Shared types (`ProjectileQueryItem`) and re-exports.
  - `detection.rs`: Collision detection with spatial grid.
  - `damage.rs`: Damage processing, crit calculation, lifesteal.
  - `effects.rs`: Explosion effects and projectile despawning.
  - `enemy_death.rs`: Enemy death particles and gold drops.
- `weapon_visuals.rs`: Spawning and updating visual effects for projectiles, sword swings, and spells.
- `enemy.rs`: Enemy AI, movement, and spawning logic.

### `src/resources/`
- `cached_assets.rs`: Handles asset loading and caching to avoid redundant `asset_server.load` calls.
- `round.rs`: Manages wave progression and round states.
- `polish.rs`: Screen shake and particle trail effects.
