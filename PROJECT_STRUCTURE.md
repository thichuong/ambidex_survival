# Project Structure

This document describes the directory and file organization of the **Ambidex Survival** project.

## Directory Layout

```text
ambidex_survival/
├── .agent/             # AI Agent tooling (rules, workflows, skills)
│   ├── rules/
│   │   └── ecs-rule.md         # ECS architecture enforcement rules
│   ├── workflows/
│   │   └── game-workflow.md    # Game development workflow (8 steps)
│   └── skills/
│       ├── bevy-ecs/SKILL.md   # Bevy 0.17 ECS pattern templates
│       ├── weapon-system/SKILL.md  # Add weapon/spell guide
│       └── add-feature/SKILL.md    # Add feature guide (enemy, shop, etc.)
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
│   │   ├── game_state.rs    # GameState and PreviousMenuState
│   │   ├── input_settings.rs # Customizable key/mouse bindings
│   │   ├── mod.rs
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
│   │   │   │   ├── meteor.rs
│   │   │   │   └── nova.rs
│   │   │   ├── mod.rs          # CombatContext, CombatInputParams, shared types
│   │   │   ├── events.rs
│   │   │   ├── elite_ai.rs     # AI for Elite enemies (Teleport + Spread fire)
│   │   │   ├── yellow_ai.rs    # AI for Mirror Mage (Yellow enemy)
│   │   │   ├── gun.rs
│   │   │   ├── player_collision.rs
│   │   │   ├── shuriken.rs
│   │   │   ├── sword.rs
│   │   │   └── sword_mechanics.rs
│   │   ├── ui/         # UI layout and update systems
│   │   │   ├── components.rs
│   │   │   ├── game_over.rs
│   │   │   ├── hud.rs
│   │   │   ├── menu/
│   │   │   │   ├── arsenal.rs
│   │   │   │   ├── components.rs
│   │   │   │   ├── confirmation.rs
│   │   │   │   ├── interaction.rs
│   │   │   │   ├── layout.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── resources.rs    # Menu-specific resources (ActiveDescriptionSide)
│   │   │   │   ├── shop.rs
│   │   │   │   ├── spawn.rs
│   │   │   │   └── systems.rs      # Update systems for menu UI
│   │   │   ├── scaling.rs
│   │   │   ├── settings.rs
│   │   │   └── tutorial.rs
│   │   ├── damage_text.rs
│   │   ├── enemy.rs
│   │   ├── physics.rs
│   │   ├── player.rs
│   │   └── status.rs
│   ├── utils/          # Generic utility functions
│   ├── visuals/        # Visual effects and UI drawing
│   │   ├── mod.rs
│   │   ├── ui_icons.rs # Procedural UI icons (Shop, HUD)
│   │   └── world/      # Game world effects (Projectiles, Spells, Weapons)
│   │       ├── mod.rs
│   │       ├── melee.rs
│   │       ├── projectiles.rs
│   │       └── spells.rs
│   └── main.rs         # Game entry point and plugin registration
├── index.html          # Web entry point
├── Trunk.toml          # Build configuration for Trunk (Wasm)
└── README.md           # Project overview and instructions
```

## Detailed Breakdown

### `src/components/`
Components are pure data structs that attach to entities.
- `player.rs`: Player marker and stat components (Health, Currency, CombatStats).
- `weapon.rs`: Weapon-specific components (MagicLoadout, SwordState, GunState) and the unified `Faction` enum.
- `enemy.rs`: Enemy marker and stat components.
- `physics.rs`: Velocity, Collider labels, sensor markers, and `UniformGrid` Resource.
- `attack_effects.rs`: Components for projectiles and damage effects.

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
- `game_state.rs`: GameState enum (Playing, Paused, GameOver, WeaponMenu, Tutorial, Settings) and `PreviousMenuState` resource.
- `input_settings.rs`: Centralized resource for customizable keyboard and mouse bindings.
- `round.rs`: Wave progression and round management.
- `polish.rs`: Screen shake and particle trail effects.

### `src/systems/`

#### `ui/`
Modularized UI systems:
- `components.rs`: UI component definitions (markers, bundles).
- `setup.rs`: Spawning the UI hierarchy.
- `hud.rs`: Real-time updates for health, gold, and cooldowns.
- `menu/`: Modularized Weapon Menu logic:
    - `spawn.rs`: Main orchestrator calling sub-modules.
    - `layout.rs`: Generic structure (Sidebar, Header, Footer).
    - `shop.rs`: Shop upgrade panel logic.
    - `arsenal.rs`: Equipment and weapon detail panels.
    - `confirmation.rs`: "New Game" confirmation dialog.
    - `interaction.rs`: Reactive handling of menu clicks and purchases.
- `scaling.rs`: Dynamic global UI scaling based on window height.
- `settings.rs`: Input rebinding UI and logic.
- `game_over.rs`: Game Over screen and restart logic.
- `tutorial.rs`: Interaction guide and skill descriptions with contextual navigation.

#### `combat/`
Modularized combat systems following Bevy 0.17 ECS best practices:
- `mod.rs`: Defines `CombatContext` for unified parameter passing and `CombatInputParams`.
- `sword.rs` & `sword_mechanics.rs`: Advanced sword logic, swing states, and frame-accurate hit detection.
- `gun.rs`: Multi-mode firearm systems (Single, Shotgun, Rapid) with automatic fire logic.
- `shuriken.rs`: Velocity-based shuriken projectiles and teleportation skill.
- `events.rs`: Unified combat events using Bevy's native `Event` system and reactive `Observer` patterns.
- `player_collision.rs`: Player-enemy overlap handling and reactive damage reception.
- **Architectural Note**: Usage of `CombatContext` struct simplifies function signatures across all weapon types.

#### `combat/magic/`
Magic spell sub-system:
- `mod.rs`: Spell slot management and input dispatching using `CombatContext`.
- `energy_bolt.rs`: Projectile spell with explosion on impact.
- `laser.rs`: Instant-hit beam spell.
- `nova.rs`: Radial burst centered on player (or cursor with Nova Core).
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
- `yellow_ai.rs`: AI for tactical yellow enemies (Mirror Mages) featuring blink mobility and screen-wide global attacks.

### `src/systems/` (Root Level)
- `player.rs`: Player spawning and movement using `Single<Window>`.
- `enemy.rs`: Enemy AI, movement, and wave spawning logic.
- `physics.rs`: Decoupled movement and velocity integration systems.
- `damage_text.rs`: Reactive floating numbers triggered by `On<DamageEvent>`.
- `status.rs`: Status effect system (Rooted, Stunned, etc).

### `src/visuals/`
Centralized visuals module for rendering game effects and UI elements.
- `ui_icons.rs`: Procedural drawing of UI icons (Heal, Sword, Shield, etc.) to avoid reliance on external assets.
- `world/`: Sub-module for spawning visual meshes for weapons, projectiles, and magic spells.
    - `melee.rs`: Sword and physical weapon animations.
    - `projectiles.rs`: Gun bullets and shuriken visuals.
    - `spells.rs`: Energy bolt, laser, and nova visual effects.

### `.agent/`
AI Agent tooling for assisted development.
- `rules/ecs-rule.md`: ECS architecture rules — enforces Bevy 0.17 patterns, `CombatContext` usage, `RequiredComponents`, faction system, code quality standards (clippy pedantic+nursery), and anti-patterns to avoid.
- `workflows/game-workflow.md`: 8-step game development workflow (Plan → Implement → Check → Lint → Test → Manual Test → Format → Build) with auto-runnable commands.
- `skills/bevy-ecs/SKILL.md`: Code templates for Components, Systems, Observers, Events, GameState scheduling, spatial partitioning, and unit testing.
- `skills/weapon-system/SKILL.md`: Step-by-step guide to add new weapons (6 steps) or spells (5 steps) with checklists.
- `skills/add-feature/SKILL.md`: Guides for adding new enemy types, shop upgrades, GameStates, and visual effects.
