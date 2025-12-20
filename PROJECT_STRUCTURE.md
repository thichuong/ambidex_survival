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
│   │   ├── combat/     # Weapon firing and skill logic
│   │   ├── ui/         # UI layout and update systems
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
- `weapons/`: Base stats for Sword, Gun, and Shuriken.
- `spells/`: Base stats for Energy Bolt, Laser, Nova, etc.

### `src/systems/`
- **`ui/`**: Modularized UI systems.
  - `setup.rs`: Spawning the entire UI hierarchy.
  - `hud.rs`: Real-time updates for health, gold, and cooldowns.
  - `interaction.rs`: Handling clicks and shop purchases via `PurchaseEvent`.
- **`combat/`**: Handles user input and weapon instantiation.
  - `weapon_logic.rs`: The main input handler for firing weapons and spells.
- `weapon_visuals.rs`: Spawning and updating visual effects for projectiles, sword swings, and spells.
- `enemy.rs`: Enemy AI, movement, and spawning logic.

### `src/resources/`
- `cached_assets.rs`: Handles asset loading and caching to avoid redundant `asset_server.load` calls.
- `round.rs`: Manages wave progression and round states.
